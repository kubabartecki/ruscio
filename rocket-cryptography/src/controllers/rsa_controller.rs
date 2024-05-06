use rocket::post;
use rocket::response::status::Custom;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Deserialize)]
struct RsaEncryptRequest {
    plaintext: String,
    public_key: String,
}

#[derive(Serialize)]
struct RsaEncryptResponse {
    ciphertext: String,
}

#[post("/rsa/encrypt", format = "json", data = "<request>")]
pub fn encrypt(request: Json<RsaEncryptRequest>) -> Result<Json<RsaEncryptResponse>, Custom<String>> {
    Ok(Json(RsaEncryptResponse {
        ciphertext: "RSA encryption logic here".to_string(),
    }))
}

#[derive(Deserialize)]
struct RsaDecryptRequest {
    ciphertext: String,
    public_key: String,
}

#[derive(serde::Serialize)]
struct RsaDecryptResponse {
    plaintext: String,
}

#[post("/rsa/decrypt", format = "json", data = "<request>")]
pub fn decrypt(request: Json<RsaDecryptRequest>) -> Result<Json<RsaDecryptResponse>, Custom<String>> {
    Ok(Json(RsaDecryptResponse {
        plaintext: "RSA decryption logic here".to_string(),
    }))
}
