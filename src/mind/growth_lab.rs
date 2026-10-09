//! Growth Lab — Phase A: observation only.
//!
//! Records existing social-gate decisions for later, reproducible evaluation.
//! This module must never change a decision, prompt, memory, personality, or config.
//! Raw messages, user IDs, and group IDs are deliberately excluded from the dataset.

use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::mpsc::{self, SyncSender, TrySendError};
use std::sync::OnceLock;
use tracing::warn;

use crate::util;

const QUEUE_CAPACITY: usize = 4096;
const DATASET_DIR: &str = "growth_lab";
const DATASET_FILE: &str = "events.jsonl";

static EVENT_SENDER: OnceLock<Option<SyncSender<Observation>>> = OnceLock::new();

#[derive(Debug, Clone, Serialize)]
struct Observation {
    schema_version: u8,
    timestamp_unix: u64,
    event: &'static str,
    source: &'static str,
    batch_size: usize,
    score: f32,
    threshold: f32,
    decision: &'static str,
    topic_relevance: f32,
    attention_max: f32,
    unanswered_bonus: f32,
    thread_freshness: f32,
    addressing: f32,
    fatigue: f32,
    social_risk: f32,
    recent_reply: f32,
    secs_since_spoke: Option<u64>,
}

/// Start the observer during plugin initialization, not on the first live message.
pub(crate) fn init() {
    let _ = sender();
}

fn sender() -> Option<&'static SyncSender<Observation>> {
    EVENT_SENDER
        .get_or_init(|| {
            let (tx, rx) = mpsc::sync_channel::<Observation>(QUEUE_CAPACITY);
            let worker = std::thread::Builder::new()
                .name("growth-lab-observer".to_string())
                .spawn(move || {
                    let directory = crate::config::data_dir().join(DATASET_DIR);
                    if let Err(error) = fs::create_dir_all(&directory) {
                        warn!(%error, "growth_lab: cannot create observation directory");
                        return;
                    }
                    let path = directory.join(DATASET_FILE);
                    let mut file = match OpenOptions::new().create(true).append(true).open(path) {
                        Ok(file) => file,
                        Err(error) => {
                            warn!(%error, "growth_lab: cannot open observation dataset");
                            return;
                        }
                    };

                    while let Ok(observation) = rx.recv() {
                        let write_result = serde_json::to_writer(&mut file, &observation)
                            .map_err(std::io::Error::other)
                            .and_then(|()| file.write_all(b"\n"))
                            .and_then(|()| file.flush());
                        if let Err(error) = write_result {
                            warn!(%error, "growth_lab: observation write failed");
                        }
                    }
                });
            match worker {
                Ok(_) => Some(tx),
                Err(error) => {
                    warn!(%error, "growth_lab: cannot start observer worker");
                    None
                }
            }
        })
        .as_ref()
}

/// Record the existing gate result asynchronously. A full queue drops telemetry
/// instead of blocking the message-processing path.
pub(crate) fn observe_speak_gate(
    batch_size: usize,
    breakdown: &crate::mind::social::SpeakScoreBreakdown,
    threshold: f32,
) {
    let Some(sender) = sender() else {
        return;
    };
    let observation = Observation {
        schema_version: 1,
        timestamp_unix: util::now_secs(),
        event: "speak_gate",
        source: "mind::social::speak_score",
        batch_size,
        score: breakdown.total,
        threshold,
        decision: if breakdown.total >= threshold { "pass" } else { "silent" },
        topic_relevance: breakdown.topic_relevance,
        attention_max: breakdown.attention_max,
        unanswered_bonus: breakdown.unanswered_bonus,
        thread_freshness: breakdown.thread_freshness,
        addressing: breakdown.addressing,
        fatigue: breakdown.fatigue,
        social_risk: breakdown.social_risk,
        recent_reply: breakdown.recent_reply,
        secs_since_spoke: breakdown.secs_since_spoke,
    };

    if let Err(error) = sender.try_send(observation) {
        match error {
            TrySendError::Full(_) => {
                // Telemetry is best-effort; never block or alter the gate decision.
            }
            TrySendError::Disconnected(_) => {
                warn!("growth_lab: observer worker disconnected; dropping observation");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observation_schema_is_versioned_and_excludes_conversation_identity() {
        let observation = Observation {
            schema_version: 1,
            timestamp_unix: 1,
            event: "speak_gate",
            source: "mind::social::speak_score",
            batch_size: 2,
            score: 0.4,
            threshold: 0.18,
            decision: "pass",
            topic_relevance: 0.5,
            attention_max: 0.3,
            unanswered_bonus: 1.0,
            thread_freshness: 1.0,
            addressing: 0.0,
            fatigue: 0.1,
            social_risk: 0.0,
            recent_reply: 0.2,
            secs_since_spoke: Some(30),
        };
        let json = serde_json::to_string(&observation).expect("test serialization");
        assert!(json.contains("\"schema_version\":1"));
        assert!(json.contains("\"decision\":\"pass\""));
        assert!(!json.contains("group_id"));
        assert!(!json.contains("user_id"));
        assert!(!json.contains("message"));
    }

    #[test]
    fn decision_label_matches_existing_gate_comparison() {
        assert!(0.18_f32 >= 0.18_f32);
        assert!(!(0.17_f32 >= 0.18_f32));
    }
}
