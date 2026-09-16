use base64::Engine as _;
use serde::Serialize;

use super::client::{ApiError, ZectrixClient};
use super::models::{ImageUpload, PushImageResult};

/// 云端限制：单次最多 5 张
const MAX_IMAGES: usize = 5;
/// 云端限制：单张不超过 2 MB
const MAX_IMAGE_BYTES: usize = 2 * 1024 * 1024;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TextBody<'a> {
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_id: Option<&'a str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StructuredTextBody<'a> {
    title: &'a str,
    body: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_id: Option<&'a str>,
}

/// 仅保留有意义的 pageId：空串等同于不传
fn normalize_page_id(page_id: Option<&str>) -> Option<&str> {
    page_id.map(str::trim).filter(|p| !p.is_empty())
}

/// 推送纯文本 — `POST /open/v1/devices/{deviceId}/display/text`
pub async fn push_text(
    client: &ZectrixClient,
    api_key: &str,
    device_id: &str,
    text: &str,
    font_size: Option<i32>,
    page_id: Option<&str>,
) -> Result<(), ApiError> {
    if text.trim().is_empty() {
        return Err(ApiError::Invalid("推送文本不能为空".into()));
    }
    let body = TextBody {
        text,
        font_size,
        page_id: normalize_page_id(page_id),
    };
    client
        .post_json(
            &format!("/devices/{device_id}/display/text"),
            api_key,
            &body,
        )
        .await
}

/// 推送标题 + 正文 — `POST /open/v1/devices/{deviceId}/display/structured-text`
pub async fn push_structured_text(
    client: &ZectrixClient,
    api_key: &str,
    device_id: &str,
    title: &str,
    body: &str,
    page_id: Option<&str>,
) -> Result<(), ApiError> {
    if title.trim().is_empty() {
        return Err(ApiError::Invalid("标题不能为空".into()));
    }
    let payload = StructuredTextBody {
        title,
        body,
        page_id: normalize_page_id(page_id),
    };
    client
        .post_json(
            &format!("/devices/{device_id}/display/structured-text"),
            api_key,
            &payload,
        )
        .await
}

/// 推送图片 — `POST /open/v1/devices/{deviceId}/display/image`（multipart）
///
/// 入参中的 base64 会在此解码为原始字节；解码前后都做体积与格式校验，
/// 避免把一个必然被云端拒绝的请求发出去。
pub async fn push_image(
    client: &ZectrixClient,
    api_key: &str,
    device_id: &str,
    images: &[ImageUpload],
    dither: bool,
    page_id: Option<&str>,
) -> Result<Option<PushImageResult>, ApiError> {
    if images.is_empty() {
        return Err(ApiError::Invalid("请至少选择一张图片".into()));
    }
    if images.len() > MAX_IMAGES {
        return Err(ApiError::Invalid(format!(
            "单次最多推送 {MAX_IMAGES} 张图片，当前选择了 {} 张",
            images.len()
        )));
    }

    let mut form = reqwest::multipart::Form::new();

    for image in images {
        let mime = image.mime.trim().to_ascii_lowercase();
        if !mime.starts_with("image/") {
            return Err(ApiError::Invalid(format!(
                "「{}」不是图片文件（MIME：{}）",
                image.name, image.mime
            )));
        }

        let bytes = base64::engine::general_purpose::STANDARD
            .decode(image.data_base64.trim())
            .map_err(|e| ApiError::Invalid(format!("图片「{}」解码失败：{e}", image.name)))?;

        if bytes.len() > MAX_IMAGE_BYTES {
            return Err(ApiError::Invalid(format!(
                "图片「{}」为 {:.2} MB，超过单张 2 MB 上限",
                image.name,
                bytes.len() as f64 / 1024.0 / 1024.0
            )));
        }

        let part = reqwest::multipart::Part::bytes(bytes)
            .file_name(image.name.clone())
            .mime_str(&mime)
            .map_err(|e| ApiError::Invalid(format!("MIME 类型不合法（{}）：{e}", image.mime)))?;

        form = form.part("images", part);
    }

    form = form.text("dither", if dither { "true" } else { "false" });
    if let Some(page_id) = normalize_page_id(page_id) {
        form = form.text("pageId", page_id.to_string());
    }

    client
        .post_multipart(
            &format!("/devices/{device_id}/display/image"),
            api_key,
            form,
        )
        .await
}

/// 删除页面 — `DELETE /open/v1/devices/{deviceId}/display/pages/{pageId}`
///
/// 不传 `page_id` 时会删除该设备的**全部**页面。
pub async fn clear_pages(
    client: &ZectrixClient,
    api_key: &str,
    device_id: &str,
    page_id: Option<&str>,
) -> Result<(), ApiError> {
    let path = match normalize_page_id(page_id) {
        Some(page_id) => format!("/devices/{device_id}/display/pages/{page_id}"),
        None => format!("/devices/{device_id}/display/pages"),
    };
    client.delete(&path, api_key).await
}
