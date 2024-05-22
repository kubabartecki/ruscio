#[macro_use] extern crate rocket;

mod controllers;
mod models;
mod services;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .register("/", catchers![
        controllers::error_controller::not_found,
        controllers::error_controller::bad_input
    ])
    .mount("/", routes![index])
    .mount("/", routes![
        controllers::aes_controller::encrypt,
        controllers::aes_controller::decrypt,
        controllers::rsa_controller::generate_keys,
        controllers::rsa_controller::encrypt,
        controllers::rsa_controller::decrypt,
        controllers::sha_controller::hash,
    ])
}
