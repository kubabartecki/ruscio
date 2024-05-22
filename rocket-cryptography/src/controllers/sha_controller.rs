use rocket::serde::{json::Json};
use crate::services::sha_service::ShaService;
use crate::models::{ShaRequest, ShaResponse};

#[post("/sha256/hash", format = "json", data = "<request>")]
pub async fn hash(request: Json<ShaRequest>) -> Json<ShaResponse> {
    let response = ShaService::hash(request.into_inner());
    Json(response)
}
