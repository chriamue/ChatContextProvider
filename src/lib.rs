use rocket::get;

#[get("/")]
pub fn world() -> &'static str {
    "Hello, world!"
}
