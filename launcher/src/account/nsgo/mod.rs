//! NSGO OAuth 2.1 + Yggdrasil authentication provider for Nshorizon.
use super::*;

pub mod agent;
pub mod oauth;

pub const DEFAULT_BASE_URL: &str = "https://go.naborajs.me";
pub const CLIENT_ID: &str = "nshorizon-launcher";
pub const SCOPES: &str = "openid profile email";
pub const REDIRECT_PORTS: [u16; 3] = [38467, 38468, 38469];
pub const EXPIRED: &str = "Your NSGO sign-in expired. Sign in again.";

pub fn base_url() -> String {
    std::env::var("NSHORIZON_AUTH_BASE_URL")
        .unwrap_or_else(|_| DEFAULT_BASE_URL.into())
        .trim_end_matches('/')
        .to_string()
}

pub fn authorize_url() -> String {
    format!("{}/api/oauth/authorize", base_url())
}

pub fn token_url() -> String {
    format!("{}/api/oauth/token", base_url())
}

pub fn userinfo_url() -> String {
    format!("{}/api/oauth/userinfo", base_url())
}

pub fn yggdrasil_url() -> String {
    format!("{}/api/yggdrasil", base_url())
}

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub uuid: String,
    pub name: String,
    pub skin: Option<(String, String)>,
    pub cape: Option<String>,
}

fn sanitize_username(candidate: &str) -> String {
    let mut name: String = candidate
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
        .collect();

    while name.contains("__") {
        name = name.replace("__", "_");
    }
    let trimmed = name.trim_matches('_');
    if !trimmed.is_empty() {
        name = trimmed.to_string();
    }
    if name.len() > 16 {
        name.truncate(16);
    }
    while name.len() < 3 {
        name.push('_');
    }
    name
}

fn sanitize_uuid(raw: &str) -> String {
    let undashed = raw.replace('-', "").to_lowercase();
    if undashed.len() == 32 && undashed.chars().all(|c| c.is_ascii_hexdigit()) {
        return undashed;
    }
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());
    let hash = hasher.finalize();
    hash.iter().take(16).map(|b| format!("{:02x}", b)).collect()
}

pub fn fetch_userinfo(token: &str) -> Result<UserInfo, String> {
    let url = userinfo_url();
    let res = request_to("GET", &url)
        .set("Authorization", &format!("Bearer {token}"))
        .call()
        .map_err(|e| format!("Could not reach NSGO userinfo endpoint: {e}"))?;

    let body = res
        .into_string()
        .map_err(|e| format!("Invalid NSGO userinfo response: {e}"))?;

    let val: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| format!("NSGO userinfo is not valid JSON: {e}"))?;

    let raw_uuid = val
        .get("minecraft_uuid")
        .or_else(|| val.get("uuid"))
        .or_else(|| val.get("id"))
        .or_else(|| val.get("sub"))
        .and_then(|v| v.as_str())
        .unwrap_or("00000000000000000000000000000000");

    let undashed_uuid = sanitize_uuid(raw_uuid);

    let candidate_name = val
        .get("minecraft_username")
        .or_else(|| val.get("username"))
        .or_else(|| val.get("preferred_username"))
        .or_else(|| val.get("name"))
        .or_else(|| val.get("display_name"))
        .or_else(|| val.get("full_name"))
        .and_then(|v| v.as_str())
        .or_else(|| {
            val.get("email")
                .and_then(|v| v.as_str())
                .and_then(|e| e.split('@').next())
        })
        .unwrap_or("Player");

    let username = sanitize_username(candidate_name);

    let skin_url = val
        .get("skin_url")
        .or_else(|| val.get("skin"))
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let skin_model = val
        .get("skin_model")
        .or_else(|| val.get("model"))
        .and_then(|v| v.as_str())
        .unwrap_or("classic")
        .to_string();

    let cape_url = val
        .get("cape_url")
        .or_else(|| val.get("cape"))
        .and_then(|v| v.as_str())
        .map(str::to_string);

    Ok(UserInfo {
        uuid: undashed_uuid,
        name: username,
        skin: skin_url.map(|url| (url, skin_model)),
        cape: cape_url,
    })
}

pub(super) const NSGO_RENEW_BEFORE: u64 = 300;

pub(super) fn refresh(account: &Account, mut secrets: Secrets, force: bool) -> Check {
    if !secrets.nsgo {
        return Check::NoAnswer(EXPIRED.into(), secrets);
    }
    if !force && secrets.minecraft_expires_at > now() + NSGO_RENEW_BEFORE && !secrets.sign_in_required {
        return Check::Owns(account.clone(), secrets);
    }

    match oauth::refresh_tokens(&secrets.refresh_token) {
        Ok((access_token, new_refresh_token, expires_in)) => {
            secrets.minecraft_token = access_token;
            if let Some(r) = new_refresh_token {
                secrets.refresh_token = r;
            }
            secrets.minecraft_expires_at = now() + expires_in.unwrap_or(86400);
            secrets.sign_in_required = false;
            secrets.owns = true;

            let mut renewed = account.clone();
            if let Ok(info) = fetch_userinfo(&secrets.minecraft_token) {
                renewed.name = info.name;
                renewed.skin = info.skin;
                renewed.cape = info.cape;
            }
            Check::Owns(renewed, secrets)
        }
        Err(e) => {
            secrets.sign_in_required = true;
            Check::NoAnswer(e, secrets)
        }
    }
}
