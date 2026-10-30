use base64::{Engine, engine::general_purpose::STANDARD};
use luo9_sdk::bus::Bus;
use luo9_sdk::napcat::ApiClient;
use luo9_sdk::payload::BusPayload;
use regex::Regex;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::ffi::CString;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const MAX_SECONDS: f64 = 15.0;
const MAX_FRAMES: usize = 8;

#[derive(Clone)]
struct Tracker {
    snapshot: Value,
    group_id: u64,
    user_id: u64,
}

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_default()
}

fn command_output(mut command: Command) -> Result<Vec<u8>, String> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|error| format!("启动视频工具失败：{error}"))?;
    let started = Instant::now();
    loop {
        match child.try_wait().map_err(|error| error.to_string())? {
            Some(status) => {
                let output = child
                    .wait_with_output()
                    .map_err(|error| error.to_string())?;
                if !status.success() {
                    return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
                }
                return Ok(output.stdout);
            }
            None if started.elapsed() >= Duration::from_secs(12) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("视频处理超时".into());
            }
            None => thread::sleep(Duration::from_millis(40)),
        }
    }
}

fn source_from_cq(source: &str) -> Result<String, String> {
    if !source.starts_with("[CQ:video,") {
        return Ok(source.to_string());
    }
    let body = source
        .strip_prefix("[CQ:video,")
        .and_then(|value| value.split_once(']').map(|(body, _)| body))
        .ok_or("无法解析 CQ 视频资源")?;
    let fields = body
        .split(',')
        .filter_map(|field| field.split_once('='))
        .collect::<BTreeMap<_, _>>();
    if let Some(url) = fields.get("url") {
        return Ok(luo9_sdk::segment::unescape(url).to_string());
    }
    let file = fields.get("file").ok_or("CQ 视频缺少 url/file")?;
    let file = luo9_sdk::segment::unescape(file);
    let data = ApiClient::new(Duration::from_secs(5))
        .get_file(&file)
        .map_err(|error| format!("NapCat get_file 失败：{error}"))?;
    data.get("url")
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| "NapCat 未返回视频 URL".into())
}

fn temp_dir() -> Result<PathBuf, String> {
    let root = std::env::temp_dir().join(format!("luo9-video-{}", std::process::id()));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(root)
}

fn analyze_video(source: &str, prompt: &str) -> Result<String, String> {
    let source = source_from_cq(source)?;
    let mut probe = Command::new("ffprobe");
    probe.args([
        "-v",
        "error",
        "-show_entries",
        "format=duration",
        "-of",
        "default=noprint_wrappers=1:nokey=1",
        &source,
    ]);
    let duration_text = String::from_utf8_lossy(&command_output(probe)?).to_string();
    let duration = duration_text
        .lines()
        .find_map(|line| line.trim().parse::<f64>().ok())
        .ok_or("无法读取视频时长")?;
    if !duration.is_finite() || duration <= 0.0 || duration > MAX_SECONDS {
        return Err(format!("视频时长 {:.1}s，超过 15s 限制", duration));
    }
    let dir = temp_dir()?;
    let pattern = dir.join("frame-%03d.jpg");
    let mut ffmpeg = Command::new("ffmpeg");
    ffmpeg.args([
        "-hide_banner",
        "-loglevel",
        "error",
        "-i",
        &source,
        "-t",
        "15",
        "-vf",
        "fps=1",
        "-frames:v",
        "8",
        "-q:v",
        "4",
        "-y",
    ]);
    ffmpeg.arg(&pattern);
    let result = (|| {
        command_output(ffmpeg)?;
        let mut frames = std::fs::read_dir(&dir)
            .map_err(|error| error.to_string())?
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("jpg"))
            .collect::<Vec<_>>();
        frames.sort();
        frames.truncate(MAX_FRAMES);
        if frames.is_empty() {
            return Err("视频没有可用关键帧".into());
        }
        call_vlm(&frames, prompt)
    })();
    let _ = std::fs::remove_dir_all(&dir);
    result
}

