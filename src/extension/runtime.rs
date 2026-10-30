//! 隔离式扩展运行时：扩展作为独立进程，通过 Luo9 Bus 通信。

use super::{ExtensionKind, ExtensionManifest};
use crate::config;
use luo9_sdk::bus::Bus;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{
    Mutex, OnceLock,
    atomic::{AtomicU64, Ordering},
};
use std::time::{Duration, Instant};
use tracing::{info, warn};

pub(super) const BUS_ABI_VERSION: u32 = 1;

struct RunningExtension {
    id: String,
    event_topic: String,
    child: Child,
}
static RUNNING: OnceLock<Mutex<Vec<RunningExtension>>> = OnceLock::new();
static REQUEST_SEQUENCE: AtomicU64 = AtomicU64::new(1);
fn running() -> &'static Mutex<Vec<RunningExtension>> {
    RUNNING.get_or_init(|| Mutex::new(Vec::new()))
}
fn extension_dir(id: &str) -> PathBuf {
    config::data_dir().join("extensions").join(id)
}
pub(crate) fn event_topic(id: &str) -> String {
    format!("luo9_extension.{id}.event")
}
pub(crate) fn command_topic(id: &str) -> String {
    format!("luo9_extension.{id}.command")
}
pub(crate) fn response_topic(id: &str) -> String {
    format!("luo9_extension.{id}.response")
}

pub(crate) fn call(id: &str, payload: &serde_json::Value) -> Result<String, String> {
    if !is_loaded(id) {
        return Err("扩展进程未运行".to_string());
    }
    let response_topic = response_topic(id);
    let subscriber = Bus::topic(&response_topic)
        .subscribe_independent()
        .map_err(|error| format!("订阅扩展响应失败：{error:?}"))?;
    let request_id = format!(
        "{}-{}",
        std::process::id(),
        REQUEST_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    );
    let command = serde_json::json!({"request_id": request_id, "payload": payload});
    let publish_result = Bus::topic(&command_topic(id))
        .publish(&command.to_string())
        .map_err(|error| format!("发送扩展命令失败：{error:?}"));
    if let Err(error) = publish_result {
        let _ = Bus::topic(&response_topic).unsubscribe(subscriber);
        return Err(error);
    }

    let started = Instant::now();
    let result = loop {
        if started.elapsed() >= Duration::from_secs(20) {
            break Err("扩展响应超时".to_string());
        }
        if let Some(raw) = Bus::topic(&response_topic).pop(subscriber) {
            let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) else {
                continue;
            };
            if value.get("request_id").and_then(|v| v.as_str()) != Some(&request_id) {
                continue;
            }
            if value.get("ok").and_then(|v| v.as_bool()) == Some(true) {
                break Ok(value
                    .get("result")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string());
            }
            break Err(value
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("扩展返回未知错误")
                .to_string());
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    let _ = Bus::topic(&response_topic).unsubscribe(subscriber);
    result
}

fn executable_path(manifest: &ExtensionManifest) -> Result<PathBuf, String> {
    let entrypoint = manifest
        .entrypoint
        .as_deref()
        .ok_or_else(|| "manifest 缺少 entrypoint".to_string())?;
    let relative = Path::new(entrypoint);
    if relative.is_absolute()
        || relative
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err("entrypoint 必须是扩展目录内的相对路径".to_string());
    }
    let root = extension_dir(&manifest.id);
    let path = root.join(relative);
    let fallback_path = if !path.is_file()
        && path.extension().and_then(|extension| extension.to_str()) == Some("exe")
    {
        path.with_extension("")
    } else {
        path.clone()
    };
    let canonical_root = root
        .canonicalize()
        .map_err(|error| format!("扩展目录不可用: {error}"))?;
    let canonical_path = fallback_path
        .canonicalize()
        .map_err(|error| format!("扩展程序不存在: {error}"))?;
    if !canonical_path.starts_with(&canonical_root) {
        return Err("entrypoint 越过扩展目录".to_string());
    }
    Ok(canonical_path)
}

pub(crate) fn start_enabled_extensions(manifests: &[ExtensionManifest]) {
    for manifest in manifests {
        if manifest.kind == Some(ExtensionKind::External) && super::enabled(&manifest.id) {
            if let Err(error) = load(manifest) {
                warn!(id = %manifest.id, %error, "extension: 隔离进程启动失败");
            }
        }
    }
}

