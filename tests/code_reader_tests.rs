use base64::Engine;
use catch_cli::code_reader::find_and_read_files;
use std::path::Path;
use tempfile::TempDir;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn test_find_and_read_files() -> Result<(), Box<dyn std::error::Error>> {
    let public_key = r"
-----BEGIN PUBLIC KEY-----
MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAx/DGjodxkibAtLPPRLX5
sZs5t+pQn+IsiY0fpz/UIMjuC6DQjSra/qML4UA00tmp+LbqU61tCUfdecn4h3FI
+uhGDZ9Gpu3kZcml3xKpbqNda1DTIue/xo5qggM9iu28QllLsuQuGDFsuwg0hpxi
o0RpB8dxnWq6iYireXgA+dydxrrUd+YJ6x7E3NKAy9UEzPDZ9qVnWDJmbKSyONPg
vufuIFb4i7x0AGjniEcI/FZHyisZtuQjAVzT4ViWG2t/malSXOoGHyRhJTWvQmL+
aj98uYITl3H4wS1FIUf2vwYubjDt9F37MpbVKQK6CBMcS8s3Bw+7F+I35KJynsdR
WHfNvEIvVHXJZ4iwcKhpU6/+bdRhGK+Vng+F5me4FIegqiar0MooRn4joC85qwiU
rxLrh1hgT7QjqD7QInRI22rC3PfXwHNh40t6rZUpnaATiuPXu4eQAVp4tmY+/r0s
UaYc64gdj1DqG9kJCwX/eFgvs2uC44VRIP8IjAjEUP/PfV5it0uoJlO7TO6eDKd2
D24DZOsRQJAPwG66Nr6L0tlt3/3QgrAnNAHQxyQo/0vrKj+nO0VVPdCqotQMsAcK
oncNmsM2bOtsNLEgToVyYdudTpf+dswcU8DXbDw+JYqfY/26wKHcOTYiFgi28FFz
vyL+EYxcktOXt4r1Mp39HX0CAwEAAQ==
-----END PUBLIC KEY-----"
        .trim();

    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    create_test_files(temp_path).await?;

    let files = find_and_read_files(temp_path, public_key).await?;

    // 결과 검증
    assert_eq!(files.len(), 3, "Expected to find 3 files");

    let file_names: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
    assert!(file_names.contains(&"test.js"));
    assert!(file_names.contains(&"subfolder/test.py"));
    assert!(file_names.contains(&"AndroidManifest.xml"));

    for file in &files {
        // 암호화된 내용이 원본 내용과 다른지 확인
        assert_ne!(file.encrypted_file_content, "Test content");

        // 암호화된 내용이 base64로 인코딩되어 있는지 확인
        assert!(base64::engine::general_purpose::STANDARD
            .decode(&file.encrypted_file_content)
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
