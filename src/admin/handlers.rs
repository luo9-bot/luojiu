use tiny_http::{Method, Response};
use tracing::warn;

use crate::config;

use super::backup;
use super::{err, ok, parse_json};

// ── 表情包管理 ────────────────────────────────────────────────

pub(crate) fn handle_sticker() -> Response<std::io::Cursor<Vec<u8>>> {
    let (total, registered) = crate::sticker::get_stats();
    let store = crate::sticker::store::load_store();
    ok(serde_json::json!({
        "total": total,
        "registered": registered,
        "stickers": store.stickers.iter().map(|e| serde_json::json!({
            "hash": e.hash,
            "description": e.description,
            "vlm_description": e.vlm_description,
            "emotions": e.emotions,
            "query_count": e.query_count,
            "is_registered": e.is_registered,
            "is_banned": e.is_banned,
            "is_builtin": e.is_builtin,
            "path": e.path,
            "registered_at": e.registered_at,
            "last_used_at": e.last_used_at,
        })).collect::<Vec<_>>(),
    }))
}

/// 切换表情包封禁状态
pub(crate) fn handle_sticker_toggle(hash: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut store = crate::sticker::store::load_store();
    let banned = store
        .stickers
        .iter_mut()
        .find(|e| e.hash == hash)
        .map(|entry| {
            entry.is_banned = !entry.is_banned;
            entry.is_banned
        });
    if let Some(is_banned) = banned {
        crate::sticker::store::save_store(&store);
        return ok(serde_json::json!({"ok": true, "is_banned": is_banned}));
    }
    err(404, "sticker not found")
}

/// 服务表情包图片文件
///
/// 1. 优先从注册表中查找哈希对应的路径
/// 2. 注册表未命中时，直接扫描 sticker/ 和 ne_sticker/ 目录查找文件
pub(crate) fn handle_sticker_image(hash: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let data_dir = crate::config::data_dir();
    let mut full_path = None;

    // 1. 从注册表查找
    let store = crate::sticker::store::load_store();
    if let Some(entry) = store.stickers.iter().find(|e| e.hash == hash) {
        let candidate = data_dir.join(&entry.path);
        if candidate.exists() {
            full_path = Some(candidate);
        }
    }

    // 2. 注册表未命中，直接扫描目录
    if full_path.is_none() {
        for dir in &["sticker", "ne_sticker"] {
            let dir_path = data_dir.join(dir);
            if !dir_path.exists() {
                continue;
            }
            if let Ok(entries) = std::fs::read_dir(&dir_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file()
                        && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
                        && (stem == hash || stem.starts_with(hash))
                    {
                        full_path = Some(path);
                        break;
                    }
                }
            }
            if full_path.is_some() {
                break;
            }
        }
    }

    if let Some(ref fp) = full_path {
        let data = match std::fs::read(fp) {
            Ok(d) => d,
            Err(_) => return err(500, "failed to read file"),
        };
        let ext = fp
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("png")
            .to_lowercase();
        let mime = match ext.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            _ => "image/png",
        };
        return super::attach_headers(
            Response::from_data(data),
            &[
                ("Content-Type", mime),
                ("Cache-Control", "public, max-age=86400"),
            ],
        );
    }

    err(404, "image not found")
}

// ── 仪表盘统计 ────────────────────────────────────────────────

pub(crate) fn handle_dashboard() -> Response<std::io::Cursor<Vec<u8>>> {
    let user_ids = crate::memory::store::all_user_ids();
    let user_count = user_ids.len();
    let mut mem_count: usize = 0;
    for uid in &user_ids {
        mem_count += crate::memory::store::load_user_memory(*uid).entries.len();
    }

    let (_, sticker_registered) = crate::sticker::get_stats();
    let emotion_count = crate::emotion::user_count();

    ok(serde_json::json!({
        "memory_users": user_count,
        "memory_entries": mem_count,
        "sticker_count": sticker_registered,
        "emotion_users": emotion_count,
        "active_groups": crate::get_active_groups().len(),
        "active_users": crate::get_active_users().len(),
    }))
}

// ── Growth Lab: Phase A observation dashboard ───────────────────

/// Read the bounded tail of the observation log and return aggregate counters.
/// No conversation text or identity fields are present in this dataset.
pub(crate) fn handle_growth_lab(method: &Method) -> Response<std::io::Cursor<Vec<u8>>> {
    if *method != Method::Get {
        return err(405, "method not allowed");
    }

    use std::collections::VecDeque;
    use std::io::{BufRead, BufReader};

    const RECENT_LIMIT: usize = 200;
    let path = config::data_dir().join("growth_lab").join("events.jsonl");
    let file = match std::fs::File::open(&path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return ok(serde_json::json!({
                "available": false,
                "event_count": 0,
                "pass_count": 0,
                "silent_count": 0,
                "pass_rate": null,
                "average_score": null,
                "average_threshold": null,
                "recent": [],
                "coverage": "仅覆盖进入普通群聊 SpeakScore 门控的批次；其他提前返回路径尚未采集"
            }));
        }
        Err(error) => return err(500, &format!("读取 Growth Lab 数据失败: {error}")),
    };

    let mut recent = VecDeque::with_capacity(RECENT_LIMIT);
    let mut event_count: u64 = 0;
    let mut pass_count: u64 = 0;
    let mut silent_count: u64 = 0;
    let mut score_sum = 0.0_f64;
    let mut threshold_sum = 0.0_f64;

    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return err(500, &format!("读取 Growth Lab 数据行失败: {error}")),
        };
        let Ok(event) = serde_json::from_str::<serde_json::Value>(&line) else {
            continue;
        };
        if event.get("event").and_then(|v| v.as_str()) != Some("speak_gate") {
            continue;
        }
        event_count += 1;
        match event.get("decision").and_then(|v| v.as_str()) {
            Some("pass") => pass_count += 1,
            Some("silent") => silent_count += 1,
            _ => {}
        }
        score_sum += event.get("score").and_then(|v| v.as_f64()).unwrap_or(0.0);
        threshold_sum += event.get("threshold").and_then(|v| v.as_f64()).unwrap_or(0.0);

        // Whitelist fields returned to the UI instead of exposing arbitrary JSON.
        let item = serde_json::json!({
            "timestamp_unix": event.get("timestamp_unix").and_then(|v| v.as_u64()),
            "batch_size": event.get("batch_size").and_then(|v| v.as_u64()).unwrap_or(0),
            "score": event.get("score").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "threshold": event.get("threshold").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "decision": event.get("decision").and_then(|v| v.as_str()).unwrap_or("unknown"),
            "topic_relevance": event.get("topic_relevance").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "attention_max": event.get("attention_max").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "unanswered_bonus": event.get("unanswered_bonus").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "thread_freshness": event.get("thread_freshness").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "addressing": event.get("addressing").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "fatigue": event.get("fatigue").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "social_risk": event.get("social_risk").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "recent_reply": event.get("recent_reply").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "secs_since_spoke": event.get("secs_since_spoke").and_then(|v| v.as_u64()),
        });
        if recent.len() == RECENT_LIMIT {
            recent.pop_front();
        }
        recent.push_back(item);
    }

    let pass_rate = if event_count > 0 {
        Some(pass_count as f64 / event_count as f64)
    } else {
        None
    };
    ok(serde_json::json!({
        "available": true,
        "event_count": event_count,
        "pass_count": pass_count,
        "silent_count": silent_count,
        "pass_rate": pass_rate,
        "average_score": if event_count > 0 { Some(score_sum / event_count as f64) } else { None },
        "average_threshold": if event_count > 0 { Some(threshold_sum / event_count as f64) } else { None },
        "recent": recent.into_iter().rev().collect::<Vec<_>>(),
        "coverage": "仅覆盖进入普通群聊 SpeakScore 门控的批次；其他提前返回路径尚未采集",
        "data_file": "growth_lab/events.jsonl"
    }))
}

