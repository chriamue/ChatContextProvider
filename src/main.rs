#[macro_use] extern crate rocket;
use ccp::{assets, world};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![assets, world])
}