use sha2::{Sha256, Digest};
use crate::models::{ShaRequest, ShaResponse};

pub struct ShaService;

impl ShaService {
    pub fn hash(input: ShaRequest) -> ShaResponse {
        let mut hasher = Sha256::new();
        hasher.update(input.input.as_bytes());
        let result = hasher.finalize();
        ShaResponse {
            hash: format!("{:x}", result),
        }
    }
}