// ── Handler: 用户记忆（读写 memory/users/{uid}.json 存储）──────

/// 组装全部用户记忆（前端数据形状：{users: {uid: {entries: [...]}}}）
fn memory_store_json() -> serde_json::Value {
    let mut users = serde_json::Map::new();
    for uid in crate::memory::store::all_user_ids() {
        let mem = crate::memory::store::load_user_memory(uid);
        users.insert(
            uid.to_string(),
            serde_json::to_value(&mem).unwrap_or_default(),
        );
    }
    serde_json::json!({ "users": users })
}

fn parse_importance(
    value: Option<&serde_json::Value>,
) -> Result<crate::memory::store::Importance, ()> {
    match value {
        Some(v) if !v.is_null() => serde_json::from_value(v.clone()).map_err(|_| ()),
        _ => Ok(crate::memory::store::Importance::Normal),
    }
}

pub(crate) fn handle_memory(
    method: &Method,
    segs: &[&str],
    body: &[u8],
) -> Response<std::io::Cursor<Vec<u8>>> {
    use crate::memory::store::{self, MemoryEntry};

    // GET /api/memory/export -> 导出全部
    if *method == Method::Get && segs.first() == Some(&"export") {
        let data = serde_json::to_string(&memory_store_json()).unwrap_or_default();
        return super::attach_headers(
            Response::from_string(data),
            &[
                ("Content-Type", "application/json; charset=utf-8"),
                (
                    "Content-Disposition",
                    "attachment; filename=\"memory_export.json\"",
                ),
            ],
        );
    }

    match method {
        Method::Get => {
            // /api/memory -> 完整 store
            // /api/memory/{user_id} -> 单用户
            match segs.first().and_then(|s| s.parse::<u64>().ok()) {
                Some(uid) => {
                    let mem = store::load_user_memory(uid);
                    ok(serde_json::json!({
                        "user_id": uid.to_string(),
                        "entries": serde_json::to_value(&mem).unwrap_or_default()
                    }))
                }
                None => ok(memory_store_json()),
            }
        }
        Method::Post => {
            let Some(uid) = segs.first().and_then(|s| s.parse::<u64>().ok()) else {
                return err(400, "invalid user_id");
            };
            let body_val: serde_json::Value = match parse_json(body) {
                Ok(v) => v,
                Err(e) => return err(400, &e),
            };
            let Some(content) = body_val
                .get("content")
                .and_then(|v| v.as_str())
                .filter(|c| !c.is_empty())
            else {
                return err(400, "content required");
            };
            let importance = match parse_importance(body_val.get("importance")) {
                Ok(i) => i,
                Err(_) => return err(400, "invalid importance"),
            };
            let now = crate::util::now_secs();
            let mut mem = store::load_user_memory(uid);
            mem.entries.push(MemoryEntry {
                content: content.to_string(),
                importance,
                created: now,
                last_accessed: now,
                access_count: 1,
                emotional_impact: None,
            });
            store::save_user_memory(uid, &mem);
            ok(serde_json::json!({"ok": true}))
        }
        Method::Delete => {
            let Some(uid) = segs.first().and_then(|s| s.parse::<u64>().ok()) else {
                return err(400, "invalid user_id");
            };
            let Some(idx) = segs.get(1).and_then(|s| s.parse::<usize>().ok()) else {
                return err(400, "index required");
            };
            let mut mem = store::load_user_memory(uid);
            if idx >= mem.entries.len() {
                return err(404, "index out of range");
            }
            mem.entries.remove(idx);
            store::save_user_memory(uid, &mem);
            ok(serde_json::json!({"ok": true}))
        }
        _ => err(405, "method not allowed"),
    }
}

// ── Handler: 工作记忆 ──────────────────────────────────────────

pub(crate) fn handle_working_memory(
    method: &Method,
    segs: &[&str],
    _body: &[u8],
) -> Response<std::io::Cursor<Vec<u8>>> {
    match method {
        Method::Get => {
            // 工作记忆已迁入状态库：读旧文件只会拿到过期内容
            let db = crate::db::db();
            if let Some(gid) = segs.first() {
                let group_id: u64 = match gid.parse() {
                    Ok(id) => id,
                    Err(_) => return err(400, "invalid group_id"),
                };
                let entries = match db.working_memory_of_group(group_id) {
                    Ok(rows) => rows
                        .into_iter()
                        .map(|row| {
                            serde_json::json!({
                                "id": row.id,
                                "user_id": row.user_id,
                                "content": row.content,
                                "timestamp": row.created_at,
                                "bot_replied": row.bot_replied,
                            })
                        })
                        .collect::<Vec<_>>(),
                    Err(error) => return err(500, &format!("读取工作记忆失败: {error}")),
                };
                ok(serde_json::json!({"group_id": gid, "entries": entries}))
            } else {
                // 保持前端既有形状：{ "groups": { "<gid>": { "entries": [...] } } }
                let groups = match db.working_memory_groups() {
                    Ok(groups) => groups,
                    Err(error) => return err(500, &format!("读取工作记忆失败: {error}")),
                };
                let mut view = serde_json::Map::new();
                for (group_id, rows) in groups {
                    let entries: Vec<serde_json::Value> = rows
                        .into_iter()
                        .map(|row| {
                            serde_json::json!({
                                "id": row.id,
                                "user_id": row.user_id,
                                "content": row.content,
                                "timestamp": row.created_at,
                                "bot_replied": row.bot_replied,
                            })
                        })
                        .collect();
                    view.insert(
                        group_id.to_string(),
                        serde_json::json!({ "entries": entries }),
                    );
                }
                ok(serde_json::json!({ "groups": view }))
            }
        }
        _ => err(405, "method not allowed"),
    }
}

