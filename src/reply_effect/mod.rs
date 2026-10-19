//! 回复效果追踪系统

mod scoring;
mod store;

use std::sync::{Mutex, OnceLock};

use scoring::{calculate_asi, should_finalize};
pub(crate) use store::*;

use tracing::{debug, info};

static STORE_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn store_lock() -> &'static Mutex<()> {
    STORE_LOCK.get_or_init(|| Mutex::new(()))
}

pub(crate) fn record_reply(
    group_id: u64,
    target_user: u64,
    reply_text: &str,
    archive_reply_id: Option<i64>,
) {
    let _guard = store_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut s = load_store();
    let now = crate::util::now_secs();
    s.records.retain(|r| {
        r.status == EffectStatus::Pending && now.saturating_sub(r.sent_at) < OBSERVATION_WINDOW * 2
    });
    if s.records.len() >= MAX_ACTIVE_RECORDS {
        s.records.remove(0);
    }
    s.records.push(ReplyEffectRecord {
        reply_text: reply_text.to_string(),
        target_user,
        group_id,
        sent_at: now,
        followups: Vec::new(),
        asi_score: None,
        status: EffectStatus::Pending,
        archive_reply_id,
    });
    save_store(&s);
    debug!(group_id, target_user, "reply_effect: recorded");
}

pub(crate) fn observe_message(group_id: u64, user_id: u64, message: &str) {
    let _guard = store_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut s = load_store();
    let now = crate::util::now_secs();
    let mut changed = false;
    let mut judge_candidate = None;
    for rec in s.records.iter_mut() {
        if rec.group_id == group_id
            && rec.target_user == user_id
            && rec.status == EffectStatus::Pending
            && now.saturating_sub(rec.sent_at) < OBSERVATION_WINDOW
            && rec.followups.len() < MAX_FOLLOWUPS
        {
            rec.followups.push(FollowupMessage {
                user_id,
                content: message.to_string(),
                timestamp: now,
            });
            changed = true;
            if should_finalize(rec) {
                rec.status = EffectStatus::Finalized;
                let rule_score = calculate_asi(rec);

                // 先用规则分完成本轮，避免在消息入口同步调用另一个分析模型。
                // LLM Judge 只作为后台增强，不得阻塞或污染正在进行的对话。
                rec.asi_score = Some(rule_score);
                // Reward 写回训练留档：ASI 0~100 → reward 0~1，
                // "获得互动的回复"在强化重训中概率上升（L6 闭环）
                if let Some(id) = rec.archive_reply_id {
                    let reward = (rule_score / 100.0) as f32;
                    crate::mind::archive::set_reward(group_id, id, reward);
                }
                info!(
                    group_id,
                    target_user = user_id,
                    asi_score = rule_score,
                    followups = rec.followups.len(),
                    "reply_effect: finalized"
                );
                if rule_score < 50.0 {
                    judge_candidate = Some(rec.clone());
                }
            }
        }
    }
    if changed {
        save_store(&s);
    }
    drop(_guard);
    if let Some(record) = judge_candidate {
        std::thread::spawn(move || {
            let Some(scores) = scoring::judge_with_llm(&record) else {
                return;
            };
            let relational = scoring::calculate_relational_from_llm(&scores);
            let friction = scoring::calculate_friction_from_llm(&record, scores.uncanny_risk);
            let behavior = scoring::calculate_behavior_score(&record);
            let llm_score =
                ((0.45 * behavior + 0.35 * relational + 0.20 * (1.0 - friction)) * 100.0).round();

            let _guard = store_lock()
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let mut store = load_store();
            if let Some(saved) = store.records.iter_mut().find(|saved| {
                saved.group_id == record.group_id
                    && saved.target_user == record.target_user
                    && saved.sent_at == record.sent_at
                    && saved.reply_text == record.reply_text
                    && saved.status == EffectStatus::Finalized
            }) {
                saved.asi_score = Some(llm_score);
                if let Some(id) = saved.archive_reply_id {
                    crate::mind::archive::set_reward(
                        saved.group_id,
                        id,
                        (llm_score / 100.0) as f32,
                    );
                }
                save_store(&store);
                debug!(
                    group_id = record.group_id,
                    target_user = record.target_user,
                    rule_score = record.asi_score.unwrap_or_default(),
                    llm_score,
                    "reply_effect: background judge adjusted"
                );
            }
        });
    }
}
