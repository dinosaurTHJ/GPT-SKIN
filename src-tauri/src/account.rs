use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD as BASE64_STANDARD, URL_SAFE_NO_PAD},
};
use crypto_secretbox::{KeyInit, Nonce, XSalsa20Poly1305, aead::Aead};
use ed25519_dalek::{Signature, VerifyingKey};
use hkdf::Hkdf;
use rand_core::OsRng;
use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    collections::BTreeMap,
    fmt, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};
use x25519_dalek::{PublicKey, StaticSecret};

use crate::{
    api, security_config,
    theme::{ThemeInstallReport, ThemeRepository},
};

const TOKEN_ENTRY: &str = "access-token";
const DEVICE_SECRET_ENTRY: &str = "device-x25519-private";
const AUTH_REQUIRED_ERROR: &str = "RETHEME_AUTH_REQUIRED";
const MAX_ONLINE_THEME_SIZE: u64 = 30 * 1024 * 1024;
const PLATFORM_PACKAGE_MAGIC: &[u8; 4] = b"RTP1";

#[derive(Debug)]
pub struct AccountError {
    message: String,
    status: Option<u16>,
    connectivity: bool,
}

impl AccountError {
    fn message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: None,
            connectivity: false,
        }
    }

    fn connectivity(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: None,
            connectivity: true,
        }
    }

    fn invalid_session(&self) -> bool {
        matches!(self.status, Some(401 | 403 | 409))
    }
}

