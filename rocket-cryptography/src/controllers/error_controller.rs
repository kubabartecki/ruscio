use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[catch(422)]
pub fn bad_input() -> String {
    "Oh no! Content you provided is unprocessable".to_string()
}