pub(crate) fn load(manifest: &ExtensionManifest) -> Result<(), String> {
    if manifest.kind != Some(ExtensionKind::External) || is_loaded(&manifest.id) {
        return Ok(());
    }
    let path = executable_path(manifest)?;
    let event_topic = event_topic(&manifest.id);
    let config_path = config::data_dir().join("extensions").join("config.json");
    let child = Command::new(&path)
        .current_dir(extension_dir(&manifest.id))
        .env("LUO9_EXTENSION_ID", &manifest.id)
        .env("LUO9_EXTENSION_ABI", BUS_ABI_VERSION.to_string())
        .env("LUO9_EXTENSION_EVENT_TOPIC", &event_topic)
        .env("LUO9_EXTENSION_COMMAND_TOPIC", command_topic(&manifest.id))
        .env(
            "LUO9_EXTENSION_RESPONSE_TOPIC",
            response_topic(&manifest.id),
        )
        .env("LUO9_EXTENSION_CONFIG", config_path)
        .env("LUO9_AI_API_KEY", config::get().api_key)
        .env("LUO9_AI_BASE_URL", config::get().base_url)
        .env("LUO9_AI_MODEL", config::get().model)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("启动扩展进程失败: {error}"))?;
    let pid = child.id();
    running()
        .lock()
        .map_err(|_| "扩展运行时锁已中毒".to_string())?
        .push(RunningExtension {
            id: manifest.id.clone(),
            event_topic,
            child,
        });
    info!(id = %manifest.id, pid, "extension: 隔离进程已启动");
    Ok(())
}

pub(crate) fn unload(id: &str) -> Result<(), String> {
    let mut extensions = running()
        .lock()
        .map_err(|_| "扩展运行时锁已中毒".to_string())?;
    let Some(index) = extensions.iter().position(|extension| extension.id == id) else {
        return Ok(());
    };
    let mut extension = extensions.remove(index);
    let _ = extension.child.kill();
    let _ = extension.child.wait();
    info!(%id, "extension: 隔离进程已停止");
    Ok(())
}

pub(crate) fn is_loaded(id: &str) -> bool {
    let Ok(mut extensions) = running().lock() else {
        return false;
    };
    let mut index = 0;
    while index < extensions.len() {
        match extensions[index].child.try_wait() {
            Ok(Some(status)) => {
                warn!(id = %extensions[index].id, ?status, "extension: 进程已退出");
                extensions.remove(index);
            }
            Ok(None) => index += 1,
            Err(error) => {
                warn!(id = %extensions[index].id, %error, "extension: 进程状态检查失败");
                index += 1;
            }
        }
    }
    extensions.iter().any(|extension| extension.id == id)
}

pub(crate) fn dispatch_event(json: &str) {
    let Ok(extensions) = running().lock() else {
        return;
    };
    for extension in extensions.iter() {
        if let Err(error) = Bus::topic(&extension.event_topic).publish(json) {
            warn!(id = %extension.id, ?error, "extension: 事件发布失败");
        }
    }
}
pub(crate) fn status(id: &str) -> (&'static str, bool) {
    if is_loaded(id) {
        ("active", true)
    } else {
        ("stopped", false)
    }
}
pub(crate) fn shutdown_all() {
    if let Ok(mut extensions) = running().lock() {
        for mut extension in extensions.drain(..) {
            let _ = extension.child.kill();
            let _ = extension.child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn topics_are_stable_and_scoped_by_extension_id() {
        assert_eq!(event_topic("weather"), "luo9_extension.weather.event");
        assert_eq!(command_topic("weather"), "luo9_extension.weather.command");
        assert_eq!(response_topic("weather"), "luo9_extension.weather.response");
    }
    #[test]
    fn rejects_path_traversal_before_spawning() {
        let manifest = ExtensionManifest {
            id: "demo".into(),
            name: "demo".into(),
            description: String::new(),
            version: "1".into(),
            author: String::new(),
            tags: Vec::new(),
            capabilities: Vec::new(),
            permissions: Vec::new(),
            requires: Vec::new(),
            config_schema: Vec::new(),
            ui: None,
            kind: Some(ExtensionKind::External),
            entrypoint: Some("../evil.exe".into()),
            tools: Vec::new(),
        };
        assert!(executable_path(&manifest).is_err());
    }
}
