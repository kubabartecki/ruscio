use rocket::response::status::Custom;
use rocket::serde::{Deserialize, Serialize, json::Json};
use crate::models::{RsaPlaintext, RsaCiphertext, RsaKey};
use crate::services::rsa_service::RsaService;

#[derive(Serialize)]
struct RsaKeyResponse {
    public_key: String,
    private_key: String,
}

#[post("/rsa/generate_keys")]
pub fn generate_keys() -> Json<RsaKeyResponse> {
    let rsa_key: RsaKey = RsaService::generate_keys();

    Json(RsaKeyResponse {
        public_key: rsa_key.public_key,
        private_key: rsa_key.private_key,
    })
}

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
    let plaintext_bytes = request.plaintext.as_bytes().to_vec();
    let plaintext = RsaPlaintext { data: plaintext_bytes };

    let ciphertext = RsaService::encrypt(plaintext, &request.public_key);
    let ciphertext_str = base64::encode(ciphertext.data);

    Ok(Json(RsaEncryptResponse {
        ciphertext: ciphertext_str,
    }))
}

#[derive(Deserialize)]
struct RsaDecryptRequest {
    ciphertext: String,
    private_key: String,
}

#[derive(Serialize)]
struct RsaDecryptResponse {
    plaintext: String,
}

#[post("/rsa/decrypt", format = "json", data = "<request>")]
pub fn decrypt(request: Json<RsaDecryptRequest>) -> Result<Json<RsaDecryptResponse>, Custom<String>> {
    let ciphertext_bytes = base64::decode(&request.ciphertext).unwrap();
    let ciphertext = RsaCiphertext { data: ciphertext_bytes };

    let plaintext = RsaService::decrypt(ciphertext, &request.private_key);
    let plaintext_str = String::from_utf8(plaintext.data).expect("Ciphertext is not valid UTF-8");

    Ok(Json(RsaDecryptResponse {
        plaintext: plaintext_str,
    }))
}
