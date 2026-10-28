use crate::{
    anti_injection,
    config::{self, VisionApiFormat, VisionConfig, VisionDetail},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use tracing::{debug, info};

/// 提取 CQ 图片资源，优先 URL，缺少 URL 时保留 NapCat file 标识供查询。
pub(crate) fn extract_image_urls(message: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let mut remaining = message;
    while let Some(start) = remaining.find("[CQ:image,") {
        let after = &remaining[start + 10..];
        let Some(end) = after.find(']') else { break };
        let fields: Vec<(&str, &str)> = after[..end]
            .split(',')
            .filter_map(|field| field.split_once('='))
            .collect();
        let source = ["url", "file"].iter().find_map(|key| {
            fields
                .iter()
                .find(|(name, value)| name == key && !value.is_empty())
                .map(|(_, value)| *value)
        });
        if let Some(source) = source {
            urls.push(luo9_sdk::segment::unescape(source));
        }
        remaining = &after[end + 1..];
    }
    urls
}

pub(crate) fn strip_image_cq(message: &str) -> String {
    let mut result = String::with_capacity(message.len());
    let mut remaining = message;
    while let Some(start) = remaining.find("[CQ:image,") {
        result.push_str(&remaining[..start]);
        let after = &remaining[start + 10..];
        let Some(end) = after.find(']') else {
            result.push_str(&remaining[start..]);
            return result.trim().to_owned();
        };
        remaining = &after[end + 1..];
    }
    result.push_str(remaining);
    result.trim().to_owned()
}

pub(crate) fn recognize_for_user(image_url: &str, user_id: u64) -> Option<String> {
    if anti_injection::is_vision_disabled(user_id) {
        info!(user_id, "vision: 用户识图已被禁用");
        return None;
    }
    recognize(image_url)
}

#[derive(Debug)]
pub(crate) enum VlmError {
    Serialize(serde_json::Error),
    Http(crate::util::HttpError),
    Image(String),
    Provider(String),
    NoText,
}
impl std::fmt::Display for VlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialize(error) => write!(f, "JSON 编码/解析失败: {error}"),
            Self::Http(error) => write!(f, "{error}"),
            Self::Image(error) => write!(f, "图片处理失败: {error}"),
            Self::Provider(error) => write!(f, "视觉 API 返回错误: {error}"),
            Self::NoText => write!(f, "响应里没有可提取的文本"),
        }
    }
}

#[derive(Clone)]
struct VisionSettings {
    base_url: String,
    api_key: String,
    model: String,
    api_format: VisionApiFormat,
    detail: VisionDetail,
    max_image_bytes: usize,
    native_deepseek: bool,
}

fn is_deepseek(base_url: &str) -> bool {
    let lower = base_url.trim().to_ascii_lowercase();
    lower
        .strip_prefix("https://")
        .or_else(|| lower.strip_prefix("http://"))
        .and_then(|rest| rest.split('/').next())
        .and_then(|authority| authority.split(':').next())
        == Some("api.deepseek.com")
}

fn resolve_settings(
    vision: &VisionConfig,
    main_base_url: &str,
    main_api_key: &str,
    main_model: &str,
) -> VisionSettings {
    let (base_url, api_key) = if vision.use_main_api {
        (main_base_url, main_api_key)
    } else {
        (vision.base_url.as_str(), vision.api_key.as_str())
    };
    let native_deepseek = is_deepseek(base_url);
    let api_format = match vision.api_format {
        VisionApiFormat::Auto if native_deepseek => VisionApiFormat::ChatCompletions,
        VisionApiFormat::Auto => VisionApiFormat::Responses,
        other => other,
    };
    VisionSettings {
        base_url: base_url.trim().trim_end_matches('/').to_owned(),
        api_key: api_key.trim().to_owned(),
        model: if vision.model.trim().is_empty() {
            main_model.trim()
        } else {
            vision.model.trim()
        }
        .to_owned(),
        api_format,
        detail: vision.detail,
        max_image_bytes: vision.max_image_bytes.clamp(1, 32 * 1024 * 1024),
        native_deepseek,
    }
}

fn endpoint(settings: &VisionSettings) -> String {
    let base = settings.base_url.trim_end_matches('/');
    let base = base
        .strip_suffix("/chat/completions")
        .or_else(|| base.strip_suffix("/responses"))
        .unwrap_or(base);
    format!(
        "{base}/{}",
        match settings.api_format {
            VisionApiFormat::ChatCompletions => "chat/completions",
            _ => "responses",
        }
    )
}

