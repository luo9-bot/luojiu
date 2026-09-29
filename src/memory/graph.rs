//! 知识图谱模块
//!
//! 实体集合存储，支持：
//! - 实体存储（名称小写去重）
//! - Aho-Corasick 实体提取

use std::collections::HashSet;
use tracing::debug;

/// 知识图谱（实体集合）
#[derive(Debug, Clone, Default)]
pub(crate) struct KnowledgeGraph {
    entities: HashSet<String>,
}

impl KnowledgeGraph {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// 添加实体（实体以名称小写为 key）
    pub(crate) fn add_entity(&mut self, name: &str) {
        self.entities.insert(name.to_lowercase());
    }
}

/// Aho-Corasick 构建的实体匹配器，用于从文本中快速匹配已知实体
pub(crate) struct EntityMatcher {
    /// 构建失败时为 `None`（例如模式集合为空）：那时"匹配不到实体"，
    /// 而不是让调用方 panic
    ac: Option<aho_corasick::AhoCorasick>,
    entities: Vec<String>,
}

impl EntityMatcher {
    pub(crate) fn build(entities: &[String]) -> Self {
        let ac = aho_corasick::AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .build(entities)
            .ok();
        if ac.is_none() {
            tracing::debug!("graph: 实体匹配器构建失败，本次不做实体匹配");
        }
        Self {
            ac,
            entities: entities.to_vec(),
        }
    }

    /// 在文本中匹配已知实体
    pub(crate) fn match_entities(&self, text: &str) -> Vec<String> {
        let Some(ac) = &self.ac else {
            return Vec::new();
        };
        let mut found: Vec<String> = Vec::new();
        for m in ac.find_iter(text) {
            if let Some(name) = self.entities.get(m.pattern().as_usize())
                && !found.contains(name)
            {
                found.push(name.clone());
            }
        }
        found
    }
}

/// 从记忆中提取实体和关系（规则 + 模式匹配）
pub(crate) fn extract_entities_from_text(text: &str) -> Vec<(String, String, String)> {
    let mut triples = Vec::new();

    // 中文关系模式
    let patterns = [
        ("是", "is"),
        ("叫", "is"),
        ("有", "has"),
        ("喜欢", "likes"),
        ("不喜欢", "dislikes"),
        ("讨厌", "hates"),
        ("属于", "belongs_to"),
        ("包含", "contains"),
        ("位于", "located_at"),
        ("来自", "from"),
        ("使用", "uses"),
        ("知道", "knows"),
        ("住在", "lives_in"),
        ("在", "at"),
        ("的", "possesses"),
    ];

    for (keyword, predicate) in &patterns {
        let mut start = 0;
        while let Some(pos) = text[start..].find(keyword) {
            let abs_pos = start + pos;
            let before = text[..abs_pos].trim();
            let after = text[abs_pos + keyword.len()..].trim();

            let subject = extract_last_word(before);
            let object = extract_first_word(after);

            if !subject.is_empty() && !object.is_empty() {
                triples.push((subject, predicate.to_string(), object));
            }
            // 推进到下一个 char 边界，避免多字节 UTF-8 字符中间切片
            start = abs_pos + keyword.len();
            while start < text.len() && !text.is_char_boundary(start) {
                start += 1;
            }
        }
    }

    triples
}

fn extract_last_word(text: &str) -> String {
    text.split_whitespace()
        .last()
        .unwrap_or("")
        .trim_matches(|c: char| c.is_ascii_punctuation() || c == '，' || c == '。')
        .to_string()
}

fn extract_first_word(text: &str) -> String {
    text.split_whitespace()
        .next()
        .unwrap_or("")
        .trim_matches(|c: char| c.is_ascii_punctuation() || c == '，' || c == '。')
        .to_string()
}

// ── 全局知识图谱 ────────────────────────────────────────────────

use crate::util::MutexExt;
use std::sync::Mutex;

static GRAPH: Mutex<Option<KnowledgeGraph>> = Mutex::new(None);

/// 初始化知识图谱
pub(crate) fn init() {
    let mut guard = GRAPH.lock_recover();
    *guard = Some(KnowledgeGraph::new());
}

/// 获取知识图谱引用
///
/// 未初始化时惰性建一个空图：知识图谱是纯内存的加速结构，
/// "还没初始化"不该让检索路径 panic（原先这里是 `expect`）。
pub(crate) fn with_graph<F, R>(f: F) -> R
where
    F: FnOnce(&KnowledgeGraph) -> R,
{
    let mut guard = GRAPH.lock_recover();
    f(guard.get_or_insert_with(KnowledgeGraph::new))
}

/// 获取可变知识图谱引用（同 [`with_graph`]，未初始化时惰性建空图）
pub(crate) fn with_graph_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut KnowledgeGraph) -> R,
{
    let mut guard = GRAPH.lock_recover();
    f(guard.get_or_insert_with(KnowledgeGraph::new))
}

/// 从记忆中提取实体并更新实体集合
pub(crate) fn update_graph_from_memory(user_id: u64, content: &str) {
    let triples = extract_entities_from_text(content);
    let count = triples.len();
    if triples.is_empty() {
        return;
    }

    with_graph_mut(|graph| {
        for (subject, _predicate, object) in triples {
            graph.add_entity(&subject);
            graph.add_entity(&object);
        }
    });

    debug!(user_id, triples = count, "graph: updated from memory");
}

/// 构建全局实体匹配器
pub(crate) fn build_entity_matcher() -> EntityMatcher {
    with_graph(|graph| {
        let entities: Vec<String> = graph.entities.iter().cloned().collect();
        EntityMatcher::build(&entities)
    })
}
