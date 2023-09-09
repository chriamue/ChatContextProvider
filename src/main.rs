#[macro_use]
extern crate rocket;
use ccp::{assets, world};

use rocket_okapi::{openapi_get_routes, swagger_ui::*};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![assets, world])
        .mount("/", openapi_get_routes![ccp::current_project])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
