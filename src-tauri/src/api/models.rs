use serde::{Deserialize, Serialize};

/// 设备（`GET /devices`）。`device_id` 即设备 MAC 地址，后续设备级接口都用它。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub device_id: String,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub board: Option<String>,
}

/// 待办事项（`GET /todos`）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub due_time: Option<String>,
    #[serde(default)]
    pub repeat_type: Option<String>,
    /// 0 = 待完成，1 = 已完成
    #[serde(default)]
    pub status: Option<i32>,
    #[serde(default)]
    pub priority: Option<i32>,
    #[serde(default)]
    pub completed: Option<bool>,
    #[serde(default)]
    pub device_id: Option<String>,
    #[serde(default)]
    pub device_name: Option<String>,
    #[serde(default)]
    pub create_date: Option<String>,
    /// 注意：云端字段名是 `updateDate`（而非 `updatedAt`），单独指定 rename
    #[serde(default, rename = "updateDate")]
    pub updated_at: Option<i64>,
}

/// 创建 / 更新待办的请求体。所有可选字段为 `None` 时不会被序列化，
/// 这样「更新」只发用户真正改动的字段。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoDraft {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

/// 图片推送的入参。前端把文件读成 base64 传进来，Rust 侧解码成字节走 multipart。
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageUpload {
    /// 文件名，会作为 multipart 的 file_name 传给云端
    pub name: String,
    /// MIME 类型，如 image/png
    pub mime: String,
    /// 原始字节的 base64（不含 data URL 前缀）
    pub data_base64: String,
}

/// 图片推送结果（`POST /devices/{id}/display/image`）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushImageResult {
    #[serde(default)]
    pub total_pages: Option<i32>,
    #[serde(default)]
    pub pushed_pages: Option<i32>,
    #[serde(default)]
    pub page_id: Option<String>,
}