fn call_vlm(frames: &[PathBuf], prompt: &str) -> Result<String, String> {
    let base_url = env("LUO9_AI_BASE_URL").trim_end_matches('/').to_string();
    let api_key = env("LUO9_AI_API_KEY");
    let model = env("LUO9_AI_MODEL");
    let text = if prompt.trim().is_empty() {
        "按时间顺序概括视频关键变化，不要猜测看不清的内容。"
    } else {
        prompt
    };
    let mut content = vec![json!({"type":"text","text": text})];
    for frame in frames {
        let bytes = std::fs::read(frame).map_err(|error| error.to_string())?;
        content.push(json!({"type":"image_url","image_url":{"url":format!("data:image/jpeg;base64,{}", STANDARD.encode(bytes)),"detail":"low"}}));
    }
    let body =
        json!({"model":model,"messages":[{"role":"user","content":content}],"max_tokens":800});
    let mut response = ureq::Agent::new_with_config(
        ureq::config::Config::builder()
            .timeout_global(Some(Duration::from_secs(20)))
            .build(),
    )
    .post(&format!("{base_url}/chat/completions"))
    .header("Authorization", &format!("Bearer {api_key}"))
    .header("Content-Type", "application/json")
    .send(body.to_string().as_bytes())
    .map_err(|error| format!("视觉 API 请求失败：{error}"))?;
    let raw = response
        .body_mut()
        .read_to_string()
        .map_err(|error| error.to_string())?;
    let value: Value = serde_json::from_str(&raw).map_err(|error| error.to_string())?;
    value
        .pointer("choices.0.message.content")
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| "视觉 API 没有返回文本".into())
}

fn html_value(html: &str, pattern: &str) -> String {
    Regex::new(pattern)
        .ok()
        .and_then(|regex| regex.captures(html))
        .and_then(|caps| caps.get(1))
        .map(|value| value.as_str().to_string())
        .unwrap_or_default()
}

