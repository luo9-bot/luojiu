//! 注意力漂移：人不会一直紧盯着正题
//!
//! 人性缺口：真人会被新鲜、好笑、反差强或者熟悉的东西勾走一下再回来。移植时只搬"发散—回钩—短反应"
//! 这三层结构，档位文案按这里的世界重写；写死在代码里的是**档位**，
//! 提示词本体仍然走 `defaults/attention_drift.prompt`，可以热重载。
//!
//! 档位名写错不会报错，回落到默认档位：prompt 层缺东西应当表现为
//! "这一层没有内容"，不该让整条回复路径消失。

use std::collections::HashMap;

/// 漂移档位：发散到什么程度
const DRIFT_RULES: &[(&str, &str)] = &[
    (
        "subtle",
        "漂移档位：轻微。只在最近消息里出现非常自然的触发点时轻轻联想一句，多数时候继续当前话题",
    ),
    (
        "active",
        "漂移档位：活跃。可以主动抓住新鲜、好笑、反差强或熟悉的细节接话，但回复要短，而且能从最近消息里解释得通",
    ),
    (
        "scattered",
        "漂移档位：明显发散。可以被支线、关键词、熟人语气或反差点勾走，先接住那个点再回正题；一轮里允许一次可理解的突然拐弯",
    ),
    (
        "wild",
        "漂移档位：强烈跳跃。可以先被最有趣的细节劫走一下，出现短促插话或半路拐弯；但一轮最多一次明显跳跃，不能无视明确提问，最后要让人看得出你在接哪条消息",
    ),
];

/// 回钩策略：拐出去之后要不要回来
const ANCHOR_RULES: &[(&str, &str)] = &[
    (
        "strict",
        "回钩策略：严格。联想或短反应之后，立刻回到当前正在聊的主题或被回复对象",
    ),
    (
        "balanced",
        "回钩策略：自然。可以顺着支线说一句，但结尾或主要意思要回到当前聊天",
    ),
    (
        "loose",
        "回钩策略：宽松。可以保留更自由的相关联想，但不能凭空换话题，也不能无视明确提问",
    ),
];

/// 短反应风格：接话的第一句长什么样
const REACTION_RULES: &[(&str, &str)] = &[
    (
        "reserved",
        "短反应风格：克制。只有特别适合接话时，才用一句很短的反应开头",
    ),
    (
        "natural",
        "短反应风格：自然。可以偶尔先用短句、吐槽或语气词接住话题，再继续往下说",
    ),
    (
        "lively",
        "短反应风格：活泼。更容易先用短促反应开头，但别把话说得太碎，也别每次都这样",
    ),
];

/// 取档位文案，认不出的档位回落到默认值
fn pick(table: &[(&str, &str)], key: &str, fallback: &str) -> String {
    table
        .iter()
        .find(|(name, _)| *name == key)
        .or_else(|| table.iter().find(|(name, _)| *name == fallback))
        .map(|(_, rule)| (*rule).to_string())
        .unwrap_or_default()
}

/// 组装注意力漂移块（未开启或没有内容时为空字符串）
pub(crate) fn prompt_block() -> String {
    let cfg = crate::config::get();
    if !cfg.humanity.attention_drift_enabled {
        return String::new();
    }
    let drift = &cfg.humanity.attention_drift;
    let rules = (
        pick(DRIFT_RULES, &drift.drift_level, "active"),
        pick(ANCHOR_RULES, &drift.anchor_policy, "balanced"),
        pick(REACTION_RULES, &drift.reaction_style, "natural"),
    );
    let vars = HashMap::from([
        ("drift_rule", rules.0.as_str()),
        ("anchor_rule", rules.1.as_str()),
        ("reaction_rule", rules.2.as_str()),
    ]);
    crate::prompt::PromptManager::get().render("attention_drift", &vars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_levels_are_picked_verbatim() {
        assert!(pick(DRIFT_RULES, "wild", "active").contains("强烈跳跃"));
        assert!(pick(ANCHOR_RULES, "strict", "balanced").contains("严格"));
    }

    #[test]
    fn unknown_level_falls_back_instead_of_failing() {
        assert_eq!(
            pick(DRIFT_RULES, "nope", "active"),
            pick(DRIFT_RULES, "active", "active")
        );
        assert!(!pick(REACTION_RULES, "", "natural").is_empty());
    }

    #[test]
    fn every_level_has_a_non_empty_rule() {
        for table in [DRIFT_RULES, ANCHOR_RULES, REACTION_RULES] {
            assert!(table.iter().all(|(name, rule)| !name.is_empty() && !rule.is_empty()));
        }
    }
}
