use rocket::response::status::Custom;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::http::Status;

use crate::models::{AesPlaintext, AesKey, AesCiphertext};
use crate::services::aes_service::AesService;

#[derive(Deserialize)]
struct AesEncryptRequest {
    plaintext: String,
    key: String,
}

#[derive(Serialize)]
struct AesEncryptResponse {
    ciphertext: String,
}

#[post("/aes/encrypt", format = "json", data = "<request>")]
pub fn encrypt(request: Json<AesEncryptRequest>) -> Result<Json<AesEncryptResponse>, Custom<String>> {
    if request.key.len() != 16 {
        return Err(Custom(Status::BadRequest, "Key must be a string of length 16 (Aes128)".to_string()));
    }

    let plaintext = AesPlaintext { data: request.plaintext.clone() };
    let key = AesKey { key_bytes: &request.key.as_bytes() };
    let ciphertext = AesService::encrypt(plaintext, key);

    Ok(Json(AesEncryptResponse {
        ciphertext: ciphertext.encoded_data,
    }))
}

#[derive(Deserialize)]
struct AesDecryptRequest {
    ciphertext: String,
    key: String,
}

#[derive(Serialize)]
struct AesDecryptResponse {
    plaintext: String,
}

#[post("/aes/decrypt", format = "json", data = "<request>")]
pub fn decrypt(request: Json<AesDecryptRequest>) -> Result<Json<AesDecryptResponse>, Custom<String>> {
    if request.key.len() != 16 {
        return Err(Custom(Status::BadRequest, "Key must be a string of length 16 (Aes128)".to_string()));
    }

    let ciphertext = AesCiphertext { encoded_data: request.ciphertext.clone() };
    let key = AesKey { key_bytes: request.key.as_bytes() };

    let plaintext = AesService::decrypt(ciphertext, key);

    Ok(Json(AesDecryptResponse {
        plaintext: plaintext.data,
    }))
}
