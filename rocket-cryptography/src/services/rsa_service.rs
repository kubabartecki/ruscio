use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
    pkcs1::EncodeRsaPrivateKey, pkcs1::EncodeRsaPublicKey, pkcs1::LineEnding,
    pkcs1::DecodeRsaPrivateKey, pkcs1::DecodeRsaPublicKey,
};
use crate::models::{RsaPlaintext, RsaCiphertext, RsaKey};

pub struct RsaService;

impl RsaService {
    pub fn generate_keys() -> RsaKey {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        let priv_key_pem: String = priv_key.to_pkcs1_pem(pkcs1::LineEnding::LF).expect("failed to encode private key").to_string();
        let pub_key_pem: String = pub_key.to_pkcs1_pem(pkcs1::LineEnding::LF).expect("failed to encode public key").to_string();

        RsaKey {
            public_key: pub_key_pem,
            private_key: priv_key_pem,
        }
    }

    pub fn encrypt(plaintext: RsaPlaintext, public_key_pem: &str) -> RsaCiphertext {
        let pub_key = RsaPublicKey::from_pkcs1_pem(&public_key_pem).expect("failed to parse public key");

        let mut rng = rand::thread_rng();
        let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &plaintext.data).expect("failed to encrypt");
        RsaCiphertext { data: enc_data }
    }

    pub fn decrypt(ciphertext: RsaCiphertext, private_key_pem: &str) -> RsaPlaintext {
        let priv_key = RsaPrivateKey::from_pkcs1_pem(&private_key_pem).expect("failed to parse private key");

        let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &ciphertext.data).expect("failed to decrypt");
        RsaPlaintext { data: dec_data }
    }
}
