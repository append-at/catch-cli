use catch_cli::cryptography::encrypt_rsa4096_base64;
use rand::rngs::OsRng;
use rsa::{pkcs8::EncodePublicKey, RsaPrivateKey, RsaPublicKey};

fn generate_rsa_key_pair() -> (RsaPrivateKey, String) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 4096).expect("Failed to generate RSA key");
    let public_key = RsaPublicKey::from(&private_key);
    let public_key_pem = public_key
        .to_public_key_pem(pkcs8::LineEnding::LF)
        .expect("Failed to encode public key to PEM");
    (private_key, public_key_pem)
}

#[test]
fn test_encrypt_rsa4096_base64_valid_input() {
    let (_, public_key_pem) = generate_rsa_key_pair();
    let message = "Hello, World!";

    let result = encrypt_rsa4096_base64(&public_key_pem, message);
    assert!(result.is_ok(), "Encryption should succeed with valid input");

    let encrypted = result.unwrap();
    assert!(
        !encrypted.is_empty(),
        "Encrypted message should not be empty"
    );
    assert_ne!(
        encrypted, message,
        "Encrypted message should differ from original"
    );
}

#[test]
fn test_encrypt_rsa4096_base64_invalid_key() {
    let invalid_key = "-----BEGIN PUBLIC KEY-----\nInvalidKeyData\n-----END PUBLIC KEY-----";
    let message = "Hello, World!";

    let result = encrypt_rsa4096_base64(invalid_key, message);
    assert!(result.is_err(), "Encryption should fail with invalid key");
}

#[test]
fn test_encrypt_rsa4096_base64_wrong_key_size() {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate RSA key");
    let public_key = RsaPublicKey::from(&private_key);
    let public_key_pem = public_key
        .to_public_key_pem(pkcs8::LineEnding::LF)
        .expect("Failed to encode public key to PEM");

    let message = "Hello, World!";

    let result = encrypt_rsa4096_base64(&public_key_pem, message);
    assert!(
        result.is_err(),
        "Encryption should fail with wrong key size"
    );
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid RSA-4096 key"));
}

#[test]
fn test_encrypt_rsa4096_base64_empty_message() {
    let (_, public_key_pem) = generate_rsa_key_pair();
    let message = "";

    let result = encrypt_rsa4096_base64(&public_key_pem, message);
    assert!(
        result.is_ok(),
        "Encryption should succeed with empty message"
    );
}