fn request_body(
    settings: &VisionSettings,
    images: &[String],
    prompt: &str,
    max_tokens: u32,
) -> Value {
    let mut content = Vec::with_capacity(images.len() + 1);
    let mut body = match settings.api_format {
        VisionApiFormat::ChatCompletions => {
            content.push(json!({"type":"text", "text":prompt}));
            content.extend(images.iter().map(|url| {
                if url.starts_with("file-api-") {
                    json!({"type":"file", "file_id":url})
                } else {
                    json!({"type":"image_url", "image_url":{"url":url, "detail":settings.detail}})
                }
            }));
            json!({"model":settings.model, "messages":[{"role":"user", "content":content}], "max_tokens":max_tokens.max(1), "stream":false})
        }
        _ => {
            content.push(json!({"type":"input_text", "text":prompt}));
            content.extend(images.iter().map(|url| {
                if url.starts_with("file-api-") {
                    json!({"type":"input_image", "file_id":url})
                } else {
                    json!({"type":"input_image", "image_url":url, "detail":settings.detail})
                }
            }));
            json!({"model":settings.model, "input":[{"role":"user", "content":content}], "max_output_tokens":max_tokens.max(1)})
        }
    };
    if settings.native_deepseek && settings.api_format == VisionApiFormat::ChatCompletions {
        body["thinking"] = json!({"type":"disabled"});
    }
    body
}

/// 依据真实文件内容判断 MIME；限制字节数，避免只信扩展名。
pub(crate) fn image_data_url(bytes: &[u8], max_bytes: usize) -> Result<String, VlmError> {
    if bytes.is_empty() || bytes.len() > max_bytes {
        return Err(VlmError::Image("图片为空或超过大小上限".into()));
    }
    let mime = match image::guess_format(bytes) {
        Ok(image::ImageFormat::Png) => "image/png",
        Ok(image::ImageFormat::Jpeg) => "image/jpeg",
        Ok(image::ImageFormat::Gif) => "image/gif",
        Ok(image::ImageFormat::WebP) => "image/webp",
        _ => return Err(VlmError::Image("仅支持 JPEG、PNG、GIF、WebP 图片".into())),
    };
    Ok(format!("data:{mime};base64,{}", STANDARD.encode(bytes)))
}

fn decode_image(encoded: &str, limit: usize) -> Result<String, VlmError> {
    if encoded.len() > limit.saturating_add(2) / 3 * 4 {
        return Err(VlmError::Image("Base64 图片超过大小上限".into()));
    }
    let bytes = STANDARD
        .decode(encoded)
        .map_err(|e| VlmError::Image(e.to_string()))?;
    image_data_url(&bytes, limit)
}

fn prepare_image(source: &str, settings: &VisionSettings) -> Result<String, VlmError> {
    if source.starts_with("file-api-") {
        return Ok(source.to_owned());
    }
    if let Some(encoded) = source.strip_prefix("base64://") {
        return decode_image(encoded, settings.max_image_bytes);
    }
    if source.starts_with("data:") {
        let (header, encoded) = source
            .split_once(',')
            .ok_or_else(|| VlmError::Image("无效的 data URL".into()))?;
        if !header.ends_with(";base64") {
            return Err(VlmError::Image("图片 data URL 必须使用 Base64".into()));
        }
        return decode_image(encoded, settings.max_image_bytes);
    }
    if source.starts_with("https://") || source.starts_with("http://") {
        // QQ 临时链接由插件下载并内联传给 DeepSeek，避免模型侧无法访问 QQ 链接。
        if settings.native_deepseek {
            let bytes = crate::util::get_bytes_limited(source, settings.max_image_bytes)
                .map_err(VlmError::Http)?;
            return image_data_url(&bytes, settings.max_image_bytes);
        }
        return Ok(source.to_owned());
    }
    let path = std::path::Path::new(source);
    if path.is_file() {
        let metadata = path
            .metadata()
            .map_err(|e| VlmError::Image(e.to_string()))?;
        if metadata.len() > settings.max_image_bytes as u64 {
            return Err(VlmError::Image("本地图片超过大小上限".into()));
        }
        let bytes = std::fs::read(path).map_err(|e| VlmError::Image(e.to_string()))?;
        return image_data_url(&bytes, settings.max_image_bytes);
    }
    // file:// 或 NapCat 哈希不是插件本机路径；经宿主查询资源。
    let metadata = luo9_sdk::napcat::ApiClient::new(std::time::Duration::from_secs(5))
        .get_image(source)
        .map_err(|e| VlmError::Image(format!("获取 NapCat 图片失败: {e}")))?;
    if let Some(base64) = metadata
        .get("base64")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
    {
        return decode_image(base64, settings.max_image_bytes);
    }
    for key in ["url", "file"] {
        if let Some(resource) = metadata
            .get(key)
            .and_then(Value::as_str)
            .filter(|s| *s != source)
        {
            if resource.starts_with("http://")
                || resource.starts_with("https://")
                || std::path::Path::new(resource).is_file()
            {
                return prepare_image(resource, settings);
            }
        }
    }
    Err(VlmError::Image(
        "NapCat 只返回了远端路径，请启用 enableLocalFile2Url 或提供可访问的 URL".into(),
    ))
}

