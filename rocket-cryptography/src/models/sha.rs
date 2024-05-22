use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct ShaRequest {
    pub input: String,
}

#[derive(Serialize)]
pub struct ShaResponse {
    pub hash: String,
}
