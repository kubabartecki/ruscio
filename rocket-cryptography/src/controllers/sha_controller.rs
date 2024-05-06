use rocket::response::status::Custom;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Deserialize)]
struct ShaInput {
    input: String,
}

#[derive(Serialize)]
struct ShaOutput {
    hash: String,
}

#[post("/sha256/hash", format = "json", data = "<input>")]
pub fn hash(input: Json<ShaInput>) -> Result<Json<ShaOutput>, Custom<String>> {
    Ok(Json(ShaOutput {
        hash: "hash".to_string(),
    }))
}
