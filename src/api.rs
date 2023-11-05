use serde::Deserialize;

pub const API_BASE_URL: &str = "https://gigachat.devices.sberbank.ru/api/v1";
pub const AUTH_URL: &str = "https://ngw.devices.sberbank.ru:9443/api/v2/oauth";
pub const SCOPE_PERS: &str = "GIGACHAT_API_PERS";
pub const SCOPE_CORP: &str = "GIGACHAT_API_CORP";

#[derive(Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_at: u64,
}

#[derive(Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
}
