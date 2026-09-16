use serde::Deserialize;
use std::sync::OnceLock;

const CONFIG_SOURCE: &str = include_str!("../config/security.toml");

#[derive(Deserialize)]
struct SecurityConfig {
    package_key: String,
    license_public_key: String,
    compatibility_public_key: String,
    theme_public_key: String,
}

static CONFIG: OnceLock<SecurityConfig> = OnceLock::new();

fn config() -> &'static SecurityConfig {
    CONFIG.get_or_init(|| {
        let config: SecurityConfig =
            toml::from_str(CONFIG_SOURCE).expect("invalid config/security.toml");
        assert!(
            config.package_key.len() >= 32,
            "security package_key must contain at least 32 bytes"
        );
        for (name, public_key) in [
            ("license_public_key", &config.license_public_key),
            ("compatibility_public_key", &config.compatibility_public_key),
            ("theme_public_key", &config.theme_public_key),
        ] {
            assert!(
                public_key.len() == 64 && public_key.bytes().all(|byte| byte.is_ascii_hexdigit()),
                "security {name} must be a 32-byte hexadecimal public key"
            );
        }
        config
    })
}

pub fn package_key() -> &'static str {
    &config().package_key
}

pub fn license_public_key() -> &'static str {
    &config().license_public_key
}

pub fn compatibility_public_key() -> &'static str {
    &config().compatibility_public_key
}

pub fn theme_public_key() -> &'static str {
    &config().theme_public_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_complete_security_config() {
        assert!(package_key().len() >= 32);
        assert_eq!(license_public_key().len(), 64);
        assert_eq!(compatibility_public_key().len(), 64);
        assert_eq!(theme_public_key().len(), 64);
    }
}
