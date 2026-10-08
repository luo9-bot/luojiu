//! 表达学习系统
//!
//! 写入侧：从群聊消息里学语言风格和黑话/梗（`extract`）。
//! 读取侧：她开口时把**眼前命中的**那部分递回去（`context_block`）。
//!
//! 读取侧曾经整条没接线：她学了一肚子这个群的梗，开口时一个都想不起来。
//! 现在黑话按"这批话里有人说了它"来命中，说话路子按见过的次数取最常用的几条——
//! 想起哪个梗是因为眼前有人说了它，而不是每次把词典倒进 prompt。

mod extract;
mod store;

pub(crate) use extract::*;

use std::cmp::Reverse;

use store::{ExpressionHabit, JargonEntry};

/// 一个群最多递几条"说话路子"：多了就成了背台词
const HABIT_LIMIT: usize = 5;

/// 这条经验是给这个群的（`source_group == 0` 视为通用）
fn in_group(source_group: u64, group_id: u64) -> bool {
    source_group == 0 || source_group == group_id
}

/// 眼前这批话里被人说起的黑话/梗
fn recalled_jargon<'a>(
    jargon: &'a [JargonEntry],
    group_id: u64,
    text: &'a str,
) -> impl Iterator<Item = String> + 'a {
    jargon
        .iter()
        .filter(move |j| in_group(j.source_group, group_id))
        .filter(move |j| text.contains(j.content.as_str()))
        .map(|j| format!("- {}：{}", j.content, j.meaning))
}

/// 这个群最常用的几条说话路子
fn usual_habits(expressions: &[ExpressionHabit], group_id: u64) -> Vec<String> {
    let mut matched: Vec<&ExpressionHabit> = expressions
        .iter()
        .filter(|e| in_group(e.source_group, group_id))
        .collect();
    matched.sort_by_key(|e| Reverse(e.count));
    matched
        .into_iter()
        .take(HABIT_LIMIT)
        .map(|e| format!("- {} 时可以 {}", e.situation, e.style))
        .collect()
}

/// 读取侧入口：这个群的黑话/梗与说话路子（没有可递的内容时为 `None`）
pub(crate) fn context_block(group_id: u64, text: &str) -> Option<String> {
    let store = store::load_store();

    let jargon: Vec<String> = recalled_jargon(&store.jargon, group_id, text).collect();
    let habits = usual_habits(&store.expressions, group_id);

    let mut sections: Vec<String> = Vec::new();
    if !jargon.is_empty() {
        sections.push(format!(
            "# 眼前有人说的黑话/梗（你早就懂）\n{}",
            jargon.join("\n")
        ));
    }
    if !habits.is_empty() {
        sections.push(format!(
            "# 这个群的说话路子（想起来的时候自然用上，不用照抄）\n{}",
            habits.join("\n")
        ));
    }
    (!sections.is_empty()).then(|| sections.join("\n\n"))
}
