use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey}};
use base64::{decode, encode};


pub trait RsaEncryption {
    fn generate_keys() -> (String, String);
    fn encrypt(&self, public_key: &str, message: &str) -> String;
    fn decrypt(&self, private_key: &str, message: &str) -> String;
}

pub struct RsaEncryptionImpl;

impl RsaEncryption for RsaEncryptionImpl {
    fn generate_keys() -> (String, String) {
        let mut rng = OsRng;
        let bits = 4096;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);

        let private_key_pem = private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).expect("failed to encode private key").to_string();
        let public_key_pem = public_key.to_public_key_pem(rsa::pkcs8::LineEnding::LF).expect("failed to encode public key").to_string();

        (private_key_pem, public_key_pem)
    }

    fn encrypt(&self, public_key: &str, message: &str) -> String {
        let public_key = RsaPublicKey::from_public_key_pem(public_key).expect("failed to decode public key");
        let encrypted = public_key.encrypt(&mut OsRng, rsa::pkcs1v15::Pkcs1v15Encrypt, message.as_bytes()).expect("failed to encrypt");

        encode(&encrypted)
    }

    fn decrypt(&self, private_key: &str, message: &str) -> String {
        let private_key = RsaPrivateKey::from_pkcs8_pem(private_key).expect("failed to decode private key");
        let decoded_message = decode(message).expect("failed to decode base64 message");
        let decrypted = private_key.decrypt(rsa::pkcs1v15::Pkcs1v15Encrypt, &decoded_message).expect("failed to decrypt");

        String::from_utf8(decrypted).expect("failed to convert decrypted bytes to string")
    }
}
