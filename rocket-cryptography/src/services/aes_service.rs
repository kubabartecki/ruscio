use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes128Gcm, Key, Nonce
};

use crate::models::{AesKey, AesPlaintext, AesCiphertext};

pub struct AesService;

impl AesService {
    pub fn encrypt(plaintext: AesPlaintext, key: AesKey) -> AesCiphertext {
        let plaintext_bytes = plaintext.data.as_bytes();
        let key = Key::<Aes128Gcm>::from_slice(key.key_bytes);
        let nonce = Aes128Gcm::generate_nonce(&mut OsRng);

        let cipher = Aes128Gcm::new(key);

        let ciphered_data = cipher.encrypt(&nonce, plaintext_bytes)
            .expect("failed to encrypt");
        
        // combining nonce and encrypted data together
        // for storage purpose
        let mut encrypted_data: Vec<u8> = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphered_data);
        
        let hex_data: String = hex::encode(encrypted_data);
        AesCiphertext { encoded_data: hex_data }
    }

    pub fn decrypt(ciphertext: AesCiphertext, key: AesKey) -> AesPlaintext {
        let encrypted_data = hex::decode(ciphertext.encoded_data)
            .expect("failed to decode hex string into vec");
        let key = Key::<Aes128Gcm>::from_slice(key.key_bytes);

        let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_arr);

        let cipher = Aes128Gcm::new(key);

        let plaintext_decrypted = cipher.decrypt(nonce, ciphered_data)
            .expect("failed to decrypt data");

        let plaintext = String::from_utf8(plaintext_decrypted)
            .expect("failed to convert vector of bytes to string");

        AesPlaintext { data: plaintext }
    }
}
