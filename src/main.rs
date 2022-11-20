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

    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment).mount("/", routes![ping,])
}

#[get("/ping")]
fn ping() -> RawJson<&'static str> {
    RawJson(
        r#"{
    "pong": true  
}"#,
    )
}
