use super::client::{ApiError, ZectrixClient};
use super::models::Device;

/// 获取设备列表 — `GET /open/v1/devices`
pub async fn list(client: &ZectrixClient, api_key: &str) -> Result<Vec<Device>, ApiError> {
    client.get("/devices", api_key, &[]).await
}
