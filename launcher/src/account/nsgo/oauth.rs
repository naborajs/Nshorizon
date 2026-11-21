//! NSGO OAuth 2.1 Authorization Code Flow with S256 PKCE.
use super::*;
use base64::Engine;
use sha2::{Digest, Sha256};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{Duration, Instant};
use url::Url;

fn generate_random_bytes(len: usize) -> Result<Vec<u8>, String> {
    let mut buf = vec![0u8; len];
    getrandom::fill(&mut buf).map_err(|e| format!("Failed to generate secure random bytes: {e}"))?;
    Ok(buf)
}

fn generate_verifier() -> Result<String, String> {
    let bytes = generate_random_bytes(32)?;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes))
}

fn challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}

fn generate_state() -> Result<String, String> {
    let bytes = generate_random_bytes(24)?;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes))
}

pub struct BrowserFlow {
    listener: TcpListener,
    pub port: u16,
    pub state: String,
    pub verifier: String,
}

impl BrowserFlow {
    pub fn new() -> Result<Self, String> {
        let mut bound = None;
        for &port in &REDIRECT_PORTS {
            if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) {
                let _ = listener.set_nonblocking(true);
                bound = Some((listener, port));
                break;
            }
        }

        let (listener, port) = bound.ok_or_else(|| {
            format!(
                "Could not bind loopback listener to any port in {:?}",
                REDIRECT_PORTS
            )
        })?;

        let verifier = generate_verifier()?;
        let state = generate_state()?;

