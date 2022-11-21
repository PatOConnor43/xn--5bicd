#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use ::webfinger::{Link, Webfinger};
use rocket::{http::Status, response::content::RawJson, serde::json::Json};

lazy_static! {
    static ref DOMAIN: &'static str = "xn--5bicd.fly.dev";
    static ref ACCOUNT_URL: &'static str = "acct:referee@xn--5bicd.fly.dev";
    static ref ACTOR_URL: &'static str = "https://xn--5bicd.fly.dev/@referee";
}

#[launch]
async fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "0.0.0.0"))
        .merge(("log_level", rocket::config::LogLevel::Debug));

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
    info!("{:?}", resource);
    let valid_queries = vec![
        "referee".to_string(),
        format!("referee@{}", DOMAIN.to_string()),
        "acct:referee".to_string(),
        format!("acct:referee@{}", DOMAIN.to_string()),
    ];
    if valid_queries.contains(&resource) {
        return Ok(Json(Webfinger {
            subject: ACCOUNT_URL.to_string(),
            aliases: vec![],
            links: vec![Link {
                rel: "self".to_string(),
                href: Some(ACTOR_URL.to_string()),
                template: None,
                mime_type: Some("application/ld+json".to_string()),
            }],
        }));
    }
    Err(Status::NotFound)
}