/// 所有识图、表情包描述和表情包选择共用此调用路径。
pub(crate) fn call_vlm(images: &[&str], prompt: &str, max_tokens: u32) -> Result<String, VlmError> {
    if !crate::extension::enabled("core.vision") {
        return Err(VlmError::Provider("视觉扩展已停用".into()));
    }
    let cfg = config::get();
    let settings = resolve_settings(&cfg.vision, &cfg.base_url, &cfg.api_key, &cfg.model);
    if !cfg.vision.enabled() || settings.api_key.is_empty() {
        return Err(VlmError::Provider("识图未启用或 API 密钥为空".into()));
    }
    let images: Vec<String> = images
        .iter()
        .map(|source| prepare_image(source, &settings))
        .collect::<Result<_, _>>()?;
    send_vlm(
        &settings,
        &images,
        prompt,
        max_tokens,
        &crate::ai::no_error_agent(),
    )
}

/// 表情包缓存与识图使用同一资源解析路径，支持 file 标识和内联图片。
pub(crate) fn load_image_bytes(source: &str) -> Result<Vec<u8>, VlmError> {
    let cfg = config::get();
    let settings = resolve_settings(&cfg.vision, &cfg.base_url, &cfg.api_key, &cfg.model);
    if source.starts_with("http://") || source.starts_with("https://") {
        return crate::util::get_bytes_limited(source, settings.max_image_bytes)
            .map_err(VlmError::Http);
    }
    let url = prepare_image(source, &settings)?;
    if url.starts_with("http://") || url.starts_with("https://") {
        return crate::util::get_bytes_limited(&url, settings.max_image_bytes)
            .map_err(VlmError::Http);
    }
    let (_, encoded) = url
        .split_once(',')
        .ok_or_else(|| VlmError::Image("图片资源不是内联数据".into()))?;
    STANDARD
        .decode(encoded)
        .map_err(|error| VlmError::Image(error.to_string()))
}

fn send_vlm(
    settings: &VisionSettings,
    images: &[String],
    prompt: &str,
    max_tokens: u32,
    agent: &ureq::Agent,
) -> Result<String, VlmError> {
    let url = endpoint(settings);
    let json_body = serde_json::to_string(&request_body(settings, images, prompt, max_tokens))
        .map_err(VlmError::Serialize)?;
    debug!(model = %settings.model, format = ?settings.api_format, "vision: 发送视觉请求");
    let body = crate::util::post_json(agent, &url, &settings.api_key, &json_body)
        .map_err(VlmError::Http)?;
    let value: Value = serde_json::from_str(&body).map_err(VlmError::Serialize)?;
    if let Some(error) = value.get("error").filter(|e| !e.is_null()) {
        let message = error
            .get("message")
            .and_then(Value::as_str)
            .or_else(|| error.as_str())
            .unwrap_or("上游请求失败");
        return Err(VlmError::Provider(message.to_owned()));
    }
    extract_vlm_value(&value).ok_or(VlmError::NoText)
}

