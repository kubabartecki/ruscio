pub struct RsaKey {
    pub public_key: String,
    pub private_key: String,
}

pub struct RsaPlaintext {
    pub data: Vec<u8>,
}

pub struct RsaCiphertext {
    pub data: Vec<u8>,
}
