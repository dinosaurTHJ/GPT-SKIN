use hmac::{Hmac, Mac};
use reqwest::{
    Request, RequestBuilder, Url,
    header::{HeaderName, HeaderValue},
};
use serde::Deserialize;
use sha2::Sha256;
use std::{
    sync::{OnceLock, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

const CONFIG_SOURCE: &str = include_str!("../config/api.toml");
const ACCESS_KEY: HeaderName = HeaderName::from_static("accesskey");
const CONTENT_DATE: HeaderName = HeaderName::from_static("content-date");
const CONTENT_MD5: HeaderName = HeaderName::from_static("content-md5");
const DEFAULT_LANGUAGE: &str = "zh-CN";
pub const MISSING_API_CONFIG_ERROR: &str = "RETHEME_API_CONFIG_MISSING";

#[derive(Deserialize)]
struct ApiConfig {
    base_url: String,
    secret_id: String,
    secret_key: String,
}

static CONFIG: OnceLock<ApiConfig> = OnceLock::new();
static LANGUAGE: OnceLock<RwLock<String>> = OnceLock::new();

fn config() -> &'static ApiConfig {
    CONFIG.get_or_init(|| {
        let mut config: ApiConfig = toml::from_str(CONFIG_SOURCE).expect("invalid config/api.toml");
        if let Ok(secret_id) = std::env::var("RETHEME_API_SECRET_ID") {
            if !secret_id.trim().is_empty() {
                config.secret_id = secret_id;
            }
        }
        if let Ok(secret_key) = std::env::var("RETHEME_API_SECRET_KEY") {
            if !secret_key.trim().is_empty() {
                config.secret_key = secret_key;
            }
        }
        assert!(
            !config.secret_id.is_empty(),
            "API secret_id cannot be empty"
        );
        assert!(
            !config.secret_key.is_empty(),
            "API secret_key cannot be empty"
        );
        config
    })
}

pub fn base() -> &'static str {
    config().base_url.trim_end_matches('/')
}

pub fn url(path: &str) -> String {
    format!("{}{}", base(), path)
}

pub fn set_language(locale: &str) {
    let language = normalize_language(locale);
    *LANGUAGE
        .get_or_init(|| RwLock::new(DEFAULT_LANGUAGE.to_owned()))
        .write()
        .expect("API language poisoned") = language.to_owned();
}

fn normalize_language(locale: &str) -> &'static str {
    if locale.to_ascii_lowercase().starts_with("zh") {
        DEFAULT_LANGUAGE
    } else {
        "en"
    }
}

fn language() -> String {
    LANGUAGE
        .get_or_init(|| RwLock::new(DEFAULT_LANGUAGE.to_owned()))
        .read()
        .expect("API language poisoned")
        .clone()
}

pub fn sign(request: RequestBuilder) -> Result<Request, String> {
    sign_at(request, unix_time())
}

fn sign_at(request: RequestBuilder, timestamp: u64) -> Result<Request, String> {
    let config = config();
    validate_credentials(config)?;
    sign_at_with_credentials(request, timestamp, &config.secret_id, &config.secret_key)
}

fn sign_at_with_credentials(
    request: RequestBuilder,
    timestamp: u64,
    secret_id: &str,
    secret_key: &str,
) -> Result<Request, String> {
    let mut request = request
        .build()
        .map_err(|error| format!("无法构建 API 请求：{error}"))?;
    let signature = signature(request.url(), timestamp, secret_key)?;
    let headers = request.headers_mut();
    headers.insert(
        ACCESS_KEY,
        HeaderValue::from_str(secret_id).map_err(|_| "API SecretID 格式无效".to_owned())?,
    );
    headers.insert(
        CONTENT_DATE,
        HeaderValue::from_str(&timestamp.to_string())
            .map_err(|_| "API 请求时间格式无效".to_owned())?,
    );
    headers.insert(
        CONTENT_MD5,
        HeaderValue::from_str(&signature).map_err(|_| "API 请求签名格式无效".to_owned())?,
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        HeaderValue::from_str(&language()).map_err(|_| "API 语言格式无效".to_owned())?,
    );
    Ok(request)
}

fn validate_credentials(config: &ApiConfig) -> Result<(), String> {
    if is_placeholder_credential(&config.secret_id) || is_placeholder_credential(&config.secret_key)
    {
        return Err(MISSING_API_CONFIG_ERROR.to_owned());
    }
    Ok(())
}

fn is_placeholder_credential(value: &str) -> bool {
    matches!(
        value.trim(),
        "" | "local-development"
            | "replace-with-desktop-secret-id"
            | "replace-with-desktop-secret-key"
    )
}

fn signature(url: &Url, timestamp: u64, secret_key: &str) -> Result<String, String> {
    let payload = format!(
        "{}\n{}\n{}",
        url.path(),
        url.query().unwrap_or_default(),
        timestamp
    );
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
        .map_err(|_| "API SecretKey 格式无效".to_owned())?;
    mac.update(payload.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

fn unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[test]
    fn uses_desktop_api_by_default() {
        assert_eq!(config().base_url, "https://theme.dux.cn/api/desktop/v1");
    }

    #[test]
    fn joins_api_paths_once() {
        assert_eq!(
            url("/compatibility/codex"),
            format!("{}/compatibility/codex", base())
        );
    }

    #[test]
    fn signs_final_path_query_and_timestamp() {
        let url = Url::parse(
            "https://theme.dux.cn/api/desktop/v1/compatibility/codex?codex_version=1.2.3&engine_version=1",
        )
        .expect("url");
        assert_eq!(
            signature(&url, 1_721_234_567, "test-secret").expect("signature"),
            "e4f00e01ab240c00ff5c450b854216761065b885749742d322a82cbe71cf40a8"
        );
    }

    #[test]
    fn adds_all_dux_signature_headers() {
        let request = Client::new().get("https://theme.dux.cn/api/desktop/v1/account?view=summary");
        let request =
            sign_at_with_credentials(request, 1_721_234_567, "test-secret-id", "test-secret-key")
                .expect("signed request");
        assert!(!request.headers()[ACCESS_KEY].is_empty());
        assert_eq!(request.headers()[CONTENT_DATE], "1721234567");
        assert_eq!(request.headers()[CONTENT_MD5].as_bytes().len(), 64);
        assert_eq!(request.headers()[reqwest::header::ACCEPT_LANGUAGE], "zh-CN");
    }

    #[test]
    fn rejects_placeholder_credentials_before_network_request() {
        let request = Client::new().get("https://theme.dux.cn/api/desktop/v1/account");
        assert_eq!(
            sign_at(request, 1_721_234_567).expect_err("placeholder config should fail"),
            MISSING_API_CONFIG_ERROR
        );
    }

    #[test]
    fn normalizes_the_desktop_locale_for_api_requests() {
        assert_eq!(normalize_language("zh-CN"), "zh-CN");
        assert_eq!(normalize_language("zh-TW"), "zh-CN");
        assert_eq!(normalize_language("en-US"), "en");
    }
}