fn extract_vlm_value(value: &Value) -> Option<String> {
    let clean = |text: String| (!text.trim().is_empty()).then(|| text.trim().to_owned());
    let from_responses = value
        .get("output")
        .and_then(Value::as_array)
        .and_then(|output| {
            let texts: Vec<&str> = output
                .iter()
                .filter(|item| item.get("type").and_then(Value::as_str) != Some("reasoning"))
                .filter_map(|item| item.get("content").and_then(Value::as_array))
                .flatten()
                .filter_map(|part| part.get("text").and_then(Value::as_str))
                .collect();
            clean(texts.join("\n"))
        });
    let from_choices = value
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| {
            if let Some(text) = content.as_str() {
                clean(text.to_owned())
            } else {
                content.as_array().and_then(|parts| {
                    clean(
                        parts
                            .iter()
                            .filter_map(|part| part.get("text").and_then(Value::as_str))
                            .collect::<Vec<_>>()
                            .join("\n"),
                    )
                })
            }
        });
    from_responses.or(from_choices).or_else(|| {
        ["output_text", "text"]
            .iter()
            .filter_map(|key| value.get(key).and_then(Value::as_str))
            .find_map(|text| clean(text.to_owned()))
    })
}

#[cfg(test)]
fn extract_vlm_text(body: &str) -> Option<String> {
    extract_vlm_value(&serde_json::from_str::<Value>(body).ok()?)
}