impl fmt::Display for AccountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AccountError {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entitlement {
    id: u64,
    #[serde(rename = "type")]
    grant_type: String,
    theme_id: Option<u64>,
    theme_slug: Option<String>,
    source: Value,
    meta: Option<Value>,
    granted_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeTrial {
    theme_id: u64,
    theme_slug: Option<String>,
    started_at: String,
    expires_at: String,
    expires_at_timestamp: u64,
    remaining_seconds: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountStatus {
    authenticated: bool,
    email: Option<String>,
    pro: bool,
    device_name: String,
    device_generation: Option<u64>,
    heartbeat_state: HeartbeatState,
    last_heartbeat_at: Option<u64>,
    lease_expires_at: Option<String>,
    entitlements: Vec<Entitlement>,
    trials: Vec<ThemeTrial>,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AccountDeviceSummary {
    id: String,
    name: String,
    current: bool,
    active: bool,
    registered_at: Option<String>,
    last_seen_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountThemeAuthor {
    id: String,
    name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountThemePreview {
    background: String,
    surface: String,
    accent: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountThemeLocalization {
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AccountThemeSummary {
    slug: String,
    name: String,
    description: Option<String>,
    #[serde(default)]
    locales: BTreeMap<String, AccountThemeLocalization>,
    version: String,
    author: AccountThemeAuthor,
    preview: Option<AccountThemePreview>,
    cover_url: Option<String>,
    first_used_at: Option<String>,
    last_used_at: Option<String>,
    use_count: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountThemeLibrary {
    favorites: Vec<AccountThemeSummary>,
    used: Vec<AccountThemeSummary>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountSync {
    devices: Vec<AccountDeviceSummary>,
    themes: AccountThemeLibrary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum HeartbeatState {
    Offline,
    Online,
    Grace,
    Replaced,
}

#[derive(Debug, Clone, Deserialize)]
struct AccountInfo {
    email: String,
    #[serde(default)]
    pro: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct LoginResponse {
    token: String,
    account: AccountInfo,
}

#[derive(Debug, Deserialize)]
struct OAuthStartResponse {
    authorize_url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthStart {
    authorize_url: String,
}

#[derive(Debug, Clone, Deserialize)]
struct DeviceInfo {
    id: String,
    name: String,
    generation: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct LeaseResponse {
    device: DeviceInfo,
    #[serde(rename = "heartbeat_interval_seconds")]
    _heartbeat_interval_seconds: u64,
    license: String,
    license_expires_at: String,
    #[serde(rename = "entitlements")]
    _entitlements: Vec<Entitlement>,
    #[serde(rename = "trials")]
    _trials: Vec<ThemeTrial>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct LeasePayload {
    jti: String,
    account_id: u64,
    account_email: String,
    device_id: String,
    generation: u64,
    issued_at: u64,
    expires_at: u64,
    entitlements: Vec<Entitlement>,
    trials: Vec<ThemeTrial>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredLease {
    token: String,
    expires_at: String,
}

#[derive(Debug, Clone)]
struct VerifiedLease {
    stored: StoredLease,
    payload: LeasePayload,
}

#[derive(Debug, Default)]
struct AccountSession {
    account: Option<AccountInfo>,
    generation: Option<u64>,
    lease: Option<VerifiedLease>,
    heartbeat_state: Option<HeartbeatState>,
    last_heartbeat_at: Option<u64>,
    heartbeat_interval_seconds: u64,
    error: Option<String>,
}

#[derive(Debug, Clone)]
struct PendingOAuth {
    state: String,
    code_verifier: String,
}

#[derive(Clone)]
pub struct AccountRuntime {
    client: Client,
    data_dir: PathBuf,
    device_id: String,
    device_name: String,
    session: Arc<Mutex<AccountSession>>,
    pending_oauth: Arc<Mutex<Option<PendingOAuth>>>,
    verifying_key: VerifyingKey,
}

#[derive(Debug, Deserialize)]
struct ApiEnvelope {
    code: u16,
    message: String,
    data: Value,
}

#[derive(Debug, Deserialize)]
struct OnlineTheme {
    slug: String,
}

#[derive(Debug, Deserialize)]
struct OnlineDownload {
    theme: OnlineTheme,
    package: OnlinePackage,
}

#[derive(Debug, Deserialize)]
pub struct ThemeUseAuthorization {
    pub expires_at: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct OnlinePackage {
    download_url: String,
    expires_at: u64,
    digest: String,
    size: u64,
    plain_size: u64,
}

impl AccountRuntime {
    pub fn new(data_dir: PathBuf) -> Result<Self, AccountError> {
        fs::create_dir_all(&data_dir)
            .map_err(|error| AccountError::message(format!("无法创建账号数据目录：{error}")))?;
        secure_directory(&data_dir)?;
        let device_id = load_or_create_device_id(&data_dir)?;
        let device_name = std::env::var("COMPUTERNAME")
            .or_else(|_| std::env::var("HOSTNAME"))
            .unwrap_or_else(|_| "ReTheme Desktop".to_owned());
        Ok(Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(20))
                .build()
                .map_err(|error| AccountError::message(format!("无法初始化网络客户端：{error}")))?,
            data_dir,
            device_id,
            device_name,
            session: Arc::new(Mutex::new(AccountSession::default())),
            pending_oauth: Arc::new(Mutex::new(None)),
            verifying_key: license_verifying_key()?,
        })
    }

    pub fn status(&self) -> AccountStatus {
        let session = self.session.lock().expect("account session poisoned");
        let entitlements = session
            .lease
            .as_ref()
            .map(|lease| lease.payload.entitlements.clone())
            .unwrap_or_default();
        let trials = session
            .lease
            .as_ref()
            .map(|lease| lease.payload.trials.clone())
            .unwrap_or_default();
        let pro = session.lease.as_ref().is_some_and(|lease| {
            validate_pro_access(
                &lease.payload,
                &self.device_id,
                session.generation,
                unix_time(),
            )
            .is_ok()
        });
        AccountStatus {
            authenticated: session.account.is_some(),
            email: session
                .account
                .as_ref()
                .map(|account| account.email.clone()),
            pro,
            device_name: self.device_name.clone(),
            device_generation: session.generation,
            heartbeat_state: session.heartbeat_state.unwrap_or(HeartbeatState::Offline),
            last_heartbeat_at: session.last_heartbeat_at,
            lease_expires_at: session
                .lease
                .as_ref()
                .map(|lease| lease.stored.expires_at.clone()),
            entitlements,
            trials,
            error: session.error.clone(),
        }
    }

    pub fn has_active_pro(&self) -> bool {
        let now = unix_time();
        let session = self.session.lock().expect("account session poisoned");
        session.lease.as_ref().is_some_and(|lease| {
            validate_pro_access(&lease.payload, &self.device_id, session.generation, now).is_ok()
        })
    }

    pub async fn sync(&self) -> Result<AccountSync, AccountError> {
        let token = self.require_token()?;
        self.get(
            &format!("/account/sync?device_id={}", self.device_id),
            &token,
        )
        .await
    }

    pub fn theme_cache_key(&self) -> Result<[u8; 32], AccountError> {
        let secret = self.device_secret()?;
        let hkdf = Hkdf::<Sha256>::new(None, &secret.to_bytes());
        let mut key = [0_u8; 32];
        hkdf.expand(b"retheme-theme-cache-v1", &mut key)
            .map_err(|_| AccountError::message("无法派生在线主题缓存密钥"))?;
        Ok(key)
    }

    pub async fn initialize(&self) {
        let Some(token) = self.read_secret(TOKEN_ENTRY).ok().flatten() else {
            self.clear_local_session();
            return;
        };
        match self.get::<AccountInfo>("/account", &token).await {
            Ok(account) => {
                if let Err(error) = self.activate(&token, account).await {
                    self.handle_connection_error(error);
                }
            }
            Err(error) if error.connectivity => {
                self.restore_lease();
            }
            Err(error) => {
                self.clear_credentials();
                self.set_error(error.message, HeartbeatState::Offline);
            }
        }
    }

    pub async fn login(
        &self,
        email: String,
        password: String,
    ) -> Result<AccountStatus, AccountError> {
        let result: LoginResponse = self
            .post(
                "/auth/login",
                None,
                json!({
                    "email": email,
                    "password": password,
                    "client_type": "desktop"
                }),
            )
            .await?;
        self.accept_login(result).await
    }

    pub async fn start_oauth(&self, provider: String) -> Result<OAuthStart, AccountError> {
        if !matches!(provider.as_str(), "github" | "linuxdo") {
            return Err(AccountError::message("不支持的第三方登录方式"));
        }
        let state = random_url_token();
        let code_verifier = random_url_token();
        let code_challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(code_verifier.as_bytes()));
        let result: OAuthStartResponse = self
            .post(
                "/auth/oauth/start",
                None,
                json!({
                    "provider": provider,
                    "state": state,
                    "code_challenge": code_challenge,
                    "code_challenge_method": "S256",
                    "redirect_uri": "retheme://auth/callback",
                    "client_type": "desktop"
                }),
            )
            .await?;
        if !is_allowed_oauth_url(&result.authorize_url) {
            return Err(AccountError::message("服务端返回了不受信任的登录地址"));
        }
        *self.pending_oauth.lock().expect("oauth state poisoned") = Some(PendingOAuth {
            state,
            code_verifier,
        });
        Ok(OAuthStart {
            authorize_url: result.authorize_url,
        })
    }

    pub async fn complete_oauth(
        &self,
        code: String,
        state: String,
    ) -> Result<AccountStatus, AccountError> {
        let pending = {
            let mut pending_oauth = self.pending_oauth.lock().expect("oauth state poisoned");
            let pending = pending_oauth
                .as_ref()
                .ok_or_else(|| AccountError::message("登录请求已失效，请重新发起第三方登录"))?;
            if code.is_empty() || state != pending.state {
                return Err(AccountError::message("第三方登录状态校验失败"));
            }
            pending_oauth.take().expect("validated oauth state")
        };
        let result: LoginResponse = self
            .post(
                "/auth/oauth/exchange",
                None,
                json!({
                    "code": code,
                    "state": state,
                    "code_verifier": pending.code_verifier,
                    "redirect_uri": "retheme://auth/callback",
                    "client_type": "desktop"
                }),
            )
            .await?;
        self.accept_login(result).await
    }

    pub async fn request_register_code(
        &self,
        email: String,
    ) -> Result<Option<String>, AccountError> {
        let result: Value = self
            .post("/auth/register/code", None, json!({ "email": email }))
            .await?;
        Ok(result
            .get("debug_code")
            .and_then(Value::as_str)
            .map(str::to_owned))
    }

    pub async fn register(
        &self,
        email: String,
        code: String,
        password: String,
    ) -> Result<AccountStatus, AccountError> {
        let result: LoginResponse = self
            .post(
                "/auth/register",
                None,
                json!({
                    "email": email,
                    "code": code,
                    "password": password,
                    "client_type": "desktop"
                }),
            )
            .await?;
        self.accept_login(result).await
    }

    pub async fn logout(&self) {
        if let Ok(Some(token)) = self.read_secret(TOKEN_ENTRY) {
            self.deactivate_with_token(&token).await;
            let _: Result<Value, _> = self.post("/auth/logout", Some(&token), json!({})).await;
        }
        self.clear_credentials();
        *self.session.lock().expect("account session poisoned") = AccountSession::default();
    }

    pub async fn deactivate(&self) {
        if let Ok(Some(token)) = self.read_secret(TOKEN_ENTRY) {
            self.deactivate_with_token(&token).await;
        }
        let mut session = self.session.lock().expect("account session poisoned");
        session.generation = None;
        session.lease = None;
        session.heartbeat_state = Some(HeartbeatState::Offline);
        let _ = fs::remove_file(self.lease_path());
    }

    pub async fn redeem_cdk(&self, code: String) -> Result<AccountStatus, AccountError> {
        let token = self.require_token()?;
        let _: Value = self
            .post(
                "/cdk/redeem",
                Some(&token),
                json!({
                    "code": code,
                    "device_id": self.device_id
                }),
            )
            .await?;
        self.heartbeat(&token).await?;
        Ok(self.status())
    }

    pub async fn download_theme(
        &self,
        slug: String,
        themes: &ThemeRepository,
    ) -> Result<ThemeInstallReport, AccountError> {
        let token = self.require_theme_download_token()?;
        let download: OnlineDownload = self
            .post(
                &format!("/themes/{slug}/download"),
                Some(&token),
                json!({
                    "device_id": self.device_id
                }),
            )
            .await?;
        if download.theme.slug != slug {
            return Err(AccountError::message("下载主题标识不一致"));
        }
        let archive = self.download_package(&download.package).await?;
        themes
            .install_online(&archive, slug)
            .map_err(|error| AccountError::message(error.to_string()))
    }

    pub async fn authorize_theme(&self, slug: &str) -> Result<ThemeUseAuthorization, AccountError> {
        let token = self.require_token()?;
        self.post(
            &format!("/themes/{slug}/authorize"),
            Some(&token),
            json!({
                "device_id": self.device_id
            }),
        )
        .await
    }

    pub async fn maintain(&self) -> bool {
        let now = unix_time();
        let should_heartbeat = {
            let session = self.session.lock().expect("account session poisoned");
            session.account.is_some()
                && session.last_heartbeat_at.is_none_or(|last| {
                    now.saturating_sub(last) >= session.heartbeat_interval_seconds.max(1800)
                })
        };
        if should_heartbeat {
            match self.require_token() {
                Ok(token) => match self.heartbeat(&token).await {
                    Ok(()) => return false,
                    Err(error) if error.invalid_session() => {
                        self.set_error(error.message, HeartbeatState::Replaced);
                        return true;
                    }
                    Err(error) => self.handle_connection_error(error),
                },
                Err(_) => return true,
            }
        }
        let session = self.session.lock().expect("account session poisoned");
        session.lease.as_ref().is_some_and(|lease| {
            validate_lease(&lease.payload, &self.device_id, session.generation, now).is_err()
        })
    }

    async fn activate(&self, token: &str, account: AccountInfo) -> Result<(), AccountError> {
        let secret = self.device_secret()?;
        let public = PublicKey::from(&secret);
        let lease: LeaseResponse = self
            .post(
                "/desktop/session/activate",
                Some(token),
                json!({
                    "device_id": self.device_id,
                    "name": self.device_name,
                    "public_key": BASE64_STANDARD.encode(public.as_bytes())
                }),
            )
            .await?;
        self.accept_lease(account, lease)
    }

    async fn accept_login(&self, result: LoginResponse) -> Result<AccountStatus, AccountError> {
        self.write_secret(TOKEN_ENTRY, &result.token)?;
        if let Err(error) = self.activate(&result.token, result.account).await {
            self.delete_secret(TOKEN_ENTRY);
            return Err(error);
        }
        Ok(self.status())
    }

    async fn heartbeat(&self, token: &str) -> Result<(), AccountError> {
        let (generation, account) = {
            let session = self.session.lock().expect("account session poisoned");
            (
                session
                    .generation
                    .ok_or_else(|| AccountError::message("当前设备尚未激活"))?,
                session
                    .account
                    .clone()
                    .ok_or_else(|| AccountError::message("请先登录"))?,
            )
        };
        let lease: LeaseResponse = self
            .post(
                "/desktop/session/heartbeat",
                Some(token),
                json!({
                    "device_id": self.device_id,
                    "generation": generation
                }),
            )
            .await?;
        self.accept_lease(account, lease)
    }

    fn accept_lease(
        &self,
        mut account: AccountInfo,
        lease: LeaseResponse,
    ) -> Result<(), AccountError> {
        if lease.device.id != self.device_id || lease.device.name.is_empty() {
            return Err(AccountError::message("服务端返回了不匹配的设备租约"));
        }
        let payload = verify_token(&lease.license, &self.verifying_key)?;
        validate_lease(
            &payload,
            &self.device_id,
            Some(lease.device.generation),
            unix_time(),
        )?;
        account.email = payload.account_email.clone();
        account.pro = has_pro_entitlement(&payload.entitlements);
        let stored = StoredLease {
            token: lease.license,
            expires_at: lease.license_expires_at,
        };
        let encoded = serde_json::to_vec(&stored)
            .map_err(|error| AccountError::message(error.to_string()))?;
        fs::write(self.lease_path(), encoded)
            .map_err(|error| AccountError::message(format!("无法保存离线租约：{error}")))?;
        let mut session = self.session.lock().expect("account session poisoned");
        session.account = Some(account);
        session.generation = Some(lease.device.generation);
        session.lease = Some(VerifiedLease { stored, payload });
        session.heartbeat_state = Some(HeartbeatState::Online);
        session.last_heartbeat_at = Some(unix_time());
        session.heartbeat_interval_seconds = 1800;
        session.error = None;
        Ok(())
    }

    fn restore_lease(&self) {
        let result = fs::read(self.lease_path())
            .map_err(|error| AccountError::message(error.to_string()))
            .and_then(|bytes| {
                serde_json::from_slice::<StoredLease>(&bytes)
                    .map_err(|error| AccountError::message(error.to_string()))
            })
            .and_then(|stored| {
                verify_token(&stored.token, &self.verifying_key).map(|payload| (stored, payload))
            });
        if let Ok((stored, payload)) = result
            && validate_lease(
                &payload,
                &self.device_id,
                Some(payload.generation),
                unix_time(),
            )
            .is_ok()
        {
            let mut session = self.session.lock().expect("account session poisoned");
            session.account = Some(AccountInfo {
                email: payload.account_email.clone(),
                pro: has_pro_entitlement(&payload.entitlements),
            });
            session.generation = Some(payload.generation);
            session.lease = Some(VerifiedLease { stored, payload });
            session.heartbeat_state = Some(HeartbeatState::Grace);
            session.error = Some("服务暂时不可达，正在使用 24 小时离线租约".into());
        }
    }

    fn handle_connection_error(&self, error: AccountError) {
        let state = {
            let session = self.session.lock().expect("account session poisoned");
            if session.lease.as_ref().is_some_and(|lease| {
                validate_lease(
                    &lease.payload,
                    &self.device_id,
                    session.generation,
                    unix_time(),
                )
                .is_ok()
            }) {
                HeartbeatState::Grace
            } else {
                HeartbeatState::Offline
            }
        };
        self.set_error(error.message, state);
    }

    async fn download_package(&self, package: &OnlinePackage) -> Result<Vec<u8>, AccountError> {
        if package.expires_at <= unix_time() {
            return Err(AccountError::message("主题下载链接已过期，请重试"));
        }
        if package.size == 0
            || package.size > MAX_ONLINE_THEME_SIZE + 64
            || package.plain_size == 0
            || package.plain_size > MAX_ONLINE_THEME_SIZE
        {
            return Err(AccountError::message("主题包体积无效"));
        }
        let url = reqwest::Url::parse(&package.download_url)
            .map_err(|_| AccountError::message("主题下载链接无效"))?;
        if url.scheme() != "https" {
            return Err(AccountError::message("主题下载链接必须使用 HTTPS"));
        }
        let mut response = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/octet-stream")
            .send()
            .await
            .map_err(|error| AccountError::connectivity(format!("无法下载主题包：{error}")))?;
        if !response.status().is_success() {
            return Err(AccountError::message(format!(
                "主题包下载失败（HTTP {}）",
                response.status().as_u16()
            )));
        }
        if response
            .content_length()
            .is_some_and(|size| size != package.size || size > MAX_ONLINE_THEME_SIZE)
        {
            return Err(AccountError::message("主题包响应体积不一致"));
        }
        let mut archive = Vec::with_capacity(package.size as usize);
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|error| AccountError::connectivity(format!("无法读取主题包：{error}")))?
        {
            if archive.len() + chunk.len() > package.size as usize
                || archive.len() + chunk.len() > MAX_ONLINE_THEME_SIZE as usize
            {
                return Err(AccountError::message("主题包响应超过允许体积"));
            }
            archive.extend_from_slice(&chunk);
        }
        if archive.len() != package.size as usize {
            return Err(AccountError::message("主题包下载体积不一致"));
        }
        decrypt_platform_package(&archive, &package.digest, package.plain_size)
    }

    async fn deactivate_with_token(&self, token: &str) {
        let generation = self
            .session
            .lock()
            .expect("account session poisoned")
            .generation;
        if let Some(generation) = generation {
            let _: Result<Value, _> = self
                .post(
                    "/desktop/session/deactivate",
                    Some(token),
                    json!({
                        "device_id": self.device_id,
                        "generation": generation
                    }),
                )
                .await;
        }
    }

    async fn get<T: DeserializeOwned>(&self, path: &str, token: &str) -> Result<T, AccountError> {
        self.request(Method::GET, path, Some(token), None).await
    }

    async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        token: Option<&str>,
        body: Value,
    ) -> Result<T, AccountError> {
        self.request(Method::POST, path, token, Some(body)).await
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        token: Option<&str>,
        body: Option<Value>,
    ) -> Result<T, AccountError> {
        let mut request = self
            .client
            .request(method, api::url(path))
            .header(reqwest::header::ACCEPT, "application/json");
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        let request = api::sign(request).map_err(AccountError::message)?;
        let response = self.client.execute(request).await.map_err(|error| {
            AccountError::connectivity(format!("无法连接 ReTheme 服务：{error}"))
        })?;
        let status = response.status();
        let body = response.text().await.map_err(|error| {
            AccountError::connectivity(format!("无法读取 ReTheme 服务响应：{error}"))
        })?;
        parse_api_response(status, &body)
    }

    fn device_secret(&self) -> Result<StaticSecret, AccountError> {
        if let Some(encoded) = self.read_secret(DEVICE_SECRET_ENTRY)? {
            let bytes = decode_array::<32>(&encoded, "设备私钥")?;
            return Ok(StaticSecret::from(bytes));
        }
        let secret = StaticSecret::random_from_rng(OsRng);
        self.write_secret(
            DEVICE_SECRET_ENTRY,
            &BASE64_STANDARD.encode(secret.to_bytes()),
        )?;
        Ok(secret)
    }

    fn require_token(&self) -> Result<String, AccountError> {
        self.read_secret(TOKEN_ENTRY)?
            .ok_or_else(|| AccountError::message("请先登录 ReTheme"))
    }

    fn require_theme_download_token(&self) -> Result<String, AccountError> {
        self.read_secret(TOKEN_ENTRY)?
            .ok_or_else(|| AccountError::message(AUTH_REQUIRED_ERROR))
    }

    fn read_secret(&self, name: &str) -> Result<Option<String>, AccountError> {
        match fs::read_to_string(self.credential_path(name)) {
            Ok(value) => Ok(Some(value)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(AccountError::message(format!("无法读取账号凭据：{error}"))),
        }
    }

    fn write_secret(&self, name: &str, value: &str) -> Result<(), AccountError> {
        let path = self.credential_path(name);
        let mut options = fs::OpenOptions::new();
        options.create(true).write(true).truncate(true);
        #[cfg(unix)]
        options.mode(0o600);
        let mut file = options
            .open(&path)
            .map_err(|error| AccountError::message(format!("无法保存账号凭据：{error}")))?;
        file.write_all(value.as_bytes())
            .map_err(|error| AccountError::message(format!("无法保存账号凭据：{error}")))?;
        file.sync_all()
            .map_err(|error| AccountError::message(format!("无法同步账号凭据：{error}")))?;
        secure_file(&path)?;
        Ok(())
    }

    fn delete_secret(&self, name: &str) {
        let _ = fs::remove_file(self.credential_path(name));
    }

    fn clear_credentials(&self) {
        self.delete_secret(TOKEN_ENTRY);
        let _ = fs::remove_file(self.lease_path());
    }

    fn clear_local_session(&self) {
        self.clear_credentials();
        *self.session.lock().expect("account session poisoned") = AccountSession::default();
    }

    fn set_error(&self, error: String, state: HeartbeatState) {
        let mut session = self.session.lock().expect("account session poisoned");
        session.error = Some(error);
        session.heartbeat_state = Some(state);
        if state == HeartbeatState::Replaced {
            session.lease = None;
            session.generation = None;
            let _ = fs::remove_file(self.lease_path());
        }
    }

    fn lease_path(&self) -> PathBuf {
        self.data_dir.join("license-lease.json")
    }

    fn credential_path(&self, name: &str) -> PathBuf {
        self.data_dir.join(format!(".credential-{name}"))
    }
}

fn secure_directory(path: &Path) -> Result<(), AccountError> {
    #[cfg(unix)]
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))
        .map_err(|error| AccountError::message(format!("无法保护账号数据目录：{error}")))?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

fn secure_file(path: &Path) -> Result<(), AccountError> {
    #[cfg(unix)]
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))
        .map_err(|error| AccountError::message(format!("无法保护账号凭据：{error}")))?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

fn parse_api_response<T: DeserializeOwned>(
    status: StatusCode,
    body: &str,
) -> Result<T, AccountError> {
    let envelope = serde_json::from_str::<ApiEnvelope>(body)
        .map_err(|_| invalid_api_response(status, body))?;
    if !status.is_success() || envelope.code >= 400 {
        return Err(AccountError {
            message: envelope.message,
            status: Some(status.as_u16()),
            connectivity: status.is_server_error(),
        });
    }
    serde_json::from_value(envelope.data)
        .map_err(|error| AccountError::message(format!("服务端数据格式无效：{error}")))
}

fn invalid_api_response(status: StatusCode, body: &str) -> AccountError {
    let detail = readable_response_excerpt(body);
    let message = if detail.is_empty() {
        format!("服务端返回空响应（HTTP {}）", status.as_u16())
    } else {
        format!("服务端响应不是 JSON（HTTP {}）：{detail}", status.as_u16())
    };
    AccountError {
        message,
        status: Some(status.as_u16()),
        connectivity: status.is_server_error(),
    }
}

fn readable_response_excerpt(body: &str) -> String {
    let mut text = String::new();
    let mut inside_tag = false;
    for character in body.trim().chars() {
        match character {
            '<' => inside_tag = true,
            '>' => {
                inside_tag = false;
                text.push(' ');
            }
            _ if !inside_tag => text.push(character),
            _ => {}
        }
    }
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut excerpt = normalized.chars().take(180).collect::<String>();
    if normalized.chars().count() > 180 {
        excerpt.push('…');
    }
    excerpt
}

fn decrypt_platform_package(
    stored: &[u8],
    expected_digest: &str,
    expected_size: u64,
) -> Result<Vec<u8>, AccountError> {
    let nonce_start = PLATFORM_PACKAGE_MAGIC.len();
    let ciphertext_start = nonce_start + 24;
    if stored.len() <= ciphertext_start + 16
        || &stored[..PLATFORM_PACKAGE_MAGIC.len()] != PLATFORM_PACKAGE_MAGIC
    {
        return Err(AccountError::message("平台主题包格式无效"));
    }
    let key = Sha256::digest(security_config::package_key().as_bytes());
    let cipher = XSalsa20Poly1305::new_from_slice(&key)
        .map_err(|_| AccountError::message("平台主题包密钥无效"))?;
    let plain = cipher
        .decrypt(
            Nonce::from_slice(&stored[nonce_start..ciphertext_start]),
            &stored[ciphertext_start..],
        )
        .map_err(|_| AccountError::message("平台主题包解密失败"))?;
    if plain.len() != expected_size as usize
        || hex::encode(Sha256::digest(&plain)) != expected_digest
    {
        return Err(AccountError::message("平台主题包摘要或体积不一致"));
    }
    Ok(plain)
}

fn license_verifying_key() -> Result<VerifyingKey, AccountError> {
    let mut bytes = [0_u8; 32];
    hex::decode_to_slice(security_config::license_public_key(), &mut bytes)
        .map_err(|error| AccountError::message(format!("授权公钥无效：{error}")))?;
    VerifyingKey::from_bytes(&bytes).map_err(|_| AccountError::message("授权公钥无效"))
}

fn verify_token(token: &str, key: &VerifyingKey) -> Result<LeasePayload, AccountError> {
    let payload = verify_signed_bytes(token, key)?;
    serde_json::from_slice(&payload)
        .map_err(|error| AccountError::message(format!("离线租约内容无效：{error}")))
}

fn verify_signed_bytes(token: &str, key: &VerifyingKey) -> Result<Vec<u8>, AccountError> {
    let (encoded, signature) = token
        .split_once('.')
        .ok_or_else(|| AccountError::message("授权令牌格式无效"))?;
    let signature = URL_SAFE_NO_PAD
        .decode(signature)
        .map_err(|_| AccountError::message("授权签名格式无效"))?;
    let signature =
        Signature::from_slice(&signature).map_err(|_| AccountError::message("授权签名长度无效"))?;
    key.verify_strict(encoded.as_bytes(), &signature)
        .map_err(|_| AccountError::message("授权签名验证失败"))?;
    URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(|_| AccountError::message("授权内容编码无效"))
}

fn validate_lease(
    payload: &LeasePayload,
    device_id: &str,
    generation: Option<u64>,
    now: u64,
) -> Result<(), AccountError> {
    if payload.device_id != device_id || generation != Some(payload.generation) {
        return Err(AccountError::message("离线租约与当前设备不匹配"));
    }
    if payload.issued_at > now.saturating_add(300) || payload.expires_at <= now {
        return Err(AccountError::message("离线租约已过期"));
    }
    Ok(())
}

fn validate_pro_access(
    payload: &LeasePayload,
    device_id: &str,
    generation: Option<u64>,
    now: u64,
) -> Result<(), AccountError> {
    validate_lease(payload, device_id, generation, now)?;
    if has_pro_entitlement(&payload.entitlements) {
        return Ok(());
    }
    Err(AccountError::message("当前账号没有有效的 Pro 权益"))
}

fn has_pro_entitlement(entitlements: &[Entitlement]) -> bool {
    entitlements
        .iter()
        .any(|grant| matches!(grant.grant_type.as_str(), "pro_annual" | "pro_lifetime"))
}

fn load_or_create_device_id(data_dir: &Path) -> Result<String, AccountError> {
    let path = data_dir.join("device-id");
    if let Ok(value) = fs::read_to_string(&path) {
        let value = value.trim();
        if value.starts_with("rt-device-") && value.len() == 42 {
            return Ok(value.to_owned());
        }
    }
    let mut random = [0_u8; 16];
    rand_core::RngCore::fill_bytes(&mut OsRng, &mut random);
    let value = format!("rt-device-{}", hex::encode(random));
    fs::write(path, &value)
        .map_err(|error| AccountError::message(format!("无法保存设备标识：{error}")))?;
    Ok(value)
}

fn random_url_token() -> String {
    let mut random = [0_u8; 32];
    rand_core::RngCore::fill_bytes(&mut OsRng, &mut random);
    URL_SAFE_NO_PAD.encode(random)
}

fn is_allowed_oauth_url(value: &str) -> bool {
    reqwest::Url::parse(value).is_ok_and(|url| {
        url.scheme() == "https" && matches!(url.host_str(), Some("github.com" | "connect.linux.do"))
    })
}

fn decode_array<const LENGTH: usize>(
    value: &str,
    label: &str,
) -> Result<[u8; LENGTH], AccountError> {
    let decoded = BASE64_STANDARD
        .decode(value)
        .map_err(|_| AccountError::message(format!("{label} Base64 无效")))?;
    decoded
        .try_into()
        .map_err(|_| AccountError::message(format!("{label}长度无效")))
}

pub(crate) fn unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn signed_lease(payload: &LeasePayload, key: &SigningKey) -> String {
        let encoded = URL_SAFE_NO_PAD.encode(serde_json::to_vec(payload).expect("payload"));
        let signature = key.sign(encoded.as_bytes());
        format!("{encoded}.{}", URL_SAFE_NO_PAD.encode(signature.to_bytes()))
    }

    fn payload(now: u64) -> LeasePayload {
        LeasePayload {
            jti: "test".into(),
            account_id: 7,
            account_email: "test@retheme.app".into(),
            device_id: "rt-device-test".into(),
            generation: 2,
            issued_at: now,
            expires_at: now + 3600,
            entitlements: vec![],
            trials: vec![],
        }
    }

    #[test]
    fn stores_credentials_without_system_keyring() {
        let directory = tempfile::tempdir().expect("account directory");
        let runtime = AccountRuntime::new(directory.path().join("account")).expect("account");

        runtime
            .write_secret(TOKEN_ENTRY, "account-token")
            .expect("write secret");
        assert_eq!(
            runtime.read_secret(TOKEN_ENTRY).expect("read secret"),
            Some("account-token".into())
        );
        runtime.delete_secret(TOKEN_ENTRY);
        assert_eq!(runtime.read_secret(TOKEN_ENTRY).expect("deleted"), None);
    }

    #[cfg(unix)]
    #[test]
    fn restricts_credential_permissions() {
        let directory = tempfile::tempdir().expect("account directory");
        let runtime = AccountRuntime::new(directory.path().join("account")).expect("account");
        runtime
            .write_secret(DEVICE_SECRET_ENTRY, "device-secret")
            .expect("write secret");

        let directory_mode = fs::metadata(&runtime.data_dir)
            .expect("directory metadata")
            .permissions()
            .mode()
            & 0o777;
        let file_mode = fs::metadata(runtime.credential_path(DEVICE_SECRET_ENTRY))
            .expect("secret metadata")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(directory_mode, 0o700);
        assert_eq!(file_mode, 0o600);
    }

    #[test]
    fn requires_login_before_downloading_an_online_theme() {
        let directory = tempfile::tempdir().expect("account directory");
        let runtime = AccountRuntime::new(directory.path().join("account")).expect("account");
        assert_eq!(
            runtime
                .require_theme_download_token()
                .expect_err("missing token")
                .to_string(),
            AUTH_REQUIRED_ERROR
        );
    }

    #[cfg(debug_assertions)]
    #[test]
    fn clears_stale_lease_and_account_session() {
        let directory = tempfile::tempdir().expect("account directory");
        let runtime = AccountRuntime::new(directory.path().join("account")).expect("account");
        fs::write(runtime.lease_path(), b"stale lease").expect("stale lease");
        runtime
            .session
            .lock()
            .expect("account session poisoned")
            .account = Some(AccountInfo {
            email: "stale@retheme.app".into(),
            pro: true,
        });

        runtime.clear_local_session();

        let status = runtime.status();
        assert!(!status.authenticated);
        assert_eq!(status.heartbeat_state, HeartbeatState::Offline);
        assert!(!runtime.lease_path().exists());
    }

    #[cfg(debug_assertions)]
    #[test]
    fn ignores_forged_vip_fields_without_signed_entitlement() {
        let directory = tempfile::tempdir().expect("account directory");
        let mut runtime = AccountRuntime::new(directory.path().join("account")).expect("account");
        let signing_key = SigningKey::from_bytes(&[9; 32]);
        runtime.verifying_key = signing_key.verifying_key();
        let now = unix_time();
        let mut payload = payload(now);
        payload.device_id = runtime.device_id.clone();
        let forged_pro = Entitlement {
            id: 99,
            grant_type: "pro_lifetime".into(),
            theme_id: None,
            theme_slug: None,
            source: json!({"type": "forged", "id": "proxy"}),
            meta: None,
            granted_at: None,
        };
        runtime
            .accept_lease(
                AccountInfo {
                    email: "forged@retheme.app".into(),
                    pro: true,
                },
                LeaseResponse {
                    device: DeviceInfo {
                        id: runtime.device_id.clone(),
                        name: "Forged Device".into(),
                        generation: payload.generation,
                    },
                    _heartbeat_interval_seconds: u64::MAX,
                    license: signed_lease(&payload, &signing_key),
                    license_expires_at: "2099-12-31T23:59:59Z".into(),
                    _entitlements: vec![forged_pro],
                    _trials: vec![],
                },
            )
            .expect("signed non-pro lease");

        assert!(!runtime.status().pro);
        assert!(!runtime.has_active_pro());
        assert_eq!(
            runtime
                .session
                .lock()
                .expect("account session poisoned")
                .heartbeat_interval_seconds,
            1800
        );
    }

    #[test]
    fn parses_successful_api_response() {
        let data: Value = parse_api_response(
            StatusCode::OK,
            r#"{"code":200,"message":"ok","data":{"debug_code":"123456"}}"#,
        )
        .expect("valid response");
        assert_eq!(data["debug_code"], "123456");
    }

    #[test]
    fn preserves_api_business_error() {
        let error = parse_api_response::<Value>(
            StatusCode::UNPROCESSABLE_ENTITY,
            r#"{"code":422,"message":"验证码无效","data":null}"#,
        )
        .expect_err("business error");
        assert_eq!(error.message, "验证码无效");
        assert_eq!(error.status, Some(422));
        assert!(!error.connectivity);
    }

    #[test]
    fn reports_html_and_empty_server_responses() {
        let html = parse_api_response::<Value>(
            StatusCode::BAD_GATEWAY,
            "<html><title>Bad Gateway</title><body>upstream unavailable</body></html>",
        )
        .expect_err("html response");
        assert_eq!(
            html.message,
            "服务端响应不是 JSON（HTTP 502）：Bad Gateway upstream unavailable"
        );
        assert!(html.connectivity);

        let empty = parse_api_response::<Value>(StatusCode::INTERNAL_SERVER_ERROR, "  \n")
            .expect_err("empty response");
        assert_eq!(empty.message, "服务端返回空响应（HTTP 500）");
        assert!(empty.connectivity);
    }

    #[test]
    fn reports_invalid_success_data() {
        let error = parse_api_response::<LoginResponse>(
            StatusCode::OK,
            r#"{"code":200,"message":"ok","data":{"token":7}}"#,
        )
        .expect_err("invalid data");
        assert!(error.message.starts_with("服务端数据格式无效："));
    }

    #[test]
    fn verifies_signed_lease_and_rejects_tampering() {
        let now = unix_time();
        let key = SigningKey::from_bytes(&[9; 32]);
        let token = signed_lease(&payload(now), &key);
        let verified = verify_token(&token, &key.verifying_key()).expect("valid lease");
        assert_eq!(verified.account_id, 7);
        let mut tampered = token.into_bytes();
        tampered[3] ^= 1;
        assert!(
            verify_token(
                std::str::from_utf8(&tampered).expect("utf8"),
                &key.verifying_key()
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_expired_or_mismatched_lease() {
        let now = unix_time();
        let mut lease = payload(now);
        assert!(validate_lease(&lease, "rt-device-test", Some(2), now).is_ok());
        assert!(validate_lease(&lease, "other-device", Some(2), now).is_err());
        assert!(validate_lease(&lease, "rt-device-test", Some(3), now).is_err());
        lease.expires_at = now;
        assert!(validate_lease(&lease, "rt-device-test", Some(2), now).is_err());
    }

    #[test]
    fn recognizes_active_pro_supporter_plans() {
        let now = unix_time();
        for grant_type in ["pro_annual", "pro_lifetime"] {
            let mut lease = payload(now);
            lease.entitlements.push(Entitlement {
                id: 1,
                grant_type: grant_type.into(),
                theme_id: None,
                theme_slug: None,
                source: json!({"type": "test", "id": grant_type}),
                meta: None,
                granted_at: None,
            });
            assert!(
                validate_pro_access(&lease, "rt-device-test", Some(2), now).is_ok(),
                "{grant_type} should activate the Pro supporter identity"
            );
        }
    }

    #[test]
    fn rejects_unlicensed_or_expired_pro_identity() {
        let now = unix_time();
        let lease = payload(now);
        assert!(validate_pro_access(&lease, "rt-device-test", Some(2), now).is_err());

        let mut expired = payload(now);
        expired.entitlements.push(Entitlement {
            id: 1,
            grant_type: "pro_lifetime".into(),
            theme_id: None,
            theme_slug: None,
            source: json!({"type": "test", "id": "expired"}),
            meta: None,
            granted_at: None,
        });
        expired.expires_at = now;
        assert!(validate_pro_access(&expired, "rt-device-test", Some(2), now).is_err());
    }

    #[test]
    fn decrypts_platform_package_and_rejects_tampering() {
        let nonce = [5_u8; 24];
        let plain = b"signed theme package";
        let digest = hex::encode(Sha256::digest(plain));
        let key = Sha256::digest(security_config::package_key().as_bytes());
        let cipher = XSalsa20Poly1305::new_from_slice(&key).expect("cipher");
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plain.as_slice())
            .expect("encrypt");
        let mut stored = PLATFORM_PACKAGE_MAGIC.to_vec();
        stored.extend_from_slice(&nonce);
        stored.extend_from_slice(&ciphertext);

        let opened =
            decrypt_platform_package(&stored, &digest, plain.len() as u64).expect("decrypt");
        assert_eq!(opened, plain);

        stored[ciphertext.len()] ^= 1;
        assert!(decrypt_platform_package(&stored, &digest, plain.len() as u64).is_err());
    }
}
