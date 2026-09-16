use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// 极趣云开放 API 基础地址（末位不含斜杠）
pub const API_BASE: &str = "https://cloud.zectrix.com/open/v1";

/// 云平台统一响应包装：`{ "code": 0, "msg": ..., "data": ... }`
///
/// `code == 0` 视为成功；非 0 时 `msg` 携带原因（实测 401 时 msg 为「无效或已过期的API Key」）。
///
/// 注意：`Option<T>` 字段**不要**加 `#[serde(default)]` —— serde 会因此给泛型参数
/// 追加 `T: Default` 约束，导致 `ApiResponse<Device>` 这类具体化直接编译失败。
/// `Option<T>` 本身就允许字段缺失，无需 default。
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i64,
    pub msg: Option<String>,
    pub data: Option<T>,
}

/// 统一错误类型。可序列化回前端，形如 `{ kind, message }`。
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("尚未配置 API Key，请先到「设置」页填写")]
    MissingApiKey,
    #[error("API Key 存取失败：{0}")]
    Keyring(String),
    #[error("网络请求失败：{0}")]
    Network(String),
    #[error("{msg}")]
    Api { code: i64, msg: String },
    #[error("响应解析失败：{0}")]
    Decode(String),
    #[error("{0}")]
    Invalid(String),
}

impl ApiError {
    /// 机器可读的错误分类，供前端做差异化提示
    pub fn kind(&self) -> &'static str {
        match self {
            ApiError::MissingApiKey => "missing_api_key",
            ApiError::Keyring(_) => "keyring",
            ApiError::Network(_) => "network",
            ApiError::Api { .. } => "api",
            ApiError::Decode(_) => "decode",
            ApiError::Invalid(_) => "invalid",
        }
    }
}

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ApiError", 2)?;
        state.serialize_field("kind", self.kind())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

/// 截断过长的响应体，避免把整页 HTML 塞进报错信息
fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        text.to_string()
    } else {
        let head: String = text.chars().take(max_chars).collect();
        format!("{head}…")
    }
}

/// 共享的 HTTP 客户端，复用连接池
#[derive(Clone)]
pub struct ZectrixClient {
    http: reqwest::Client,
}

impl ZectrixClient {
    pub fn new() -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .user_agent(concat!("zectrix-extended/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("构建 HTTP 客户端失败");
        Self { http }
    }

    fn url(&self, path: &str) -> String {
        format!("{API_BASE}{path}")
    }

    /// 统一发送 + 解包：注入 `X-API-Key`，`code != 0` 转为 `ApiError::Api`
    async fn send_opt<T: DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
        api_key: &str,
    ) -> Result<Option<T>, ApiError> {
        let resp = req
            .header("X-API-Key", api_key)
            .send()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;

        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;

        let parsed: ApiResponse<T> = serde_json::from_str(&body).map_err(|e| {
            ApiError::Decode(format!(
                "{e}（HTTP {status}，原始响应：{}）",
                truncate(&body, 200)
            ))
        })?;

        if parsed.code != 0 {
            let msg = parsed
                .msg
                .filter(|m| !m.trim().is_empty())
                .unwrap_or_else(|| format!("HTTP {status}"));
            return Err(ApiError::Api {
                code: parsed.code,
                msg,
            });
        }

        Ok(parsed.data)
    }

    /// GET，要求 data 存在。查询参数为空时不会给 URL 挂上多余的 `?`。
    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        api_key: &str,
        query: &[(&str, String)],
    ) -> Result<T, ApiError> {
        let mut req = self.http.get(self.url(path));
        if !query.is_empty() {
            req = req.query(query);
        }
        self.send_opt::<T>(req, api_key)
            .await?
            .ok_or_else(|| ApiError::Decode("接口返回成功，但 data 为空".into()))
    }

    /// POST JSON，忽略返回的 data
    pub(crate) async fn post_json<B: Serialize>(
        &self,
        path: &str,
        api_key: &str,
        body: &B,
    ) -> Result<(), ApiError> {
        let req = self.http.post(self.url(path)).json(body);
        self.send_opt::<serde_json::Value>(req, api_key)
            .await
            .map(|_| ())
    }

    /// PUT JSON，忽略返回的 data
    pub(crate) async fn put_json<B: Serialize>(
        &self,
        path: &str,
        api_key: &str,
        body: &B,
    ) -> Result<(), ApiError> {
        let req = self.http.put(self.url(path)).json(body);
        self.send_opt::<serde_json::Value>(req, api_key)
            .await
            .map(|_| ())
    }

    /// 无请求体的 PUT（如「标记完成」）
    pub(crate) async fn put_empty(&self, path: &str, api_key: &str) -> Result<(), ApiError> {
        let req = self.http.put(self.url(path));
        self.send_opt::<serde_json::Value>(req, api_key)
            .await
            .map(|_| ())
    }

    pub(crate) async fn delete(&self, path: &str, api_key: &str) -> Result<(), ApiError> {
        let req = self.http.delete(self.url(path));
        self.send_opt::<serde_json::Value>(req, api_key)
            .await
            .map(|_| ())
    }

    /// 多部分表单上传（图片推送）
    pub(crate) async fn post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        api_key: &str,
        form: reqwest::multipart::Form,
    ) -> Result<Option<T>, ApiError> {
        let req = self.http.post(self.url(path)).multipart(form);
        self.send_opt::<T>(req, api_key).await
    }
}

impl Default for ZectrixClient {
    fn default() -> Self {
        Self::new()
    }
}