pub(crate) fn recognize(image_url: &str) -> Option<String> {
    if !crate::extension::enabled("core.vision") {
        return None;
    }
    let cfg = config::get();
    if !cfg.vision.enabled() {
        return None;
    }
    match call_vlm(
        &[image_url],
        &crate::prompt::PromptManager::get().raw("vision_describe"),
        cfg.vision.max_tokens,
    ) {
        Ok(text) => Some(text),
        Err(error) => {
            debug!(%error, "vision: 识图失败");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn settings(format: VisionApiFormat) -> VisionSettings {
        let mut vision = VisionConfig::default();
        vision.api_key = "sk-test".into();
        vision.api_format = format;
        resolve_settings(
            &vision,
            "https://api.deepseek.com",
            "sk-main",
            "deepseek-flash",
        )
    }
    #[test]
    fn native_deepseek_uses_current_model_and_chat_image_blocks() {
        let settings = settings(VisionApiFormat::Auto);
        assert_eq!(settings.model, "deepseek-flash");
        assert_eq!(
            endpoint(&settings),
            "https://api.deepseek.com/chat/completions"
        );
        let body = request_body(
            &settings,
            &["data:image/png;base64,abc".into()],
            "描述图片",
            256,
        );
        assert_eq!(body["messages"][0]["role"], "user");
        assert_eq!(body["messages"][0]["content"][1]["type"], "image_url");
        assert_eq!(body["thinking"]["type"], "disabled");
        assert!(body.get("input").is_none());
    }
    #[test]
    fn main_credentials_and_legacy_responses_are_both_supported() {
        let mut vision = VisionConfig::default();
        vision.use_main_api = true;
        assert!(vision.enabled());
        let settings = resolve_settings(
            &vision,
            "https://api.deepseek.com/v1/",
            "sk-main",
            "deepseek-chat",
        );
        assert_eq!(settings.api_key, "sk-main");
        assert_eq!(
            endpoint(&settings),
            "https://api.deepseek.com/v1/chat/completions"
        );
        vision.use_main_api = false;
        vision.api_key = "sk-independent".into();
        vision.base_url = "https://ark.cn-beijing.volces.com/api/v3".into();
        let settings = resolve_settings(&vision, "unused", "sk-main", "model");
        assert_eq!(settings.api_format, VisionApiFormat::Responses);
        let body = request_body(
            &settings,
            &["https://example.test/a.png".into()],
            "描述",
            64,
        );
        assert_eq!(body["input"][0]["content"][1]["type"], "input_image");
        assert!(body.get("thinking").is_none());
    }
    #[test]
    fn explicit_responses_and_empty_model_fallback_work() {
        let mut settings = settings(VisionApiFormat::Responses);
        settings.base_url = "https://api.deepseek.com/responses/".into();
        assert_eq!(endpoint(&settings), "https://api.deepseek.com/responses");
        let mut vision = VisionConfig::default();
        vision.model = " \t ".into();
        assert_eq!(
            resolve_settings(&vision, "", "", "deepseek-flash").model,
            "deepseek-flash"
        );
        let body = request_body(&settings, &["file-api-existing".into()], "描述", 64);
        assert_eq!(
            body["input"][0]["content"][1]["file_id"],
            "file-api-existing"
        );
        assert!(body["input"][0]["content"][1].get("image_url").is_none());
        settings.api_format = VisionApiFormat::ChatCompletions;
        let body = request_body(&settings, &["file-api-existing".into()], "描述", 64);
        assert_eq!(body["messages"][0]["content"][1]["type"], "file");
    }
    #[test]
    fn response_text_is_complete_and_ignores_reasoning() {
        assert_eq!(extract_vlm_text(r#"{"output":[{"type":"reasoning","content":[{"text":"隐藏推理"}]},{"content":[{"text":"第一段"},{"text":"第二段"}]}]}"#).as_deref(), Some("第一段\n第二段"));
        assert_eq!(
            extract_vlm_text(
                r#"{"choices":[{"message":{"reasoning_content":"推理","content":"猫"}}]}"#
            )
            .as_deref(),
            Some("猫")
        );
        assert_eq!(
            extract_vlm_text(r#"{"output":[],"choices":[{"message":{"content":"回退"}}]}"#)
                .as_deref(),
            Some("回退")
        );
        for body in ["", "junk", "{}", r#"{"output_text":" "}"#] {
            assert!(extract_vlm_text(body).is_none());
        }
    }
    #[test]
    fn cq_images_decode_parameters_and_keep_file_only_resources() {
        assert_eq!(
            extract_image_urls(
                "看[CQ:image,file=hash,url=https://example.test/a&#44;b?x=1&amp;y=2][CQ:image,file=file://abc]"
            ),
            vec!["https://example.test/a,b?x=1&y=2", "file://abc"]
        );
        assert_eq!(strip_image_cq("看[CQ:image,file=hash]图"), "看图");
        assert_eq!(
            strip_image_cq("看[CQ:image,file=hash"),
            "看[CQ:image,file=hash"
        );
    }
    #[test]
    fn images_use_actual_format_and_enforce_size() {
        let png = b"\x89PNG\r\n\x1a\nmorebytes";
        let url = image_data_url(png, 32).unwrap();
        assert!(url.starts_with("data:image/png;base64,"));
        assert!(image_data_url(png, 2).is_err());
        assert!(image_data_url(b"not an image", 100).is_err());
        assert!(decode_image(&STANDARD.encode(png), 2).is_err());
        assert!(!is_deepseek("https://api.deepseek.com.evil.test"));
    }
    #[test]
    fn chat_and_responses_requests_work_over_http_and_return_provider_errors() {
        for format in [VisionApiFormat::ChatCompletions, VisionApiFormat::Responses] {
            let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
            let mut settings = settings(format);
            settings.base_url = format!("http://{}", server.server_addr());
            let handler =
                std::thread::spawn(move || {
                    for failed in [false, true] {
                        let mut request = server.recv().unwrap();
                        let expected = if format == VisionApiFormat::ChatCompletions {
                            "/chat/completions"
                        } else {
                            "/responses"
                        };
                        assert_eq!(request.url(), expected);
                        assert!(
                            request
                                .headers()
                                .iter()
                                .any(|h| h.field.equiv("Authorization")
                                    && h.value.as_str() == "Bearer sk-test")
                        );
                        let mut raw = String::new();
                        request.as_reader().read_to_string(&mut raw).unwrap();
                        let value: Value = serde_json::from_str(&raw).unwrap();
                        if format == VisionApiFormat::ChatCompletions {
                            assert_eq!(
                                value["messages"][0]["content"][1]["image_url"]["url"],
                                "data:image/png;base64,abc"
                            );
                        } else {
                            assert_eq!(
                                value["input"][0]["content"][1]["image_url"],
                                "data:image/png;base64,abc"
                            );
                        }
                        let body = if failed {
                            r#"{"error":{"message":"model does not support images"}}"#
                        } else if format == VisionApiFormat::ChatCompletions {
                            r#"{"choices":[{"message":{"content":"图片描述"}}]}"#
                        } else {
                            r#"{"output":[{"content":[{"text":"图片描述"}]}]}"#
                        };
                        request
                            .respond(
                                tiny_http::Response::from_string(body)
                                    .with_status_code(if failed { 400 } else { 200 }),
                            )
                            .unwrap();
                    }
                });
            let agent = crate::util::agent(crate::util::AgentSpec::reading_error_body(5));
            let images = vec!["data:image/png;base64,abc".to_owned()];
            assert_eq!(
                send_vlm(&settings, &images, "描述", 100, &agent).unwrap(),
                "图片描述"
            );
            assert!(matches!(
                send_vlm(&settings, &images, "描述", 100, &agent),
                Err(VlmError::Provider(_))
            ));
            handler.join().unwrap();
        }
    }
}