        Ok(Self {
            listener,
            port,
            state,
            verifier,
        })
    }

    pub fn authorization_url(&self) -> String {
        let mut url = Url::parse(&authorize_url()).expect("valid authorize url");
        let redirect_uri = format!("http://127.0.0.1:{}/oauth/nsgo", self.port);
        let code_challenge = challenge(&self.verifier);

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", CLIENT_ID)
            .append_pair("redirect_uri", &redirect_uri)
            .append_pair("scope", SCOPES)
            .append_pair("state", &self.state)
            .append_pair("code_challenge", &code_challenge)
            .append_pair("code_challenge_method", "S256");

        url.to_string()
    }

    pub fn finish(self, cancelled: &dyn Fn() -> bool) -> Result<(Account, Secrets), String> {
        let code = self.wait_for_code(cancelled)?;
        let tokens = exchange_code(&code, &self.verifier, self.port)?;
        let info = fetch_userinfo(&tokens.access_token)?;

        let account = Account {
            uuid: format!("nsgo:{}", info.uuid),
            name: info.name,
            skin: info.skin,
            cape: info.cape,
        };

        let secrets = Secrets {
            nsgo: true,
            refresh_token: tokens.refresh_token.unwrap_or_default(),
            sign_in_required: false,
            minecraft_token: tokens.access_token,
            minecraft_expires_at: now() + tokens.expires_in.unwrap_or(86400),
            owns: true,
            xuid: None,
        };

        Ok((account, secrets))
    }

    fn wait_for_code(&self, cancelled: &dyn Fn() -> bool) -> Result<String, String> {
        let deadline = Instant::now() + Duration::from_secs(120);
        while Instant::now() < deadline {
            if cancelled() {
                return Err("Sign-in cancelled.".into());
            }

            match self.listener.accept() {
                Ok((mut stream, _)) => {
                    let _ = stream.set_nonblocking(false);
                    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
                    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

                    let mut reader = BufReader::new(&stream);
                    let mut request_line = String::new();
                    if reader.read_line(&mut request_line).is_ok() {
                        let parts: Vec<&str> = request_line.split_whitespace().collect();
                        if parts.len() >= 2 && parts[0] == "GET" {
                            let path = parts[1];
                            let dummy_url = format!("http://127.0.0.1{path}");
                            if let Ok(parsed) = Url::parse(&dummy_url) {
                                let query: std::collections::HashMap<_, _> =
                                    parsed.query_pairs().collect();

                                if let Some(err) = query.get("error") {
                                    let desc = query
                                        .get("error_description")
                                        .map(|s| s.as_ref())
                                        .unwrap_or(err.as_ref());
                                    send_response(
                                        &mut stream,
                                        false,
                                        &format!("Sign-in refused by NSGO: {desc}"),
                                    );
                                    return Err(format!("NSGO sign-in refused: {desc}"));
                                }

                                if let (Some(code), Some(state)) =
                                    (query.get("code"), query.get("state"))
                                {
                                    if state.as_ref() == self.state {
                                        send_response(
                                            &mut stream,
                                            true,
                                            "You can now close this tab and return to Nshorizon.",
                                        );
                                        return Ok(code.to_string());
                                    } else {
                                        send_response(
                                            &mut stream,
                                            false,
                                            "State mismatch during OAuth verification.",
                                        );
                                        return Err(
                                            "OAuth state mismatch. Security verification failed."
                                                .into(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(e) => return Err(format!("Error accepting loopback connection: {e}")),
            }
        }

        Err("Sign-in timed out waiting for browser completion.".into())
    }
}

fn send_response(stream: &mut TcpStream, success: bool, message: &str) {
    let title = if success {
        "Signed in to NSGO"
    } else {
        "Sign-in Failed"
    };
    let color = if success { "#4ade80" } else { "#f87171" };

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>{title}</title>
<style>
  body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0f1117; color: #fff; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; margin: 0; }}
  .card {{ background: rgba(255,255,255,0.05); border: 1px solid rgba(255,255,255,0.1); padding: 32px 48px; border-radius: 16px; text-align: center; max-width: 420px; }}
  h1 {{ margin: 0 0 12px 0; font-size: 24px; color: {color}; }}
  p {{ color: #94a3b8; margin: 0; font-size: 15px; line-height: 1.5; }}
</style>
</head>
<body>
  <div class="card">
    <h1>{title}</h1>
    <p>{message}</p>
  </div>
</body>
</html>"#
    );

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        html.len(),
        html
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
}

fn exchange_code(code: &str, verifier: &str, port: u16) -> Result<Tokens, String> {
    let redirect_uri = format!("http://127.0.0.1:{port}/oauth/nsgo");
    let url = token_url();

    let res = request_to("POST", &url)
        .send_form(&[
            ("grant_type", "authorization_code"),
            ("client_id", CLIENT_ID),
            ("code", code),
            ("redirect_uri", &redirect_uri),
            ("code_verifier", verifier),
        ])
        .map_err(|e| format!("Failed to exchange authorization code: {e}"))?;

    let body = res
        .into_string()
        .map_err(|e| format!("Failed to read token response: {e}"))?;

    let val: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| format!("Token response is not valid JSON: {e}"))?;

    if let Some(err) = val.get("error").and_then(|v| v.as_str()) {
        let desc = val
            .get("error_description")
            .and_then(|v| v.as_str())
            .unwrap_or(err);
        return Err(format!("NSGO token error: {desc}"));
    }

    let access_token = val
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing access_token in token response".to_string())?
        .to_string();

    let refresh_token = val
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let expires_in = val.get("expires_in").and_then(|v| v.as_u64());

    Ok(Tokens {
        access_token,
        refresh_token,
        expires_in,
    })
}

pub fn refresh_tokens(refresh_token: &str) -> Result<(String, Option<String>, Option<u64>), String> {
    let url = token_url();
    let res = request_to("POST", &url)
        .send_form(&[
            ("grant_type", "refresh_token"),
            ("client_id", CLIENT_ID),
            ("refresh_token", refresh_token),
        ])
        .map_err(|e| format!("Failed to refresh NSGO token: {e}"))?;

    let body = res
        .into_string()
        .map_err(|e| format!("Failed to read refresh token response: {e}"))?;

    let val: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| format!("Token response is not valid JSON: {e}"))?;

    if let Some(err) = val.get("error").and_then(|v| v.as_str()) {
        let desc = val
            .get("error_description")
            .and_then(|v| v.as_str())
            .unwrap_or(err);
        return Err(format!("NSGO token refresh error: {desc}"));
    }

    let access_token = val
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing access_token in refresh response".to_string())?
        .to_string();

    let new_refresh_token = val
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let expires_in = val.get("expires_in").and_then(|v| v.as_u64());

    Ok((access_token, new_refresh_token, expires_in))
}
