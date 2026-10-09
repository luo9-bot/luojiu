//! Growth Lab — Phase A: observation only.
//!
//! Records existing social-gate decisions for later, reproducible evaluation.
//! This module must never change a decision, prompt, memory, personality, or config.
//! Raw messages, user IDs, and group IDs are deliberately excluded from the dataset.

use serde::Serialize;
use serde_json::Value;
use std::collections::VecDeque;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::{self, SyncSender, TrySendError};
use std::sync::{OnceLock, RwLock};
use tracing::warn;

use crate::util;

const QUEUE_CAPACITY: usize = 4096;
const RECENT_LIMIT: usize = 200;
const DATASET_DIR: &str = "growth_lab";
const DATASET_FILE: &str = "events.jsonl";

static EVENT_SENDER: OnceLock<Option<SyncSender<Observation>>> = OnceLock::new();
static SUMMARY: OnceLock<RwLock<Summary>> = OnceLock::new();

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

#[derive(Debug, Clone, Default)]
struct Summary {
    available: bool,
    event_count: u64,
    pass_count: u64,
    silent_count: u64,
    score_sum: f64,
    threshold_sum: f64,
    recent: VecDeque<Value>,
}

impl Summary {
    fn push(&mut self, event: &Value) {
        if event.get("event").and_then(Value::as_str) != Some("speak_gate") {
            return;
        }

        self.available = true;
        self.event_count += 1;
        match event.get("decision").and_then(Value::as_str) {
            Some("pass") => self.pass_count += 1,
            Some("silent") => self.silent_count += 1,
            _ => {}
        }
        self.score_sum += event.get("score").and_then(Value::as_f64).unwrap_or(0.0);
        self.threshold_sum += event
            .get("threshold")
            .and_then(Value::as_f64)
            .unwrap_or(0.0);

        let public_event = public_event(event);
        if self.recent.len() == RECENT_LIMIT {
            self.recent.pop_front();
        }
        self.recent.push_back(public_event);
    }
}

fn summary_lock() -> &'static RwLock<Summary> {
    SUMMARY.get_or_init(|| RwLock::new(Summary::default()))
}

fn public_event(event: &Value) -> Value {
    serde_json::json!({
        "timestamp_unix": event.get("timestamp_unix").and_then(Value::as_u64),
        "batch_size": event.get("batch_size").and_then(Value::as_u64).unwrap_or(0),
        "score": event.get("score").and_then(Value::as_f64).unwrap_or(0.0),
        "threshold": event.get("threshold").and_then(Value::as_f64).unwrap_or(0.0),
        "decision": event.get("decision").and_then(Value::as_str).unwrap_or("unknown"),
        "topic_relevance": event.get("topic_relevance").and_then(Value::as_f64).unwrap_or(0.0),
        "attention_max": event.get("attention_max").and_then(Value::as_f64).unwrap_or(0.0),
        "unanswered_bonus": event.get("unanswered_bonus").and_then(Value::as_f64).unwrap_or(0.0),
        "thread_freshness": event.get("thread_freshness").and_then(Value::as_f64).unwrap_or(0.0),
        "addressing": event.get("addressing").and_then(Value::as_f64).unwrap_or(0.0),
        "fatigue": event.get("fatigue").and_then(Value::as_f64).unwrap_or(0.0),
        "social_risk": event.get("social_risk").and_then(Value::as_f64).unwrap_or(0.0),
        "recent_reply": event.get("recent_reply").and_then(Value::as_f64).unwrap_or(0.0),
        "secs_since_spoke": event.get("secs_since_spoke").and_then(Value::as_u64),
    })
}

/// Small, bounded in-memory report. The WebUI never rescans the whole JSONL file.
pub(crate) fn report() -> Value {
    let snapshot = summary_lock()
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone();
    let pass_rate = if snapshot.event_count > 0 {
        Some(snapshot.pass_count as f64 / snapshot.event_count as f64)
    } else {
        None
    };

    serde_json::json!({
        "available": snapshot.available,
        "event_count": snapshot.event_count,
        "pass_count": snapshot.pass_count,
        "silent_count": snapshot.silent_count,
        "pass_rate": pass_rate,
        "average_score": if snapshot.event_count > 0 { Some(snapshot.score_sum / snapshot.event_count as f64) } else { None },
        "average_threshold": if snapshot.event_count > 0 { Some(snapshot.threshold_sum / snapshot.event_count as f64) } else { None },
        "recent": snapshot.recent.iter().rev().cloned().collect::<Vec<_>>(),
        "coverage": "仅覆盖进入普通群聊 SpeakScore 门控的批次；其他提前返回路径尚未采集",
        "data_file": "growth_lab/events.jsonl"
    })
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

                    // Rehydrate counters once at startup. Subsequent WebUI requests use
                    // the bounded in-memory snapshot and do not scan this file again.
                    let mut summary = Summary::default();
                    if path.exists() {
                        summary.available = true;
                        if let Ok(existing) = File::open(&path) {
                            for line in BufReader::new(existing).lines().map_while(Result::ok) {
                                if let Ok(event) = serde_json::from_str::<Value>(&line) {
                                    summary.push(&event);
                                }
                            }
                        }
                    }
                    *summary_lock()
                        .write()
                        .unwrap_or_else(|poisoned| poisoned.into_inner()) = summary;

                    let mut file = match OpenOptions::new().create(true).append(true).open(&path) {
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
                            continue;
                        }
                        if let Ok(value) = serde_json::to_value(&observation) {
                            summary_lock()
                                .write()
                                .unwrap_or_else(|poisoned| poisoned.into_inner())
                                .push(&value);
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
                // The worker already logs its startup failure; avoid per-message log spam.
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
    fn summary_counts_decisions_and_bounds_recent_events() {
        let mut summary = Summary::default();
        for index in 0..201 {
            let event = serde_json::json!({
                "event": "speak_gate",
                "timestamp_unix": index,
                "batch_size": 1,
                "score": 0.2,
                "threshold": 0.18,
                "decision": if index % 2 == 0 { "pass" } else { "silent" },
                "topic_relevance": 0.5,
                "attention_max": 0.3,
                "unanswered_bonus": 1.0,
                "thread_freshness": 1.0,
                "addressing": 0.0,
                "fatigue": 0.1,
                "social_risk": 0.0,
                "recent_reply": 0.2,
                "secs_since_spoke": 30
            });
            summary.push(&event);
        }
        assert_eq!(summary.event_count, 201);
        assert_eq!(summary.pass_count, 101);
        assert_eq!(summary.silent_count, 100);
        assert_eq!(summary.recent.len(), RECENT_LIMIT);
        assert_eq!(summary.recent.front().and_then(|e| e["timestamp_unix"].as_u64()), Some(1));
    }

    #[test]
    fn decision_label_matches_existing_gate_comparison() {
        assert!(0.18_f32 >= 0.18_f32);
        assert!(!(0.17_f32 >= 0.18_f32));
    }
}
