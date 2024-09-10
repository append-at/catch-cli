use crate::cryptography::encrypt_aes_256;
use base64::engine::general_purpose;
use base64::Engine;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use tokio::fs;
use tokio::io;

#[derive(Debug, Clone)]
pub struct CatchCLICodeFile {
    pub path: String,
    pub encrypted_file_content: String,
}

fn is_whitelisted(file_name: &str) -> bool {
    let whitelist = [
        "*.js",
        "*.ts",
        "*.py",
        "*.java",
        "*.kt",
        "*.swift",
        "*.m",
        "*.mm",
        "*.gradle",
        "*.kts",
        "*.toml",
        "AndroidManifest.xml",
        "Podfile",
        "*.entitlements",
        "*.plist",
        "*.xcprivacy",
    ];

    for &pattern in &whitelist {
        if pattern.starts_with('*') {
            let extension = pattern.strip_prefix('*').unwrap_or("");
            if file_name.ends_with(extension) {
                return true;
            }
        } else if file_name == pattern {
            return true;
        }
    }
    false
}

fn visit_dirs<'a>(
    encryption_key: &'a [u8; 32],
    encryption_iv: &'a [u8; 16],
    base_dir: &'a Path,
    dir: &'a Path,
    result: &'a mut Vec<CatchCLICodeFile>,
) -> Pin<Box<dyn Future<Output = io::Result<()>> + 'a>> {
    Box::pin(async move {
        let mut entries = fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(encryption_key, encryption_iv, base_dir, &path, result).await?;
            } else if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if is_whitelisted(file_name_str) {
                        let relative_path = path
                            .strip_prefix(base_dir)
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                        let content = fs::read_to_string(&path).await?;
                        let encrypted_content = general_purpose::STANDARD.encode(encrypt_aes_256(
                            encryption_key,
                            encryption_iv,
                            &content,
                        ));

                        result.push(CatchCLICodeFile {
                            path: relative_path.to_string_lossy().into_owned(),
                            encrypted_file_content: encrypted_content,
                        });
                    }
                }
            }
        }
        Ok(())
    })
}

pub async fn find_and_read_files(
    dir: &Path,
    encryption_key: &[u8; 32],
    encryption_iv: &[u8; 16],
) -> io::Result<Vec<CatchCLICodeFile>> {
    let mut result = Vec::new();
    visit_dirs(encryption_key, encryption_iv, dir, dir, &mut result).await?;
    Ok(result)
}
