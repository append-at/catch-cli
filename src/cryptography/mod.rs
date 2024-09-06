use base64::Engine;
use log::error;
use pkcs8::DecodePublicKey;
use rand::rngs::OsRng;
use rsa::traits::PublicKeyParts;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

pub fn encrypt_rsa4096_base64(
    public_key_pem: &str,
    message: &str,
) -> Result<String, Box<dyn error::Error>> {
    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)?;

    let key_size = public_key.size();
    if key_size != 512 {
        let err_msg = format!(
            "Invalid RSA-4096 key. Current key size: {} bits",
            key_size * 8
        );
        error!("{}", err_msg);
        return Err(err_msg.into());
    }

    let encoded_message = base64::engine::general_purpose::STANDARD.encode(message);

    let mut rng = OsRng;
    let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, encoded_message.as_bytes())?;

    Ok(base64::engine::general_purpose::STANDARD.encode(enc_data))
}
