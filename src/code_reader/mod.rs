use crate::cryptography::encrypt_rsa4096_base64;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use tokio::fs;
use tokio::io;

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
            let extension = &pattern[1..]; // Remove the '*'
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
    public_key: &'a str,
    base_dir: &'a Path,
    dir: &'a Path,
    result: &'a mut Vec<CatchCLICodeFile>,
) -> Pin<Box<dyn Future<Output = io::Result<()>> + 'a>> {
    Box::pin(async move {
        let mut entries = fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(public_key, base_dir, &path, result).await?;
            } else {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if is_whitelisted(file_name_str) {
                            let relative_path = path
                                .strip_prefix(base_dir)
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                            let content = fs::read_to_string(&path).await?;
                            let encrypted_content = encrypt_rsa4096_base64(public_key, &content)
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                            result.push(CatchCLICodeFile {
                                path: relative_path.to_string_lossy().into_owned(),
                                encrypted_file_content: encrypted_content,
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    })
}

pub async fn find_and_read_files(
    dir: &Path,
    public_key: &str,
) -> io::Result<Vec<CatchCLICodeFile>> {
    let mut result = Vec::new();
    visit_dirs(public_key, &dir, &dir, &mut result).await?;
    Ok(result)
}
