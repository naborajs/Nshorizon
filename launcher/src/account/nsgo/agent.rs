//! Pinned authlib-injector Java agent for NSGO Yggdrasil authentication.
use sha2::{Digest, Sha256};
use std::path::Path;

const INJECTOR_URL: &str =
    "https://authlib-injector.yushi.moe/artifact/56/authlib-injector-1.2.8.jar";
const INJECTOR_SHA256: &str = "9c7f4343e6c82034958ffb48c14a2cb0c85928be7283103ce17da00c6d5a7b10";

pub fn java_agent(dir: &Path) -> Result<String, String> {
    let path = dir.join("authlib-injector-1.2.8.jar");
    let valid = |bytes: &[u8]| format!("{:x}", Sha256::digest(bytes)) == INJECTOR_SHA256;

    if !std::fs::read(&path).is_ok_and(|bytes| valid(&bytes)) {
        let bytes = crate::backend::get_bytes(INJECTOR_URL)?;
        if !valid(&bytes) {
            return Err("The authlib-injector agent failed its SHA-256 checksum check.".into());
        }

        std::fs::create_dir_all(dir)
            .map_err(|e| format!("Could not create authlib-injector directory: {e}"))?;

        let mut random = [0u8; 12];
        getrandom::fill(&mut random)
            .map_err(|e| format!("Could not generate secure temporary filename: {e}"))?;
        let suffix: String = random.iter().map(|b| format!("{b:02x}")).collect();
        let temporary = dir.join(format!("authlib-injector-{suffix}.tmp"));

        std::fs::write(&temporary, &bytes)
            .map_err(|e| format!("Could not save authlib-injector agent: {e}"))?;

        if let Err(e) = std::fs::rename(&temporary, &path) {
            let _ = std::fs::remove_file(&temporary);
            if !std::fs::read(&path).is_ok_and(|bytes| valid(&bytes)) {
                return Err(format!("Could not finalize authlib-injector agent: {e}"));
            }
        }
    }

    Ok(format!(
        "-javaagent:{}={}",
        path.display(),
        super::yggdrasil_url()
    ))
}