fn inspect_live(url: &str) -> Result<Value, String> {
    if !url.contains("bilibili.com") {
        return Err("只支持 Bilibili 直播页".into());
    }
    let html = ureq::get(url)
        .call()
        .map_err(|error| error.to_string())?
        .into_body()
        .read_to_string()
        .map_err(|error| error.to_string())?;
    let room_id = html_value(&html, r#""room_id"\s*:\s*"?(\d+)"?"#);
    let title = html_value(&html, r#""room_name"\s*:\s*"([^"]*)""#);
    let live = html.contains("\"live_status\":1") || html.contains("直播中");
    Ok(json!({"url":url,"room_id":room_id,"title":title,"live":live}))
}

fn send(group_id: u64, user_id: u64, text: &str) {
    if group_id > 0 {
        let _ = luo9_sdk::Bot::send_group_msg(group_id, CString::new(text).unwrap_or_default());
    } else {
        let _ = luo9_sdk::Bot::send_private_msg(user_id, CString::new(text).unwrap_or_default());
    }
}

fn handle_tool(
    payload: &Value,
    trackers: &Arc<Mutex<BTreeMap<String, Tracker>>>,
) -> Result<String, String> {
    let tool = payload.get("tool").and_then(Value::as_str).unwrap_or("");
    let args = payload.get("arguments").cloned().unwrap_or_default();
    let group_id = payload
        .pointer("context.group_id")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let user_id = payload
        .pointer("context.user_id")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    match tool {
        "video_analyze" => analyze_video(
            args.get("source").and_then(Value::as_str).unwrap_or(""),
            args.get("prompt").and_then(Value::as_str).unwrap_or(""),
        ),
        "bilibili_live" => {
            let action = args
                .get("action")
                .and_then(Value::as_str)
                .unwrap_or("inspect");
            let url = args.get("url").and_then(Value::as_str).unwrap_or("");
            match action {
                "inspect" => Ok(inspect_live(url)?.to_string()),
                "track" => {
                    let snapshot = inspect_live(url)?;
                    let room = snapshot
                        .get("room_id")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    trackers.lock().map_err(|_| "跟踪锁已中毒")?.insert(
                        room.clone(),
                        Tracker {
                            snapshot,
                            group_id,
                            user_id,
                        },
                    );
                    Ok(format!("已开始跟踪直播间 {room}"))
                }
                "untrack" => {
                    trackers.lock().map_err(|_| "跟踪锁已中毒")?.remove(url);
                    Ok(format!("已停止跟踪 {url}"))
                }
                "poll" => Ok("后台跟踪正在运行".into()),
                _ => Err("action 必须是 inspect、track、poll 或 untrack".into()),
            }
        }
        _ => Err("未知视频工具".into()),
    }
}

fn handle_event(event: &str) {
    let Some(BusPayload::Message(message)) = BusPayload::parse(event) else {
        return;
    };
    let Some(start) = message.message.find("[CQ:video,") else {
        return;
    };
    let Some(end) = message.message[start..]
        .find(']')
        .map(|index| start + index + 1)
    else {
        return;
    };
    let source = &message.message[start..end];
    match analyze_video(source, "") {
        Ok(result) => send(
            message.group_id.unwrap_or(0),
            message.user_id,
            &format!("视频内容（阅后即焚）：\n{result}"),
        ),
        Err(error) => send(
            message.group_id.unwrap_or(0),
            message.user_id,
            &format!("视频暂时读不了：{error}"),
        ),
    }
}

fn main() {
    let _ = Bus::init();
    let command_topic = env("LUO9_EXTENSION_COMMAND_TOPIC");
    let event_topic = env("LUO9_EXTENSION_EVENT_TOPIC");
    let response_topic = env("LUO9_EXTENSION_RESPONSE_TOPIC");
    let command_sub = Bus::topic(&command_topic)
        .subscribe()
        .expect("command subscription");
    let event_sub = Bus::topic(&event_topic)
        .subscribe()
        .expect("event subscription");
    let trackers: Arc<Mutex<BTreeMap<String, Tracker>>> = Arc::new(Mutex::new(BTreeMap::new()));
    let poll_trackers = Arc::clone(&trackers);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60));
            let items = poll_trackers
                .lock()
                .map(|value| value.clone())
                .unwrap_or_default();
            for (room, tracker) in items {
                let Some(url) = tracker.snapshot.get("url").and_then(Value::as_str) else {
                    continue;
                };
                if let Ok(snapshot) = inspect_live(url) {
                    if snapshot != tracker.snapshot {
                        send(
                            tracker.group_id,
                            tracker.user_id,
                            &format!("直播动态：{}", snapshot),
                        );
                        if let Ok(mut current) = poll_trackers.lock() {
                            current.insert(
                                room,
                                Tracker {
                                    snapshot,
                                    ..tracker
                                },
                            );
                        }
                    }
                }
            }
        }
    });
    loop {
        if let Some(raw) = Bus::topic(&command_topic).pop(command_sub) {
            let request: Value = serde_json::from_str(&raw).unwrap_or_default();
            let request_id = request
                .get("request_id")
                .and_then(Value::as_str)
                .unwrap_or("");
            let result = handle_tool(request.get("payload").unwrap_or(&Value::Null), &trackers);
            let response = match result {
                Ok(result) => json!({"request_id":request_id,"ok":true,"result":result}),
                Err(error) => json!({"request_id":request_id,"ok":false,"error":error}),
            };
            let _ = Bus::topic(&response_topic).publish(&response.to_string());
        }
        if let Some(raw) = Bus::topic(&event_topic).pop(event_sub) {
            handle_event(&raw);
        }
        thread::sleep(Duration::from_millis(5));
    }
}