// ── Handler: 情绪 ──────────────────────────────────────────────

pub(crate) fn handle_emotion(
    method: &Method,
    segs: &[&str],
) -> Response<std::io::Cursor<Vec<u8>>> {
    match method {
        Method::Get => {
            // 控制总览的 3D 核心驱动数据：四维情绪向量聚合
            if segs.first() == Some(&"core") {
                return match emotion_core_json() {
                    Ok(body) => ok(body),
                    Err(error) => err(500, &format!("聚合情绪核心失败: {error}")),
                };
            }
            // 情绪状态已迁入状态库（按用户一行）：不能再读 emotion.json，
            // 那个文件已经退休，读它只会拿到过期内容
            let mut view = serde_json::Map::new();
            match crate::db::db().all_emotion_states() {
                Ok(states) => {
                    for (uid, json) in states {
                        match serde_json::from_str::<serde_json::Value>(&json) {
                            Ok(state) => {
                                view.insert(uid.to_string(), state);
                            }
                            Err(error) => {
                                warn!(%error, uid, "admin: 情绪状态解析失败，已跳过");
                            }
                        }
                    }
                }
                Err(error) => {
                    return err(500, &format!("读取情绪状态失败: {error}"));
                }
            }
            let store = serde_json::Value::Object(view);
            if let Some(uid) = segs.first() {
                match store.get(*uid) {
                    Some(state) => ok(serde_json::json!({"user_id": uid, "state": state})),
                    None => ok(serde_json::json!({"user_id": uid, "state": null})),
                }
            } else {
                ok(store)
            }
        }
        _ => err(405, "method not allowed"),
    }
}

// ── Handler: 情绪核心（控制总览的四维向量） ────────────────────

/// 情绪类型 → (愉悦, 好奇, 共情, 压力) 贡献权重
fn emotion_vector(e: &crate::emotion::EmotionType) -> [f32; 4] {
    use crate::emotion::EmotionType::*;
    match e {
        Happy => [1.0, 0.1, 0.4, 0.0],
        Excited => [0.9, 0.6, 0.1, 0.15],
        Like => [0.8, 0.2, 0.8, 0.0],
        Shy => [0.4, 0.1, 0.7, 0.15],
        Surprised => [0.3, 0.9, 0.1, 0.25],
        Thinking => [0.2, 1.0, 0.2, 0.1],
        Neutral => [0.55, 0.3, 0.4, 0.1],
        Sad => [0.0, 0.1, 0.5, 0.7],
        Angry => [0.0, 0.1, 0.1, 0.9],
        Worried => [0.1, 0.2, 0.4, 0.85],
        Tired => [0.2, 0.1, 0.3, 0.55],
    }
}

