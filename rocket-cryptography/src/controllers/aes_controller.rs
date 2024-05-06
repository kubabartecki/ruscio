use rocket::response::status::Custom;
use rocket::serde::{Deserialize, Serialize, json::Json};

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
    Ok(Json(AesEncryptResponse {
        ciphertext: "AES encryption logic here".to_string(),
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
    Ok(Json(AesDecryptResponse {
        plaintext: "AES decryption logic here".to_string(),
    }))
}
