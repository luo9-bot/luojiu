//! 动态库扩展运行时。
//!
//! ABI 是故意保持很小的 C ABI：扩展只需要导出 init/shutdown/on_event，
//! 宿主负责库生命周期和配置。所有入口均为可选，缺失符号会在加载时报告。

use super::{ExtensionKind, ExtensionManifest};
use crate::config;
use libloading::Library;
use std::ffi::{CStr, CString, c_char};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use tracing::{info, warn};

pub(super) const ABI_VERSION: u32 = 1;
const INIT_SYMBOL: &[u8] = b"luo9_extension_init\0";
const SHUTDOWN_SYMBOL: &[u8] = b"luo9_extension_shutdown\0";
const EVENT_SYMBOL: &[u8] = b"luo9_extension_on_event\0";

#[repr(C)]
pub(super) struct HostApi {
    pub abi_version: u32,
    pub log: Option<extern "C" fn(*const c_char)>,
    pub get_config: Option<extern "C" fn(*const c_char) -> *mut c_char>,
    pub free_string: Option<extern "C" fn(*mut c_char)>,
}

type Init = unsafe extern "C" fn(*const HostApi) -> i32;
type Shutdown = unsafe extern "C" fn();
type OnEvent = unsafe extern "C" fn(*const c_char) -> i32;

struct LoadedExtension {
    id: String,
    library: Library,
    shutdown: Shutdown,
    on_event: Option<OnEvent>,
}

static LOADED: OnceLock<Mutex<Vec<LoadedExtension>>> = OnceLock::new();

fn loaded() -> &'static Mutex<Vec<LoadedExtension>> {
    LOADED.get_or_init(|| Mutex::new(Vec::new()))
}

extern "C" fn host_log(message: *const c_char) {
    if message.is_null() {
        return;
    }
    let message = unsafe { CStr::from_ptr(message) }.to_string_lossy();
    info!(target: "extension", "{message}");
}

extern "C" fn host_get_config(id: *const c_char) -> *mut c_char {
    if id.is_null() {
        return std::ptr::null_mut();
    }
    let id = unsafe { CStr::from_ptr(id) }.to_string_lossy();
    let snapshot = super::snapshot();
    let value = snapshot
        .configs
        .get(id.as_ref())
        .cloned()
        .unwrap_or(serde_json::Value::Object(Default::default()));
    CString::new(value.to_string())
        .map(CString::into_raw)
        .unwrap_or(std::ptr::null_mut())
}

extern "C" fn host_free_string(value: *mut c_char) {
    if !value.is_null() {
        unsafe {
            drop(CString::from_raw(value));
        }
    }
}

fn host_api() -> HostApi {
    HostApi {
        abi_version: ABI_VERSION,
        log: Some(host_log),
        get_config: Some(host_get_config),
        free_string: Some(host_free_string),
    }
}

fn extension_dir(id: &str) -> PathBuf {
    config::data_dir().join("extensions").join(id)
}

fn library_path(manifest: &ExtensionManifest) -> Result<PathBuf, String> {
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
    let canonical_root = root
        .canonicalize()
        .map_err(|e| format!("扩展目录不可用: {e}"))?;
    let canonical_path = path
        .canonicalize()
        .map_err(|e| format!("动态库不存在: {e}"))?;
    if !canonical_path.starts_with(&canonical_root) {
        return Err("entrypoint 越过扩展目录".to_string());
    }
    Ok(canonical_path)
}

pub(crate) fn start_enabled_extensions(manifests: &[ExtensionManifest]) {
    for manifest in manifests {
        if manifest.kind == Some(ExtensionKind::External) && super::enabled(&manifest.id) {
            if let Err(error) = load(manifest) {
                warn!(id = %manifest.id, %error, "extension: 动态库加载失败");
            }
        }
    }
}

pub(crate) fn load(manifest: &ExtensionManifest) -> Result<(), String> {
    if manifest.kind != Some(ExtensionKind::External) {
        return Ok(());
    }
    if is_loaded(&manifest.id) {
        return Ok(());
    }
    let path = library_path(manifest)?;
    let library = unsafe { Library::new(&path) }.map_err(|e| format!("加载动态库失败: {e}"))?;
    let init: Init = unsafe {
        *library
            .get(INIT_SYMBOL)
            .map_err(|e| format!("缺少 luo9_extension_init: {e}"))?
    };
    let shutdown: Shutdown = unsafe {
        *library
            .get(SHUTDOWN_SYMBOL)
            .map_err(|e| format!("缺少 luo9_extension_shutdown: {e}"))?
    };
    let on_event = unsafe {
        library
            .get::<OnEvent>(EVENT_SYMBOL)
            .ok()
            .map(|symbol| *symbol)
    };
    let host = host_api();
    let result = unsafe { init(&host) };
    if result != 0 {
        return Err(format!("扩展初始化失败，返回码 {result}"));
    }
    loaded()
        .lock()
        .map_err(|_| "动态库运行时锁已中毒".to_string())?
        .push(LoadedExtension {
            id: manifest.id.clone(),
            library,
            shutdown,
            on_event,
        });
    info!(id = %manifest.id, path = ?path, "extension: 动态库已加载");
    Ok(())
}

pub(crate) fn unload(id: &str) -> Result<(), String> {
    let mut extensions = loaded()
        .lock()
        .map_err(|_| "动态库运行时锁已中毒".to_string())?;
    let Some(index) = extensions.iter().position(|extension| extension.id == id) else {
        return Ok(());
    };
    let extension = extensions.remove(index);
    unsafe {
        (extension.shutdown)();
    }
    drop(extension.library);
    info!(%id, "extension: 动态库已卸载");
    Ok(())
}

pub(crate) fn is_loaded(id: &str) -> bool {
    loaded()
        .lock()
        .map(|extensions| extensions.iter().any(|extension| extension.id == id))
        .unwrap_or(false)
}

pub(crate) fn dispatch_event(json: &str) {
    let Ok(payload) = CString::new(json) else {
        return;
    };
    let Ok(extensions) = loaded().lock() else {
        return;
    };
    for extension in extensions.iter() {
        if let Some(on_event) = extension.on_event {
            let result = unsafe { on_event(payload.as_ptr()) };
            if result != 0 {
                warn!(id = %extension.id, result, "extension: on_event 返回错误");
            }
        }
    }
}

pub(crate) fn status(id: &str) -> (&'static str, bool) {
    if is_loaded(id) {
        ("active", true)
    } else {
        ("manifest_only", false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_path_traversal_before_loading() {
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
            entrypoint: Some("../evil.dll".into()),
        };
        assert!(library_path(&manifest).is_err());
    }
}
