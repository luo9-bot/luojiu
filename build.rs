//! 构建脚本：保证 `frontend/dist/index.html` 与前端输入一致。
//!
//! `src/admin/ui.rs` 通过 `include_str!` 直接嵌入 Vite 的单文件 HTML。
//! 使用输入内容指纹，而不是文件修改时间判断是否需要构建：时间戳在切换
//! Git 分支、复制文件或恢复缓存后可能不可靠。
//!
//! 前端依赖由调用方安装（`cd frontend && npm ci`）；构建失败必须使 Cargo 构建失败，
//! 不能静默沿用旧页面。

use std::path::{Path, PathBuf};

const DIST_HTML: &str = "frontend/dist/index.html";
const FRONTEND_SRC: &str = "frontend/src";
const FRONTEND_PUBLIC: &str = "frontend/public";
const BUILD_STAMP: &str = "frontend/.last-build";
const FRONTEND_CONFIGS: [&str; 4] = [
    "frontend/index.html",
    "frontend/package.json",
    "frontend/package-lock.json",
    "frontend/vite.config.js",
];

fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// Stable FNV-1a fingerprint over all frontend build inputs.
fn frontend_fingerprint() -> Result<String, String> {
    let mut files = Vec::new();
    collect_files(Path::new(FRONTEND_SRC), &mut files);
    collect_files(Path::new(FRONTEND_PUBLIC), &mut files);
    files.extend(
        FRONTEND_CONFIGS
            .iter()
            .map(PathBuf::from)
            .filter(|path| path.is_file()),
    );
    files.sort();

    let mut hash = 0xcbf29ce484222325_u64;
    for path in files {
        let normalized_path = path.to_string_lossy().replace('\\\\', "/");
        for byte in normalized_path
            .as_bytes()
            .iter()
            .copied()
            .chain(std::iter::once(0))
            .chain(
                std::fs::read(&path)
                    .map_err(|error| format!("无法读取前端输入 {}：{error}", path.display()))?,
            )
            .chain(std::iter::once(0xff))
        {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    Ok(format!("v1:{hash:016x}"))
}

fn build_frontend() -> Result<(), String> {
    // Windows 上 npm 是 .cmd，需要经 cmd 执行。
    let mut command = if cfg!(target_os = "windows") {
        let mut cmd = std::process::Command::new("cmd");
        cmd.args(["/c", "npm", "run", "build"]);
        cmd
    } else {
        let mut cmd = std::process::Command::new("npm");
        cmd.args(["run", "build"]);
        cmd
    };

    let status = command
        .current_dir("frontend")
        .status()
        .map_err(|error| format!("无法执行 `npm run build`：{error}"))?;
    if !status.success() {
        return Err(format!("`npm run build` 失败（{status}）"));
    }
    Ok(())
}

fn main() {
    // Cargo 监视输入及产物：输入更新时重建，产物被删除时也触发脚本。
    println!("cargo:rerun-if-changed={FRONTEND_SRC}");
    println!("cargo:rerun-if-changed={FRONTEND_PUBLIC}");
    for config in FRONTEND_CONFIGS {
        println!("cargo:rerun-if-changed={config}");
    }
    println!("cargo:rerun-if-changed={DIST_HTML}");
    println!("cargo:rerun-if-changed=build.rs");

    if !Path::new(FRONTEND_SRC).exists() {
        // 允许仅分发 Rust 源码和已构建的 HTML 的场景。
        if !Path::new(DIST_HTML).is_file() {
            panic!(
                "{FRONTEND_SRC} 和 {DIST_HTML} 均不存在，无法嵌入管理页面。\n\\
                 请检出前端源码，或先在 frontend 目录运行：npm ci && npm run build"
            );
        }
        return;
    }

    let fingerprint = frontend_fingerprint()
        .unwrap_or_else(|error| panic!("计算前端输入指纹失败：{error}"));
    let stamp_matches = std::fs::read_to_string(BUILD_STAMP)
        .map(|stamp| stamp.trim() == fingerprint)
        .unwrap_or(false);
    let dist_exists = Path::new(DIST_HTML).is_file();

    if !dist_exists || !stamp_matches {
        println!("cargo:warning=前端输入发生变化或 dist 缺失，正在重建 WebUI ...");
        if let Err(error) = build_frontend() {
            panic!(
                "前端构建失败：{error}\n\\
                 为避免嵌入旧页面，已中止 Cargo 构建。请确认 Node.js/npm 可用，然后运行：\n\\
                 cd frontend && npm ci && npm run build"
            );
        }
        if !Path::new(DIST_HTML).is_file() {
            panic!(
                "前端构建命令成功退出，但 {DIST_HTML} 仍不存在；无法嵌入管理页面。"
            );
        }
        std::fs::write(BUILD_STAMP, &fingerprint)
            .unwrap_or_else(|error| panic!("无法写入前端构建指纹 {BUILD_STAMP}：{error}"));
    }
}
