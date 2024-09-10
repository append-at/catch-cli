use base64::Engine;
use catch_cli::code_reader::find_and_read_files;
use std::path::Path;
use tempfile::TempDir;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn test_find_and_read_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    create_test_files(temp_path).await?;

    let encryption_key = rand::random::<[u8; 32]>();
    let iv = rand::random::<[u8; 16]>();

    let files = find_and_read_files(temp_path, &encryption_key, &iv).await?;

    assert_eq!(files.len(), 3, "Expected to find 3 files");

    let file_names: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
    assert!(file_names.contains(&"test.js"));
    assert!(file_names.contains(&"subfolder/test.py"));
    assert!(file_names.contains(&"AndroidManifest.xml"));

    for file in &files {
        assert_ne!(file.content, "Test content");

        assert!(base64::engine::general_purpose::STANDARD
            .decode(&file.content)
            .is_ok());
    }

    Ok(())
}

async fn create_test_files(temp_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let js_path = temp_path.join("test.js");
    let mut js_file = fs::File::create(js_path).await?;
    js_file.write_all(b"Test content").await?;

    let subfolder_path = temp_path.join("subfolder");
    fs::create_dir(&subfolder_path).await?;
    let py_path = subfolder_path.join("test.py");
    let mut py_file = fs::File::create(py_path).await?;
    py_file.write_all(b"Test content").await?;

    // AndroidManifest.xml
    let manifest_path = temp_path.join("AndroidManifest.xml");
    let mut manifest_file = fs::File::create(manifest_path).await?;
    manifest_file.write_all(b"Test content").await?;

    let txt_path = temp_path.join("test.txt");
    let mut txt_file = fs::File::create(txt_path).await?;
    txt_file.write_all(b"This should be ignored").await?;

    Ok(())
}
