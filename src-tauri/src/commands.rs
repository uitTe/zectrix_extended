use serde::Serialize;
use tauri::State;

use crate::api::{
    devices, display, todos, ApiError, Device, ImageUpload, PushImageResult, Todo, TodoDraft,
    ZectrixClient,
};
use crate::keystore;

/// 由 Tauri 托管的全局状态，复用同一个 HTTP 客户端（连接池）
pub struct AppState {
    pub http: ZectrixClient,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            http: ZectrixClient::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// API Key 的配置状态。**只回传掩码**，完整 Key 永不离开 Rust 进程。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyStatus {
    pub configured: bool,
    pub masked: Option<String>,
}

// ==================== API Key 管理 ====================

#[tauri::command]
pub fn api_key_status() -> Result<KeyStatus, ApiError> {
    let api_key = keystore::load()?;
    Ok(match api_key {
        Some(key) => KeyStatus {
            configured: true,
            masked: Some(keystore::mask(&key)),
        },
        None => KeyStatus {
            configured: false,
            masked: None,
        },
    })
}

#[tauri::command]
pub fn save_api_key(api_key: String) -> Result<KeyStatus, ApiError> {
    keystore::save(&api_key)?;
    Ok(KeyStatus {
        configured: true,
        masked: Some(keystore::mask(api_key.trim())),
    })
}

#[tauri::command]
pub fn clear_api_key() -> Result<KeyStatus, ApiError> {
    keystore::clear()?;
    Ok(KeyStatus {
        configured: false,
        masked: None,
    })
}

/// 「测试连接」：用已存的 Key 真打一次 `/devices`，成功即证明 Key 有效
#[tauri::command]
pub async fn verify_api_key(state: State<'_, AppState>) -> Result<Vec<Device>, ApiError> {
    let api_key = keystore::require()?;
    devices::list(&state.http, &api_key).await
}

// ==================== 设备 ====================

#[tauri::command]
pub async fn list_devices(state: State<'_, AppState>) -> Result<Vec<Device>, ApiError> {
    let api_key = keystore::require()?;
    devices::list(&state.http, &api_key).await
}

// ==================== 待办 ====================

#[tauri::command]
pub async fn list_todos(
    state: State<'_, AppState>,
    status: Option<i32>,
    device_id: Option<String>,
) -> Result<Vec<Todo>, ApiError> {
    let api_key = keystore::require()?;
    todos::list(&state.http, &api_key, status, device_id.as_deref()).await
}

#[tauri::command]
pub async fn create_todo(state: State<'_, AppState>, draft: TodoDraft) -> Result<(), ApiError> {
    let api_key = keystore::require()?;
    todos::create(&state.http, &api_key, &draft).await
}

#[tauri::command]
pub async fn update_todo(
    state: State<'_, AppState>,
    id: i64,
    draft: TodoDraft,
) -> Result<(), ApiError> {
    let api_key = keystore::require()?;
    todos::update(&state.http, &api_key, id, &draft).await
}

#[tauri::command]
pub async fn toggle_todo(state: State<'_, AppState>, id: i64) -> Result<(), ApiError> {
    let api_key = keystore::require()?;
    todos::toggle_complete(&state.http, &api_key, id).await
}

#[tauri::command]
pub async fn delete_todo(state: State<'_, AppState>, id: i64) -> Result<(), ApiError> {
    let api_key = keystore::require()?;
    todos::delete(&state.http, &api_key, id).await
}

// ==================== 显示推送 ====================

#[tauri::command]
pub async fn push_text(
    state: State<'_, AppState>,
    device_id: String,
    text: String,
    font_size: Option<i32>,
    page_id: Option<String>,
) -> Result<(), ApiError> {
    let api_key = keystore::require()?;
    display::push_text(
        &state.http,
        &api_key,
        &device_id,
        &text,
        font_size,
        page_id.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn push_structured_text(
    state: State<'_, AppState>,
    device_id: String,
    title: String,
    body: String,
    page_id: Option<String>,
) -> Result<(), ApiError> {
    let api_key = keystore::require()?;
    display::push_structured_text(
        &state.http,
        &api_key,
        &device_id,
        &title,
        &body,
        page_id.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn push_image(
    state: State<'_, AppState>,
    device_id: String,
    images: Vec<ImageUpload>,
    dither: Option<bool>,
    page_id: Option<String>,
) -> Result<Option<PushImageResult>, ApiError> {
    let api_key = keystore::require()?;
    display::push_image(
        &state.http,
        &api_key,
        &device_id,
        &images,
        dither.unwrap_or(true),
        page_id.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn clear_pages(
    state: State<'_, AppState>,
    device_id: String,
    page_id: Option<String>,
) -> Result<(), ApiError> {
    let api_key = keystore::require()?;
    display::clear_pages(&state.http, &api_key, &device_id, page_id.as_deref()).await
}
