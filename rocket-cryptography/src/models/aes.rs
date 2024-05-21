pub struct AesKey<'a> {
    pub key_bytes: &'a [u8], // AES128 key is 128 bits (16 bytes)
}

pub struct AesPlaintext {
    pub data: String, // Plaintext data to be encrypted
}

pub struct AesCiphertext {
    pub encoded_data: String, // Encrypted ciphertext data
}
