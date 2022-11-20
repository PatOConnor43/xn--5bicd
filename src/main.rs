#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

use rocket::{
    http::ext::IntoOwned,
    request::{FromRequest, Outcome},
    response::content::RawJson,
    Request, Rocket,
};

#[launch]
async fn rocket() -> _ {
    // env_logger::init();

    Rocket::build().mount("/", routes!(ping,))
}

#[get("/ping")]
fn ping() -> RawJson<&'static str> {
    RawJson(
        r#"{
    "pong": true  
}"#,
    )
}

#[get("/.well-known/webfinger?<resource>")]
fn finger() -> RawJson<&'static str> {
    RawJson(r#"{}"#)
}
