use keyring::Entry;

use crate::api::ApiError;

/// Windows 凭据管理器里的服务名与账号名
const SERVICE: &str = "com.zectrix.extended";
const ACCOUNT: &str = "zectrix-api-key";

fn entry() -> Result<Entry, ApiError> {
    Entry::new(SERVICE, ACCOUNT).map_err(|e| ApiError::Keyring(e.to_string()))
}

/// 写入 API Key（存进系统凭据管理器，由 DPAPI 加密）
pub fn save(api_key: &str) -> Result<(), ApiError> {
    let api_key = api_key.trim();
    if api_key.is_empty() {
        return Err(ApiError::Invalid("API Key 不能为空".into()));
    }
    entry()?
        .set_password(api_key)
        .map_err(|e| ApiError::Keyring(e.to_string()))
}

/// 读取 API Key；未配置时返回 `None` 而不是报错
pub fn load() -> Result<Option<String>, ApiError> {
    match entry()?.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(ApiError::Keyring(e.to_string())),
    }
}

/// 读取 API Key；未配置则返回 `MissingApiKey` 错误，供命令层直接用 `?` 抛出
pub fn require() -> Result<String, ApiError> {
    load()?.ok_or(ApiError::MissingApiKey)
}

/// 清除已保存的 API Key；本来就没有也视为成功
pub fn clear() -> Result<(), ApiError> {
    match entry()?.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(ApiError::Keyring(e.to_string())),
    }
}

/// 生成掩码用于展示，绝不把完整 Key 回传前端
pub fn mask(api_key: &str) -> String {
    let chars: Vec<char> = api_key.chars().collect();
    if chars.len() <= 10 {
        return "•".repeat(chars.len());
    }
    let head: String = chars.iter().take(6).collect();
    let tail: String = chars.iter().skip(chars.len() - 4).collect();
    format!("{head}••••{tail}")
}
