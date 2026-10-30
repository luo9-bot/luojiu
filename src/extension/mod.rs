//! Luo9 扩展中心：统一描述内置能力和外部 manifest 扩展。

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};
use tracing::{info, warn};

use crate::config;

mod runtime;

const STATE_FILE: &str = "extensions/state.json";
const MANIFEST_DIR: &str = "extensions";
const CONFIG_FILE: &str = "extensions/config.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExtensionKind {
    Builtin,
    External,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExtensionManifest {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub config_schema: Vec<ExtensionConfigField>,
    #[serde(default)]
    pub ui: Option<ExtensionUi>,
    #[serde(default)]
    pub kind: Option<ExtensionKind>,
    #[serde(default)]
    pub entrypoint: Option<String>,
    #[serde(default)]
    pub tools: Vec<ExtensionTool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExtensionTool {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub parameters: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExtensionConfigField {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub secret: bool,
    #[serde(default)]
    pub default: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExtensionUi {
    pub route: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub group: String,
    #[serde(default)]
    pub order: i32,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ExtensionRecord {
    #[serde(flatten)]
    pub manifest: ExtensionManifest,
    pub enabled: bool,
    pub healthy: bool,
    pub source: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ExtensionSnapshot {
    pub extensions: Vec<ExtensionRecord>,
    pub market: MarketSnapshot,
    pub configs: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct MarketSnapshot {
    pub configured: bool,
    pub source: String,
    pub entries: Vec<MarketEntry>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MarketEntry {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub download_url: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
}

#[derive(Default)]
struct ExtensionState {
    enabled: BTreeMap<String, bool>,
}
static STATE: OnceLock<RwLock<ExtensionState>> = OnceLock::new();

fn default_version() -> String {
    "0.1.0".to_string()
}
fn state() -> &'static RwLock<ExtensionState> {
    STATE.get_or_init(|| {
        RwLock::new(ExtensionState {
            enabled: read_state_file(),
        })
    })
}
fn state_path() -> PathBuf {
    config::data_dir().join(STATE_FILE)
}
fn read_state_file() -> BTreeMap<String, bool> {
    std::fs::read_to_string(state_path())
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}
fn persist_state(enabled: &BTreeMap<String, bool>) -> Result<(), String> {
    let raw = serde_json::to_string_pretty(enabled).map_err(|error| error.to_string())?;
    crate::util::atomic_write(&state_path(), raw).map_err(|error| error.to_string())
}
fn config_path() -> PathBuf {
    config::data_dir().join(CONFIG_FILE)
}
fn read_configs() -> BTreeMap<String, Value> {
    std::fs::read_to_string(config_path())
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}
fn persist_configs(configs: &BTreeMap<String, Value>) -> Result<(), String> {
    let raw = serde_json::to_string_pretty(configs).map_err(|error| error.to_string())?;
    crate::util::atomic_write(config_path(), raw).map_err(|error| error.to_string())
}
fn builtin(
    id: &str,
    name: &str,
    description: &str,
    group: &str,
    order: i32,
    capabilities: &[&str],
) -> ExtensionManifest {
    ExtensionManifest {
        id: id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        author: "luoy-oss".to_string(),
        tags: vec!["builtin".to_string()],
        capabilities: capabilities.iter().map(|v| (*v).to_string()).collect(),
        permissions: Vec::new(),
        requires: Vec::new(),
        config_schema: Vec::new(),
        ui: Some(ExtensionUi {
            route: format!("builtin:{id}"),
            icon: String::new(),
            group: group.to_string(),
            order,
        }),
        kind: Some(ExtensionKind::Builtin),
        entrypoint: None,
        tools: Vec::new(),
    }
}
pub(crate) fn builtin_manifests() -> Vec<ExtensionManifest> {
    vec![
        builtin(
            "core.chat",
            "对话核心",
            "消息感知、回复决策和上下文编排",
            "core",
            10,
            &["message.receive", "message.reply"],
        ),
        builtin(
            "core.vision",
            "视觉理解",
            "DeepSeek 原生图片理解和图片资源处理",
            "ai",
            20,
            &["vision.image", "vision.ocr"],
        ),
        builtin(
            "core.memory",
            "长期记忆",
            "用户记忆、工作记忆和语义检索",
            "data",
            30,
            &["memory.read", "memory.write"],
        ),
        builtin(
            "core.sticker",
            "表情包",
            "表情包注册、描述、筛选和选择",
            "media",
            40,
            &["media.image", "message.segment"],
        ),
        builtin(
            "core.security",
            "安全滤壳",
            "防注入、审计和风险状态",
            "security",
            50,
            &["security.filter", "security.audit"],
        ),
        builtin(
            "core.humanity",
            "人性化行为",
            "情绪、社交电池、节律和主动行为",
            "behavior",
            60,
            &["behavior.emotion", "behavior.proactive"],
        ),
        builtin(
            "core.admin",
            "管理控制台",
            "配置、数据、诊断和扩展管理",
            "system",
            70,
            &["admin.config", "admin.diagnostics"],
        ),
    ]
}
fn load_external_manifests() -> Vec<ExtensionManifest> {
    let Ok(entries) = std::fs::read_dir(config::data_dir().join(MANIFEST_DIR)) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path().join("manifest.json");
            let raw = std::fs::read_to_string(&path).ok()?;
            match serde_json::from_str::<ExtensionManifest>(&raw) {
                Ok(mut manifest) => {
                    manifest.kind = Some(ExtensionKind::External);
                    Some(manifest)
                }
                Err(error) => {
                    warn!(path = ?path, %error, "extension: manifest 解析失败");
                    None
                }
            }
        })
        .collect()
}
fn record(manifest: ExtensionManifest, enabled: bool, source: &str) -> ExtensionRecord {
    let external =
        manifest.kind.clone().unwrap_or(ExtensionKind::Builtin) == ExtensionKind::External;
    let (runtime_status, runtime_healthy) = if external {
        runtime::status(&manifest.id)
    } else {
        ("active", true)
    };
    ExtensionRecord {
        manifest,
        enabled,
        healthy: if external { runtime_healthy } else { true },
        source: source.to_string(),
        status: if external {
            runtime_status
        } else if enabled {
            "active"
        } else {
            "disabled"
        }
        .to_string(),
    }
}
pub(crate) fn initialize() {
    let _ = state();
    runtime::start_enabled_extensions(&load_external_manifests());
    info!(
        count = load_external_manifests().len(),
        "extension: registry initialized"
    );
}

pub(crate) struct ShutdownGuard;
impl Drop for ShutdownGuard {
    fn drop(&mut self) {
        runtime::shutdown_all();
    }
}
pub(crate) fn snapshot() -> ExtensionSnapshot {
    let enabled = state()
        .read()
        .map(|s| s.enabled.clone())
        .unwrap_or_default();
    let mut extensions = builtin_manifests()
        .into_iter()
        .map(|m| {
            let e = enabled.get(&m.id).copied().unwrap_or(true);
            record(m, e, "builtin")
        })
        .collect::<Vec<_>>();
    extensions.extend(load_external_manifests().into_iter().map(|m| {
        let e = enabled.get(&m.id).copied().unwrap_or(false);
        record(m, e, "data")
    }));
    extensions.sort_by_key(|e| e.manifest.ui.as_ref().map(|ui| ui.order).unwrap_or(999));
    ExtensionSnapshot {
        extensions,
        market: market_snapshot(),
        configs: read_configs(),
    }
}
pub(crate) fn enabled(id: &str) -> bool {
    state()
        .read()
        .map(|s| s.enabled.get(id).copied().unwrap_or(true))
        .unwrap_or(true)
}

pub(crate) fn tool_defs() -> Vec<crate::ai::Tool> {
    load_external_manifests()
        .into_iter()
        .filter(|manifest| enabled(&manifest.id) && runtime::is_loaded(&manifest.id))
        .flat_map(|manifest| {
            manifest.tools.into_iter().map(|tool| crate::ai::Tool {
                tool_type: "function".to_string(),
                function: crate::ai::FunctionDef {
                    name: tool.name,
                    description: tool.description,
                    parameters: tool.parameters,
                },
            })
        })
        .collect()
}

pub(crate) fn execute_tool(
    name: &str,
    args: &Value,
    group_id: u64,
    user_id: u64,
) -> Option<crate::ai::ToolOutcome> {
    let manifest = load_external_manifests().into_iter().find(|manifest| {
        enabled(&manifest.id)
            && manifest.tools.iter().any(|tool| tool.name == name)
            && runtime::is_loaded(&manifest.id)
    })?;
    let request = serde_json::json!({
        "tool": name,
        "arguments": args,
        "context": {"group_id": group_id, "user_id": user_id}
    });
    Some(match runtime::call(&manifest.id, &request) {
        Ok(result) => crate::ai::ToolOutcome::Continue(result),
        Err(error) => crate::ai::ToolOutcome::Continue(format!(
            "第三方扩展 {} 执行失败：{}",
            manifest.name, error
        )),
    })
}

pub(crate) fn dispatch_event(json: &str) {
    runtime::dispatch_event(json);
}
pub(crate) fn set_enabled(id: &str, value: bool) -> Result<ExtensionSnapshot, String> {
    if !builtin_manifests().iter().any(|m| m.id == id)
        && !load_external_manifests().iter().any(|m| m.id == id)
    {
        return Err(format!("未知扩展: {id}"));
    }
    let mut state = state()
        .write()
        .map_err(|_| "扩展状态锁已中毒".to_string())?;
    state.enabled.insert(id.to_string(), value);
    persist_state(&state.enabled)?;
    drop(state);
    if let Some(manifest) = load_external_manifests()
        .into_iter()
        .find(|manifest| manifest.id == id)
    {
        if value {
            runtime::load(&manifest)?;
        } else {
            runtime::unload(id)?;
        }
    }
    Ok(snapshot())
}

pub(crate) fn set_config(id: &str, value: Value) -> Result<ExtensionSnapshot, String> {
    if !snapshot()
        .extensions
        .iter()
        .any(|extension| extension.manifest.id == id)
    {
        return Err(format!("未知扩展: {id}"));
    }
    let mut configs = read_configs();
    configs.insert(id.to_string(), value);
    persist_configs(&configs)?;
    Ok(snapshot())
}
fn market_snapshot() -> MarketSnapshot {
    let source = std::env::var("LUO9_EXTENSION_MARKET_URL").unwrap_or_default();
    if source.trim().is_empty() {
        return MarketSnapshot {
            configured: false,
            source,
            entries: Vec::new(),
            error: None,
        };
    }
    match crate::util::get_text(&source) {
        Ok(raw) => match serde_json::from_str::<Vec<MarketEntry>>(&raw) {
            Ok(entries) => MarketSnapshot {
                configured: true,
                source,
                entries,
                error: None,
            },
            Err(error) => MarketSnapshot {
                configured: true,
                source,
                entries: Vec::new(),
                error: Some(format!("市场目录解析失败: {error}")),
            },
        },
        Err(error) => MarketSnapshot {
            configured: true,
            source,
            entries: Vec::new(),
            error: Some(format!("市场目录读取失败: {error}")),
        },
    }
}
#[cfg(test)]
fn parse_market_entry(value: &Value) -> Option<MarketEntry> {
    serde_json::from_value(value.clone()).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builtin_manifests_have_unique_ids_and_capabilities() {
        let mut ids = std::collections::BTreeSet::new();
        for manifest in builtin_manifests() {
            assert!(ids.insert(manifest.id));
            assert!(!manifest.capabilities.is_empty());
            assert!(manifest.ui.is_some());
        }
    }
    #[test]
    fn market_entries_accept_optional_distribution_fields() {
        let entry = parse_market_entry(
            &serde_json::json!({"id":"weather","name":"天气","version":"1.0.0","tags":["utility"]}),
        )
        .expect("market entry should parse");
        assert_eq!(entry.id, "weather");
        assert!(entry.download_url.is_none());
    }
}