fn clamp01(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

/// 聚合全员情绪 + 关系 + 身体信号，产出控制总览所需的四维向量与仪表指标
///
/// - 愉悦 = 全员情绪加权愉悦 0.65 + 关系好感 0.35
/// - 好奇 = 全员情绪加权好奇 0.5 + 关系好奇心均值 0.5
/// - 共情 = 全员情绪加权共情 0.5 + (好感+信任+互惠)/3 0.5
/// - 压力 = 全员情绪加权压力 0.55 + (紧张+烦躁)/2 0.25 + (1-社交余量) 0.20，
///         有危机记录时抬底
/// - coherence = 平静/正面情绪的质量占比；entropy = 情绪分布香农熵(归一化)；
///   resonance = 平均互动频率归一化
fn emotion_core_json() -> Result<serde_json::Value, String> {
    let db = crate::db::db();

    // ── 1. 全员情绪状态 ──
    let states = db.all_emotion_states().map_err(|e| e.to_string())?;
    let mut sum = [0.0f32; 4];
    let mut weight_total = 0.0f32;
    let mut type_mass: std::collections::BTreeMap<String, f32> = std::collections::BTreeMap::new();
    let mut positive_mass = 0.0f32;
    let mut rate_sum = 0.0f32;
    let mut rate_n = 0usize;
    let mut crisis_hits = 0usize;
    let mut users = 0usize;
    let mut self_state: Option<serde_json::Value> = None;

    for (uid, json) in &states {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else {
            continue;
        };
        let Ok(etype) = serde_json::from_value::<crate::emotion::EmotionType>(
            v.get("current").cloned().unwrap_or(serde_json::Value::Null),
        ) else {
            continue;
        };
        users += 1;
        let intensity = v
            .get("intensity")
            .and_then(|x| x.as_f64())
            .unwrap_or(0.3) as f32;
        let w = intensity.max(0.05);
        let vec = emotion_vector(&etype);
        for i in 0..4 {
            sum[i] += vec[i] * w;
        }
        weight_total += w;
        *type_mass.entry(format!("{:?}", etype)).or_insert(0.0) += w;
        positive_mass += vec[0] * w;
        if let Some(rate) = v.get("interaction_rate").and_then(|x| x.as_f64()) {
            rate_sum += rate as f32;
            rate_n += 1;
        }
        if let Some(level) = v.get("crisis_level").and_then(|x| x.as_str()) {
            if level != "None" {
                crisis_hits += 1;
            }
        }
        if *uid == 0 {
            self_state = Some(v);
        }
    }

    let (emo_joy, emo_cur, emo_emp, emo_stress) = if weight_total > 0.0 {
        (
            sum[0] / weight_total,
            sum[1] / weight_total,
            sum[2] / weight_total,
            sum[3] / weight_total,
        )
    } else {
        // 无任何情绪数据时的静息值
        (0.55, 0.40, 0.50, 0.20)
    };

    // ── 2. 关系维度 ──
    let rels = db
        .all_per_user_states(crate::db::PerUserState::Relationship)
        .map_err(|e| e.to_string())?;
    let (mut cur_sum, mut aff_sum, mut trust_sum, mut recp_sum, mut tens_sum, mut annoy_sum) =
        (0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);
    let mut rel_n = 0usize;
    for (_, json) in &rels {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else {
            continue;
        };
        let num = |k: &str| v.get(k).and_then(|x| x.as_f64()).unwrap_or(0.0) as f32;
        cur_sum += num("curiosity");
        aff_sum += num("affection");
        trust_sum += num("trust");
        recp_sum += num("reciprocity");
        tens_sum += num("tension");
        annoy_sum += num("annoyance");
        rel_n += 1;
    }
    let rel_avg = |sum: f32| -> f32 {
        if rel_n > 0 {
            sum / rel_n as f32
        } else {
            0.0
        }
    };
    let (curiosity_rel, affection, trust, reciprocity, tension, annoyance) = (
        rel_avg(cur_sum),
        rel_avg(aff_sum),
        rel_avg(trust_sum),
        rel_avg(recp_sum),
        rel_avg(tens_sum),
        rel_avg(annoy_sum),
    );
    // 无关系数据时的默认好感/信任/互惠（让四维仍有合理落点）
    let (affinity, trust_v, reciprocity_v) = if rel_n > 0 {
        (affection, trust, reciprocity)
    } else {
        (0.55, 0.5, 0.5)
    };

    // ── 3. 身体信号（精力/社交余量） ──
    let signals = crate::mind::body_signals();
    let signal_level = |name: &str| -> Option<f32> {
        signals
            .iter()
            .find(|s| s.name == name)
            .map(|s| s.level.clamp(0.0, 1.0))
    };
    let battery = signal_level("社交余量");
    let energy = signal_level("精力");

    // ── 4. 四维融合 ──
    let mut joy = clamp01(0.65 * emo_joy + 0.35 * affinity);
    let curiosity = clamp01(0.5 * emo_cur + 0.5 * if rel_n > 0 { curiosity_rel } else { 0.4 });
    let empathy = clamp01(
        0.5 * emo_emp + 0.5 * (affinity + trust_v + reciprocity_v) / 3.0,
    );
    let battery_stress = 1.0 - battery.unwrap_or(0.5);
    let mut stress = clamp01(0.55 * emo_stress + 0.25 * (tension + annoyance) / 2.0 + 0.20 * battery_stress);
    if crisis_hits > 0 {
        stress = stress.max(0.6);
    }

    // ── 5. 仪表指标 ──
    let total_mass: f32 = type_mass.values().sum();
    let entropy_norm = if total_mass > 0.0 && type_mass.len() > 1 {
        let h: f32 = type_mass
            .values()
            .map(|m| {
                let p = m / total_mass;
                -p * p.ln()
            })
            .sum();
        clamp01(h / 11f32.ln())
    } else {
        0.0
    };
    let coherence = if weight_total > 0.0 {
        clamp01(positive_mass / weight_total)
    } else {
        0.6
    };
    let resonance = if rate_n > 0 {
        clamp01(rate_sum / rate_n as f32 / 3.0)
    } else {
        0.4
    };

    // 自身情绪（uid=0）作为前端状态标签；无记录时用 Neutral
    let self_current = self_state
        .as_ref()
        .and_then(|v| v.get("current"))
        .and_then(|v| v.as_str())
        .unwrap_or("Neutral")
        .to_string();
    let self_intensity = self_state
        .as_ref()
        .and_then(|v| v.get("intensity"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.3);
    // 愉悦过低时（全员低落）给一个下限，避免核心完全熄灭
    joy = joy.max(0.08);

    Ok(serde_json::json!({
        "joy": joy,
        "curiosity": curiosity,
        "empathy": empathy,
        "stress": stress,
        "coherence": coherence,
        "resonance": resonance,
        "entropy": entropy_norm,
        "energy": energy,
        "battery": battery,
        "state": self_current,
        "state_intensity": self_intensity,
        "users": users,
        "relationships": rel_n,
        "crisis": crisis_hits,
        "updated": crate::util::now_secs(),
    }))
}

// ── Handler: 黑名单 ──────────────────────────────────────────

pub(crate) fn handle_blocklist(
    method: &Method,
    segs: &[&str],
    body: &[u8],
) -> Response<std::io::Cursor<Vec<u8>>> {
    // 运行时黑名单由进程级门禁状态持有（`crate::get_blacklist`），
    // 落盘是它的快照。这里不再自己读写 blocklist.json——第二条写路径
    // 会让主循环看到的内存集合与文件不一致。
    match method {
        Method::Get => {
            let runtime = crate::get_blacklist();
            // 配置里的 blacklist 是启动时并入运行时的静态名单，仍然分别展示，
            // 便于定位"这条为什么被拉黑"。
            let from_config = config::get().blacklist.clone();
            let mut all = runtime.clone();
            for uid in &from_config {
                if !all.contains(uid) {
                    all.push(*uid);
                }
            }
            ok(serde_json::json!({
                "blocked": all,
                "from_runtime": runtime,
                "from_config": from_config,
            }))
        }
        Method::Post => {
            let body_val: serde_json::Value = match parse_json(body) {
                Ok(v) => v,
                Err(e) => return err(400, &e),
            };
            let uid = match body_val.get("user_id").and_then(|v| v.as_u64()) {
                Some(u) => u,
                None => return err(400, "user_id required"),
            };
            backup::before_modify("blocklist");
            crate::set_blacklisted(crate::db::Actor::Admin, uid, true);
            ok(serde_json::json!({"ok": true}))
        }
        Method::Delete => {
            let uid: u64 = match segs.first().and_then(|s| s.parse().ok()) {
                Some(u) => u,
                None => return err(400, "user_id required"),
            };
            backup::before_modify("blocklist");
            crate::set_blacklisted(crate::db::Actor::Admin, uid, false);
            ok(serde_json::json!({"ok": true}))
        }
        _ => err(405, "method not allowed"),
    }
}

// ── Handler: 归档 ──────────────────────────────────────────────

pub(crate) fn handle_archive() -> Response<std::io::Cursor<Vec<u8>>> {
    let path = config::data_dir().join("archive.json");
    let data = std::fs::read_to_string(&path).unwrap_or_else(|_| "{}".into());
    let store: serde_json::Value = serde_json::from_str(&data)
        .unwrap_or(serde_json::json!({"working_memory": [], "long_term": []}));
    ok(store)
}

// ── Handler: 备份 ──────────────────────────────────────────────

pub(crate) fn handle_backups(
    method: &Method,
    segs: &[&str],
    body: &[u8],
) -> Response<std::io::Cursor<Vec<u8>>> {
    match method {
        Method::Get => {
            if let Some(data_type) = segs.first() {
                ok(backup::list(data_type))
            } else {
                ok(backup::list_all_types())
            }
        }
        Method::Post => {
            let body_val: serde_json::Value = match parse_json(body) {
                Ok(v) => v,
                Err(e) => return err(400, &e),
            };
            let action = body_val
                .get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let data_type = body_val
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("memory");
            match action {
                "create" => {
                    backup::before_modify(data_type);
                    ok(serde_json::json!({"ok": true}))
                }
                "restore" => {
                    let filename = match body_val.get("filename").and_then(|v| v.as_str()) {
                        Some(f) => f,
                        None => return err(400, "filename required"),
                    };
                    match backup::restore(data_type, filename) {
                        Ok(()) => ok(serde_json::json!({"ok": true})),
                        Err(e) => err(400, &e),
                    }
                }
                "delete" => {
                    let filename = match body_val.get("filename").and_then(|v| v.as_str()) {
                        Some(f) => f,
                        None => return err(400, "filename required"),
                    };
                    let backup_path = config::data_dir()
                        .join("backups")
                        .join(data_type)
                        .join(filename);
                    if backup_path.exists() {
                        std::fs::remove_file(&backup_path).ok();
                    }
                    ok(serde_json::json!({"ok": true}))
                }
                _ => err(400, "unknown action: use create, restore, or delete"),
            }
        }
        _ => err(405, "method not allowed"),
    }
}

// ── 配额追踪 ────────────────────────────────────────────────────

pub(crate) fn handle_quota(method: &Method, segs: &[&str]) -> Response<std::io::Cursor<Vec<u8>>> {
    if *method != Method::Get {
        return err(405, "method not allowed");
    }
    if !segs.is_empty() {
        return err(404, "not found");
    }
    // API quota 配置
    let cfg = &config::get().quota;
    ok(serde_json::json!({
        "enabled": cfg.enabled,
        "segment_minutes": cfg.segment_minutes,
        "segments": cfg.segments,
    }))
}

// ── 防注入状态管理 ────────────────────────────────────────────────

pub(crate) fn handle_anti_injection(
    method: &Method,
    segs: &[&str],
) -> Response<std::io::Cursor<Vec<u8>>> {
    match method {
        Method::Get => {
            // GET /api/anti-injection/users - 获取所有用户风险状态
            if segs.first() == Some(&"users") {
                let users = crate::anti_injection::get_all_user_statuses();
                return ok(serde_json::json!({"users": users}));
            }
            // GET /api/anti-injection/:user_id - 获取特定用户状态
            if let Some(&user_id_str) = segs.first()
                && let Ok(user_id) = user_id_str.parse::<u64>()
            {
                let status = crate::anti_injection::get_user_status(user_id);
                let reputation = crate::anti_injection::get_reputation(user_id);
                let violation_count = crate::anti_injection::get_violation_count(user_id);
                let vision_disabled = crate::anti_injection::is_vision_disabled(user_id);
                let silent_banned = crate::anti_injection::is_silent_banned(user_id);
                let penalty = crate::anti_injection::get_penalty_multiplier(user_id);

                return ok(serde_json::json!({
                    "user_id": user_id,
                    "status": status,
                    "reputation": reputation,
                    "violation_count": violation_count,
                    "vision_disabled": vision_disabled,
                    "silent_banned": silent_banned,
                    "penalty_multiplier": penalty,
                }));
            }

            // 返回配置信息
            let cfg = &config::get().anti_injection;
            ok(serde_json::json!({
                "config": {
                    "input": {
                        "sensitive_action": cfg.input.sensitive_action,
                    },
                    "output": {
                        "action": cfg.output.action,
                    },
                    "behavior": {
                        "rate_limit": cfg.behavior.rate_limit,
                        "max_messages_per_minute": cfg.behavior.max_messages_per_minute,
                        "max_messages_per_hour": cfg.behavior.max_messages_per_hour,
                        "reputation_threshold": cfg.behavior.reputation_threshold,
                        "auto_ban": cfg.behavior.auto_ban,
                        "auto_ban_threshold": cfg.behavior.auto_ban_threshold,
                    }
                },
                "note": "关键词过滤、注入模式检测、编码绕过检测、色情/暴力/违法内容检测、输出检测始终强制开启"
            }))
        }
        Method::Post => {
            // POST /api/anti-injection/{user_id}/{action}
            if segs.len() < 2 {
                return err(400, "path required: /api/anti-injection/{user_id}/{action}");
            }
            let user_id = match segs[0].parse::<u64>() {
                Ok(u) => u,
                Err(_) => return err(400, "invalid user_id"),
            };
            let action = segs[1];

            match action {
                "unban" => {
                    crate::anti_injection::unban_user(user_id);
                    ok(
                        serde_json::json!({"success": true, "message": format!("用户{}已解封", user_id)}),
                    )
                }
                "enable-vision" => {
                    crate::anti_injection::enable_vision(user_id);
                    ok(
                        serde_json::json!({"success": true, "message": format!("用户{}识图已启用", user_id)}),
                    )
                }
                "reset-reputation" => {
                    crate::anti_injection::reset_reputation(user_id);
                    ok(
                        serde_json::json!({"success": true, "message": format!("用户{}信誉已重置", user_id)}),
                    )
                }
                _ => err(404, "unknown action"),
            }
        }
        _ => err(405, "method not allowed"),
    }
}

// ── 配置管理 ──────────────────────────────────────────────────

pub(crate) fn handle_config(
    method: &Method,
    segs: &[&str],
    body: &[u8],
) -> Response<std::io::Cursor<Vec<u8>>> {
    // GET /api/config/status — 配置解析状态
    if *method == Method::Get && segs.first() == Some(&"status") {
        let error = config::error_message();
        let runtime = serde_json::to_value(config::get()).ok();
        let disk = std::fs::read_to_string(config::data_dir().join("config.yaml"))
            .ok()
            .and_then(|content| serde_yaml::from_str::<serde_json::Value>(&content).ok());
        let pending_file_changes = match (&runtime, &disk) {
            (Some(active), Some(saved)) => active != saved,
            _ => false,
        };
        return ok(serde_json::json!({
            "ok": error.is_empty(),
            "error": error,
            "pending_file_changes": pending_file_changes
        }));
    }
    // POST /api/config/reload — 热重载配置
    if *method == Method::Post && segs.first() == Some(&"reload") {
        match config::reload() {
            Ok(()) => return ok(serde_json::json!({"ok": true, "message": "配置已重新载入"})),
            Err(e) => return err(500, &e),
        }
    }
    if !segs.is_empty() {
        return err(404, "not found");
    }
    // 原有的 config GET/PUT 逻辑
    handle_config_main(method, body)
}

fn handle_config_main(method: &Method, body: &[u8]) -> Response<std::io::Cursor<Vec<u8>>> {
    let config_path = config::data_dir().join("config.yaml");
    match method {
        Method::Get => {
            // 展示当前运行时快照，而不是未经应用的磁盘内容。
            // GET /api/config/status 会单独提示文件是否存在待载入修改。
            let mut cfg: serde_json::Value = match serde_json::to_value(config::get()) {
                Ok(v) => v,
                Err(e) => return err(500, &format!("serialize runtime config: {e}")),
            };
            // 脱敏：隐藏 api_key
            if let Some(obj) = cfg.as_object_mut() {
                if let Some(key) = obj.get_mut("api_key")
                    && let Some(s) = key.as_str()
                    && s.len() > 8
                {
                    *key = serde_json::json!(format!("{}...{}", &s[..4], &s[s.len() - 4..]));
                }
                if let Some(v) = obj.get_mut("vision").and_then(|v| v.as_object_mut())
                    && let Some(key) = v.get_mut("api_key")
                    && let Some(s) = key.as_str()
                    && s.len() > 8
                {
                    *key = serde_json::json!(format!("{}...{}", &s[..4], &s[s.len() - 4..]));
                }
            }
            ok(cfg)
        }
        Method::Put => {
            let new_cfg: serde_json::Value = match serde_json::from_slice(body) {
                Ok(v) => v,
                Err(e) => return err(400, &format!("invalid json: {}", e)),
            };
            // 读取现有配置以保留未发送的字段
            let existing = std::fs::read_to_string(&config_path).unwrap_or_default();
            let existing_cfg: serde_json::Value =
                serde_yaml::from_str(&existing).unwrap_or(serde_json::json!({}));

            // 深合并：新配置中未发送的嵌套字段保留原值
            let mut merged = deep_merge(&existing_cfg, &new_cfg);

            // 脱敏字段还原：包含 "..." 的 api_key 保留原值
            if let (Some(new_obj), Some(old_obj)) =
                (merged.as_object_mut(), existing_cfg.as_object())
            {
                // api_key
                if let Some(key) = new_obj.get("api_key").and_then(|v| v.as_str())
                    && key.contains("...")
                    && let Some(old_key) = old_obj.get("api_key")
                {
                    new_obj.insert("api_key".to_string(), old_key.clone());
                }
                // vision.api_key
                if let (Some(new_vis), Some(old_vis)) = (
                    new_obj.get_mut("vision").and_then(|v| v.as_object_mut()),
                    old_obj.get("vision").and_then(|v| v.as_object()),
                ) && let Some(key) = new_vis.get("api_key").and_then(|v| v.as_str())
                    && key.contains("...")
                    && let Some(old_key) = old_vis.get("api_key")
                {
                    new_vis.insert("api_key".to_string(), old_key.clone());
                }
            }

            // 类型化保存：反序列化成 Config 再原子落盘。
            // 同时在写入前检查人设文件，避免保存后热重载失败。
            let parsed: config::Config = match serde_json::from_value(merged.clone()) {
                Ok(cfg) => cfg,
                Err(e) => return err(400, &format!("配置校验失败（未写入）: {e}")),
            };
            let prompt_path = config::data_dir().join("prompts").join(&parsed.prompts);
            if !prompt_path.is_file() {
                return err(400, &format!("人设文件不存在（未写入）: {}", prompt_path.display()));
            }

            // 这些设置绑定了启动时创建的资源，更新 Config 快照并不能让对应资源
            // 自动重建；明确告知用户，不把“配置已热重载”误报成“全部即时生效”。
            let restart_fields = [
                ("admin.port", "管理 WebUI 端口在启动时绑定"),
                ("log.enabled", "日志输出管线在启动时创建"),
                ("log.level", "日志过滤器在启动时创建"),
                ("self_qq", "机器人身份自检在启动时执行"),
                ("auto_start_users", "自动启动私聊名单在启动时应用"),
                ("auto_start_groups", "自动启动群聊名单在启动时应用"),
            ];
            let restart_required: Vec<serde_json::Value> = restart_fields
                .iter()
                .filter(|(path, _)| config_path_value(&existing_cfg, path) != config_path_value(&merged, path))
                .map(|(field, reason)| serde_json::json!({"field": field, "reason": reason}))
                .collect();

            let reauth_required =
                config_path_value(&existing_cfg, "admin.token") != config_path_value(&merged, "admin.token");
            let requires_restart = !restart_required.is_empty();

            if let Err(e) = config::save(&parsed) {
                return err(500, &format!("write config: {e}"));
            }
            if let Err(e) = config::reload() {
                return err(500, &format!("配置已保存，但运行时应用失败：{e}"));
            }
            ok(serde_json::json!({
                "ok": true,
                "applied": true,
                "restart_required": restart_required,
                "reauth_required": reauth_required,
                "message": if !requires_restart {
                    "配置已保存并应用到运行时".to_string()
                } else {
                    "可热更新项已立即应用；部分配置需要重启才能完全生效".to_string()
                }
            }))
        }
        _ => err(405, "method not allowed"),
    }
}

// ── 日程计划 ──────────────────────────────────────────────────

pub(crate) fn handle_analytics() -> Response<std::io::Cursor<Vec<u8>>> {
    ok(crate::tracking::summary())
}

/// Turn 契约影子观测：最近 24 小时的分布
///
/// 这是"要不要把「文本即发言」换成严格 `Turn` tagged union"的决策依据——
/// 看 `failure_rate`：它就是在当前流量下，严格契约会失败的比例。
pub(crate) fn handle_turn_shadow() -> Response<std::io::Cursor<Vec<u8>>> {
    ok(crate::ai::shadow::report(24 * 3600).to_json())
}

/// 后台操作审计：最近 100 条"谁在什么时候改了什么"
///
/// 审计表此前**只写不读**——写入方（[`crate::db::Db::record_audit`]）在，
/// 读方没有出口，只有它自己的单元测试读过。没有读方的审计回答不了它本来
/// 要回答的问题，所以补上这个出口而不是删掉读方。
pub(crate) fn handle_audit() -> Response<std::io::Cursor<Vec<u8>>> {
    const PAGE_SIZE: usize = 100;

    match crate::db::db().recent_audit(PAGE_SIZE) {
        Ok(entries) => {
            let items: Vec<serde_json::Value> = entries
                .iter()
                .map(|entry| {
                    serde_json::json!({
                        "actor": entry.actor,
                        "command": entry.command,
                        "detail": entry.detail,
                        "created_at": entry.created_at,
                    })
                })
                .collect();
            ok(serde_json::json!({ "entries": items }))
        }
        Err(error) => err(500, &format!("读取审计失败: {error}")),
    }
}

// ── 日程计划 ──────────────────────────────────────────────────

pub(crate) fn handle_schedule(method: &Method) -> Response<std::io::Cursor<Vec<u8>>> {
    if *method != Method::Get {
        return err(405, "method not allowed");
    }

    // GET: 返回计划数据
    //
    // 这里**绝不能**调用会写状态的推动逻辑：早先版本在 GET 里调了
    // `check_plan_push()`（它会把当天内容标记成"已推送"），于是每打开
    // 一次日程页面就替她消耗掉一次推动，bot 再也收不到。
    let mut timeframes = serde_json::Map::new();
    for timeframe in crate::schedule::Timeframe::ALL {
        let plan = crate::schedule::plan_of(timeframe);
        let total = plan.items.len();
        let done = plan.items.iter().filter(|i| i.completed).count();
        let key = match timeframe {
            crate::schedule::Timeframe::Day => "day",
            crate::schedule::Timeframe::Week => "week",
            crate::schedule::Timeframe::Month => "month",
        };
        timeframes.insert(
            key.to_string(),
            serde_json::json!({
                "label": timeframe.label(),
                "period": plan.period,
                "items": plan.items,
                "total": total,
                "done": done,
                "reflection": plan.reflection,
            }),
        );
    }

    ok(serde_json::json!({
        "timeframes": timeframes,
        "history": crate::schedule::push_history(),
    }))
}

// ── 对话管理 ──────────────────────────────────────────────────

pub(crate) fn handle_conversations(
    method: &Method,
    segs: &[&str],
) -> Response<std::io::Cursor<Vec<u8>>> {
    match method {
        Method::Get => {
            // GET /api/conversations -- 列出所有活跃群聊和私聊
            let groups = crate::get_active_groups();
            let users = crate::get_active_users();
            ok(serde_json::json!({
                "groups": groups,
                "private_users": users,
            }))
        }
        Method::Post => {
            // POST /api/conversations/group/{id}/enable
            // POST /api/conversations/group/{id}/disable
            // POST /api/conversations/private/{id}/enable
            // POST /api/conversations/private/{id}/disable
            if segs.len() < 3 {
                return err(
                    400,
                    "path: /api/conversations/{group|private}/{id}/{enable|disable}",
                );
            }
            let kind = segs[0];
            let id: u64 = match segs[1].parse() {
                Ok(v) => v,
                Err(_) => return err(400, "invalid id"),
            };
            let enable = match segs[2] {
                "enable" => true,
                "disable" => false,
                _ => return err(400, "action must be enable or disable"),
            };

            let changed = match kind {
                "group" => crate::toggle_group_chat(crate::db::Actor::Admin, id, enable),
                "private" => crate::toggle_private_chat(crate::db::Actor::Admin, id, enable),
                _ => return err(400, "kind must be group or private"),
            };

            let action = if enable { "开启" } else { "关闭" };
            let target = if kind == "group" {
                format!("群{}", id)
            } else {
                format!("用户{}", id)
            };
            ok(serde_json::json!({
                "ok": true,
                "changed": changed,
                "message": if changed { format!("已{}{}", action, target) } else { format!("{}已处于{}状态", target, action) }
            }))
        }
        _ => err(405, "method not allowed"),
    }
}

// ── Handler: 人性化状态 ──────────────────────────────────────────

pub(crate) fn handle_humanity() -> Response<std::io::Cursor<Vec<u8>>> {
    let cfg = config::get();

    let battery = if cfg.humanity.social_battery_enabled {
        let b = crate::social_battery::load();
        Some(serde_json::json!({
            "level": b.level,
            "capacity": b.capacity,
            "percentage": crate::social_battery::level_percentage(&b),
            "is_burned_out": b.is_burned_out,
            "is_passive_mode": b.is_passive_mode,
            "active_minutes": b.active_minutes,
        }))
    } else {
        None
    };

    let circadian = if cfg.humanity.circadian_enabled {
        let c = crate::circadian::calculate();
        Some(serde_json::json!({
            "energy_level": c.energy_level,
            "cognitive_clarity": c.cognitive_clarity,
            "patience_level": c.patience_level,
            "sociability": c.sociability,
            "humor_sensitivity": c.humor_sensitivity,
            "current_hour": c.current_hour,
            "is_quiet_hours": crate::circadian::is_quiet_hours(),
        }))
    } else {
        None
    };

    let attention = if cfg.humanity.attention_enabled {
        let a = crate::conversation::attention::load_attention();
        Some(serde_json::json!({
            "attention_level": a.attention_level,
            "flow_state": a.flow_state,
            "focused_topic": a.focused_topic,
            "flow_recovering": a.flow_recovery_until > crate::util::now_secs(),
        }))
    } else {
        None
    };

    let biases = if cfg.humanity.cognitive_biases_enabled {
        let b = crate::memory::cognitive_biases::load_biases();
        Some(serde_json::json!({
            "confirmation_bias": b.confirmation_bias,
            "mood_congruence": b.mood_congruence,
            "anchoring_strength": b.anchoring_strength,
            "availability_heuristic": b.availability_heuristic,
        }))
    } else {
        None
    };

    // 关系已迁入状态库：读旧文件会永远拿到迁移那一刻的快照
    let rel_count = crate::db::db()
        .per_user_count(crate::db::PerUserState::Relationship)
        .unwrap_or(0);

    ok(serde_json::json!({
        "social_battery": battery,
        "circadian": circadian,
        "attention": attention,
        "cognitive_biases": biases,
        "relationship_count": rel_count,
        "config_enabled": {
            "social_battery": cfg.humanity.social_battery_enabled,
            "circadian": cfg.humanity.circadian_enabled,
            "attention": cfg.humanity.attention_enabled,
            "cognitive_biases": cfg.humanity.cognitive_biases_enabled,
            "response_timing": cfg.humanity.response_timing_enabled,
            "unpredictability": cfg.humanity.unpredictability_enabled,
        },
    }))
}

pub(crate) fn handle_relationships(
    method: &Method,
    segs: &[&str],
) -> Response<std::io::Cursor<Vec<u8>>> {
    if *method != Method::Get {
        return err(405, "method not allowed");
    }
    if let Some(uid) = segs.first() {
        // 单用户详细关系数据（包含新维度）
        let uid_num: u64 = match uid.parse() {
            Ok(v) => v,
            Err(_) => return err(400, "invalid user_id"),
        };
        let summary = crate::person_info::relationship::get_relationship_summary(uid_num);
        return ok(summary);
    }
    // 关系已迁入状态库：保持前端既有形状 { "relationships": { "<uid>": {...} } }
    let stored = match crate::db::db().all_per_user_states(crate::db::PerUserState::Relationship) {
        Ok(stored) => stored,
        Err(error) => return err(500, &format!("读取关系失败: {error}")),
    };
    let mut view = serde_json::Map::new();
    for (user_id, json) in stored {
        match serde_json::from_str::<serde_json::Value>(&json) {
            Ok(state) => {
                view.insert(user_id.to_string(), state);
            }
            Err(error) => warn!(%error, user_id, "admin: 关系解析失败，已跳过"),
        }
    }
    ok(serde_json::json!({ "relationships": view }))
}

// ── Handler: 内存操作日志 ──────────────────────────────────────────

pub(crate) fn handle_memory_ops_log(
    method: &Method,
    segs: &[&str],
) -> Response<std::io::Cursor<Vec<u8>>> {
    match method {
        Method::Get => {
            let limit = segs
                .first()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(500);
            let logs = crate::memory::ops_log::get_logs(Some(limit));
            ok(serde_json::json!({ "entries": logs, "total": logs.len() }))
        }
        Method::Post => {
            if segs.first() == Some(&"clear") {
                crate::memory::ops_log::clear();
                return ok(serde_json::json!({"ok": true, "message": "日志已清空"}));
            }
            err(400, "use /api/memory-ops-log/clear to clear")
        }
        _ => err(405, "method not allowed"),
    }
}

/// 深合并两个 JSON 对象
/// - 对于两个都是 Object 的 key：递归合并
/// - 对于其他情况：新值覆盖旧值
///
/// 这样前端发送部分嵌套字段时，不会丢失未发送的字段
/// Resolve a dotted path in a JSON value; used to report settings that require restart.
fn config_path_value<'a>(value: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    path.split('.').try_fold(value, |current, segment| current.get(segment))
}

fn deep_merge(base: &serde_json::Value, patch: &serde_json::Value) -> serde_json::Value {
    match (base, patch) {
        (serde_json::Value::Object(base_map), serde_json::Value::Object(patch_map)) => {
            let mut result = base_map.clone();
            for (key, patch_val) in patch_map {
                if let Some(base_val) = result.get(key) {
                    // 两边都是 Object → 递归深合并
                    if base_val.is_object() && patch_val.is_object() {
                        result.insert(key.clone(), deep_merge(base_val, patch_val));
                    } else {
                        // 其他类型（包括 Array）：新值覆盖
                        result.insert(key.clone(), patch_val.clone());
                    }
                } else {
                    // patch 中有而 base 中没有的 key：直接插入
                    result.insert(key.clone(), patch_val.clone());
                }
            }
            serde_json::Value::Object(result)
        }
        // 非 Object 类型：直接返回 patch
        _ => patch.clone(),
    }
}

// ── Handler: 心灵（意识流/日记/档案/心事/审计） ────────────────

fn stream_event_json(event: &crate::mind::StreamEvent) -> serde_json::Value {
    let mut value = serde_json::json!({
        "kind": event.kind,
        "content": event.content,
        "time": event.time,
        "about": event.about,
    });
    if let Some(recall) = &event.recall {
        value["recall"] = serde_json::json!({
            "id": recall.id,
            "source": recall.source,
            "completed": crate::mind::recall::is_completed(&recall.id),
            "completed_at": crate::mind::recall::completed_at(&recall.id),
        });
    }
    value
}

fn mind_now() -> serde_json::Value {
    let signals: Vec<serde_json::Value> = crate::mind::body_signals()
        .iter()
        .map(|s| serde_json::json!({"name": s.name, "level": s.level}))
        .collect();
    let loops: Vec<serde_json::Value> = crate::mind::wake::all()
        .iter()
        .map(|p| {
            serde_json::json!({
                "id": p.id, "kind": p.kind, "due_at": p.due_at,
                "reason": p.reason, "about_user": p.about_user,
                "target_group": p.target_group, "target_user": p.target_user,
            })
        })
        .collect();
    let recent_stream: Vec<serde_json::Value> = crate::mind::recent(2 * 3600, 20)
        .iter()
        .map(stream_event_json)
        .collect();
    let today = crate::util::ts_to_date_str(crate::util::now_secs());
    let diary_today = crate::mind::diary::recent(100)
        .iter()
        .filter(|e| e.date == today)
        .count();
    serde_json::json!({
        "night": crate::mind::is_night(),
        "body": signals,
        "loops": loops,
        "recent_stream": recent_stream,
        "diary_today": diary_today,
        "time": crate::util::now_formatted_cst(),
    })
}

pub(crate) fn handle_mind(
    method: &Method,
    segs: &[&str],
) -> Response<std::io::Cursor<Vec<u8>>> {
    let section = segs.first().copied().unwrap_or("");
    let rest = &segs[1.min(segs.len())..];
    match (method, section) {
        (Method::Get, "now") => ok(mind_now()),
        (Method::Get, "stream") => match rest.first() {
            Some(date) => {
                let events: Vec<serde_json::Value> = crate::mind::stream::events_on_date(date)
                    .iter()
                    .map(stream_event_json)
                    .collect();
                ok(serde_json::json!({"date": date, "events": events}))
            }
            None => ok(serde_json::json!({"dates": crate::mind::stream::known_dates()})),
        },
        (Method::Get, "diary") => {
            let entries: Vec<serde_json::Value> = crate::mind::diary::recent(200)
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "id": e.id, "date": e.date, "content": e.content,
                        "feeling": e.feeling, "about": e.about,
                    })
                })
                .collect();
            ok(serde_json::json!({"entries": entries}))
        }
        (Method::Get, "persons") => {
            let persons: Vec<serde_json::Value> = crate::mind::persons::all()
                .into_iter()
                .map(|(uid, file)| serde_json::json!({"user_id": uid, "file": file}))
                .collect();
            ok(serde_json::json!({"persons": persons}))
        }
        (Method::Get, "security") => {
            ok(serde_json::json!({"events": crate::mind::security::tail(300)}))
        }
        (Method::Get, "social") => match rest.first() {
            Some(gid) => match gid.parse::<u64>() {
                Ok(group_id) => ok(serde_json::json!({
                    "group_id": group_id,
                    "state": crate::mind::social::state_for_admin(group_id),
                })),
                Err(_) => err(400, "invalid group_id"),
            },
            None => ok(serde_json::json!({
                "groups": crate::mind::social::known_group_ids(),
            })),
        },
        (Method::Get, "kernel") => match crate::mind::self_model::kernel() {
            Some(k) => ok(serde_json::json!({"kernel": k})),
            None => err(404, "kernel.json 不存在——先在 data/self/kernel.json 创建"),
        },
        _ => err(404, "not found"),
    }
}
