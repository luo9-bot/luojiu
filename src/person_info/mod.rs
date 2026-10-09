//! 人物档案系统

pub(crate) mod relationship;
mod store;

pub(crate) use store::*;

use tracing::info;

pub(crate) fn register_person(user_id: u64) {
    if user_id == 0 {
        return;
    }
    let now = crate::util::now_secs();
    let mut is_new = false;
    update_profile(user_id, |p| {
        if p.know_since == 0 {
            is_new = true;
            p.know_since = now;
        }
    });
    if is_new {
        info!(user_id, "person_info: new person");
    }
}

/// 获取用户显示名称（用于 AI 上下文，避免暴露原始 user_id）
///
/// 人物档案（含创作者播种的名字，见 mind::persons）优先——那是她认人的依据；
/// 其次自动学到的 person_name，再次群昵称。
/// 记录 QQ 消息入口提供的昵称/群名片。
///
/// user_id 是稳定身份；nickname/card 都是可变展示属性。AI 的长期认知
/// （person_name / mind::persons）仍然保留，不会被 QQ 昵称覆盖。
pub(crate) fn update_qq_identity(
    user_id: u64,
    group_id: u64,
    nickname: Option<&str>,
    card: Option<&str>,
) {
    if user_id == 0 {
        return;
    }
    let nickname = nickname.map(str::trim).filter(|s| !s.is_empty());
    let card = card.map(str::trim).filter(|s| !s.is_empty());
    if nickname.is_none() && card.is_none() {
        return;
    }

    update_profile(user_id, |profile| {
        if let Some(name) = nickname {
            profile.qq_nickname = name.to_string();
        }
        if group_id > 0
            && let Some(name) = card.or(nickname)
        {
            profile.group_nicknames.insert(group_id, name.to_string());
        }
    });
}

/// 获取用户显示名称（用于 AI 上下文，避免暴露原始 user_id）
pub(crate) fn get_display_name(user_id: u64, group_id: u64) -> Option<String> {
    if let Some(name) = crate::mind::persons::display_name_or_address(user_id) {
        return Some(name);
    }
    let profile = load_profile(user_id)?;
    if !profile.person_name.is_empty() {
        return Some(profile.person_name);
    }
    if group_id > 0
        && let Some(name) = profile.group_nicknames.get(&group_id)
    {
        return Some(name.clone());
    }
    if !profile.qq_nickname.is_empty() {
        return Some(profile.qq_nickname.clone());
    }
    None
}

/// 返回给 AI 的结构化身份信息：稳定 QQ 号 + QQ 昵称/群名片 + AI 认知名。
pub(crate) fn get_identity_label(user_id: u64, group_id: u64) -> Option<String> {
    if user_id == 0 {
        return None;
    }

    let profile = load_profile(user_id);
    let cognitive_name = crate::mind::persons::display_name_or_address(user_id).or_else(|| {
        profile.as_ref().and_then(|p| {
            if p.person_name.is_empty() {
                None
            } else {
                Some(p.person_name.clone())
            }
        })
    });
    let qq_nickname = profile.as_ref().and_then(|p| {
        if p.qq_nickname.is_empty() {
            None
        } else {
            Some(p.qq_nickname.clone())
        }
    });
    let group_name = if group_id > 0 {
        profile
            .as_ref()
            .and_then(|p| p.group_nicknames.get(&group_id).cloned())
    } else {
        None
    };

    let mut parts = vec![format!("QQ:{user_id}")];
    if let Some(name) = qq_nickname {
        parts.push(format!("QQ昵称:{name}"));
    }
    if let Some(name) = group_name {
        parts.push(format!("群名片:{name}"));
    }
    if let Some(name) = cognitive_name {
        parts.push(format!("认知:{name}"));
    }
    Some(format!("[{}]", parts.join("|")))
}

/// 从对话中自动提取用户事实
///
/// 使用 LLM 从用户消息和 bot 回复中提取稳定事实。
pub(crate) fn extract_facts_from_conversation(user_id: u64, user_message: &str, bot_reply: &str) {
    if user_id == 0 || user_message.is_empty() {
        return;
    }

    let prompt = format!(
        "从以下对话中提取关于用户的稳定事实（如姓名、兴趣、职业、习惯等）。\n\
         只提取有明确证据的事实，不要推测。\n\n\
         用户消息：{}\n\
         Bot回复：{}\n\n\
         如果没有可提取的事实，返回空。否则返回一行一个事实。",
        user_message, bot_reply
    );

    let _ = crate::ai::analyze("", &prompt);
}
