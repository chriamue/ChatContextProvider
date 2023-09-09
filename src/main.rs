#[macro_use] extern crate rocket;
use ccp::world;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world])
}