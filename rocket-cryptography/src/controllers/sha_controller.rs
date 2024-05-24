use rocket::serde::{json::Json};
use rocket::response::status::Custom;
use rocket::http::Status;
use crate::services::sha_service::ShaService;
use crate::models::{ShaRequest, ShaResponse, HashRequest, Hash};

#[post("/sha256/hash", format = "json", data = "<request>")]
pub async fn hash(request: Json<ShaRequest>) -> Json<ShaResponse> {
    let response = ShaService::hash(request.into_inner());
    Json(response)
}

#[post("/sha256/mine", format = "json", data = "<request>")]
pub async fn find_hex_digits(request: Json<HashRequest>) -> Custom<Json<i32>> {
    let time_taken = ShaService::find_hex_digits(request.num_digits);
    Custom(Status::Ok, Json(time_taken))
}

#[get("/sha256/hashes?<num_digits>")]
pub async fn get_hashes_by_digits(num_digits: i32) -> Custom<Json<Vec<Hash>>> {
    let hashes = ShaService::get_hashes_by_digits(num_digits);
    Custom(Status::Ok, Json(hashes))
}
