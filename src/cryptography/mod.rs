use base64::Engine;
use libaes::Cipher;
use pkcs8::DecodePublicKey;
use rand::rngs::OsRng;
use rsa::sha2::Sha256;
use rsa::traits::PublicKeyParts;
use rsa::{Oaep, RsaPublicKey};

pub fn encrypt_aes_256(key: &[u8; 32], iv: &[u8; 16], message: &str) -> Vec<u8> {
    let cipher = Cipher::new_256(key);
    cipher.cbc_encrypt(iv, message.as_bytes())
}

pub fn encrypt_rsa4096_base64(
    public_key_pem: &str,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let encoded_message = base64::engine::general_purpose::STANDARD.encode(message);
    encrypt_rsa4096_base64_internal(public_key_pem, encoded_message.as_bytes())
}

pub fn encrypt_rsa4096_base64_bytes(
    public_key_pem: &str,
    message: &[u8],
) -> Result<String, Box<dyn std::error::Error>> {
    encrypt_rsa4096_base64_internal(public_key_pem, message)
}

fn encrypt_rsa4096_base64_internal(
    public_key_pem: &str,
    encoded_message: &[u8],
) -> Result<String, Box<dyn std::error::Error>> {
    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)?;

    let key_size = public_key.size();
    if key_size != 512 {
        let err_msg = format!(
            "Invalid RSA-4096 key. Current key size: {} bits",
            key_size * 8
        );
        log::error!("{}", err_msg);
        return Err(err_msg.into());
    }

    let padding = Oaep::new::<Sha256>();
    let mut rng = OsRng;
    let enc_data = public_key.encrypt(&mut rng, padding, encoded_message)?;

    Ok(base64::engine::general_purpose::STANDARD.encode(enc_data))
}
