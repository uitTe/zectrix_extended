use super::client::{ApiError, ZectrixClient};
use super::models::{Todo, TodoDraft};

/// 获取待办列表 — `GET /open/v1/todos`
///
/// `status`：0 = 待完成，1 = 已完成；`device_id` 用于按设备过滤。
pub async fn list(
    client: &ZectrixClient,
    api_key: &str,
    status: Option<i32>,
    device_id: Option<&str>,
) -> Result<Vec<Todo>, ApiError> {
    let mut query: Vec<(&str, String)> = Vec::new();
    if let Some(status) = status {
        query.push(("status", status.to_string()));
    }
    if let Some(device_id) = device_id.filter(|d| !d.trim().is_empty()) {
        query.push(("deviceId", device_id.to_string()));
    }
    client.get("/todos", api_key, &query).await
}

/// 创建待办 — `POST /open/v1/todos`（`title` 必填）
pub async fn create(
    client: &ZectrixClient,
    api_key: &str,
    draft: &TodoDraft,
) -> Result<(), ApiError> {
    let title = draft
        .title
        .as_deref()
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .ok_or_else(|| ApiError::Invalid("待办标题不能为空".into()))?;

    let body = TodoDraft {
        title: Some(title.to_string()),
        ..draft.clone()
    };
    client.post_json("/todos", api_key, &body).await
}

/// 更新待办 — `PUT /open/v1/todos/{id}`
pub async fn update(
    client: &ZectrixClient,
    api_key: &str,
    id: i64,
    draft: &TodoDraft,
) -> Result<(), ApiError> {
    client
        .put_json(&format!("/todos/{id}"), api_key, draft)
        .await
}

/// 标记完成 / 取消完成 — `PUT /open/v1/todos/{id}/complete`
pub async fn toggle_complete(
    client: &ZectrixClient,
    api_key: &str,
    id: i64,
) -> Result<(), ApiError> {
    client
        .put_empty(&format!("/todos/{id}/complete"), api_key)
        .await
}

/// 删除待办 — `DELETE /open/v1/todos/{id}`
pub async fn delete(client: &ZectrixClient, api_key: &str, id: i64) -> Result<(), ApiError> {
    client.delete(&format!("/todos/{id}"), api_key).await
}
