#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use ::webfinger::Webfinger;
use rocket::{http::Status, response::content::RawJson, serde::json::Json};

#[launch]
async fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment).mount("/", routes![ping, webfinger])
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
pub fn webfinger(resource: String) -> Result<Json<Webfinger>, Status> {
    let webfinger_subject = "acct:@referee@xn--5bicd.fly.dev".to_string();
    if resource != webfinger_subject {
        return Err(Status::NotFound);
    }
    Ok(Json(Webfinger {
        subject: webfinger_subject,
        aliases: vec![],
        links: vec![],
    }))
}
