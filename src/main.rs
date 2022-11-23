#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use ::webfinger::{Link, Webfinger};
use rocket::{
    http::{ContentType, Header, Status},
    response::content::RawJson,
    serde::json::Json,
};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref DOMAIN: &'static str = "xn--5bicd.fly.dev";
    static ref ACCOUNT_URL: &'static str = "acct:referee@xn--5bicd.fly.dev";
    static ref ACTOR_URL: &'static str = "https://xn--5bicd.fly.dev/@referee";
    static ref AS_CONTEXT: &'static str = "https://www.w3.org/ns/activitystreams";
    static ref SEC_CONTEXT: &'static str = "https://w3id.org/security/v1";
    static ref PUBLIC_KEY: &'static str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAnUphqhdcFnFAX7WRUo9b
WJkKYhJCURMib82QGnQUCCy65h0+/FglkkQEiCGV0QfQq9dJgwhDxXGF4E/bq1qu
1VmAIf6/7JGahPcwxyaqyDHfj4rCMkBW9QPTim8ptwGHuJh0t+95BmO/uKLwDMF5
7fD6k1f36DYJHvrPtB2wEM+3oX8gywzKn+bYPC40iiA3Rtwy+BXL4vH5w31CZ/iX
HUUtvIm0HlzzxfYI/ySIFjpesTZ5V5JBr9dqL6X5tRLtg3XUkvz2fCQnzF0TMr+O
dA6XZPOg780gFlcUb5iWAGG5aXcjjtzjwEQFwgrx2lSQqpiAWowF+s1m9/j3BiW6
OwIDAQAB
-----END PUBLIC KEY-----"#;
}

#[launch]
async fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "0.0.0.0"))
        .merge(("log_level", rocket::config::LogLevel::Debug));

    rocket::custom(figment).mount("/", routes![ping, webfinger, referee])
}

#[get("/ping")]
fn ping() -> RawJson<&'static str> {
    RawJson(
        r#"{
    "pong": true  
}"#,
    )
}

#[derive(Responder)]
pub enum WebfingerApiResponse<T> {
    #[response(status = 200, content_type = "application/jrd+json")]
    Ok(T),
    #[response(status = 404)]
    NotFound(String),
}

#[get("/.well-known/webfinger?<resource>")]
pub fn webfinger(resource: String) -> WebfingerApiResponse<Json<Webfinger>> {
    info!("{:?}", resource);
    let valid_queries = vec![
        "referee".to_string(),
        format!("referee@{}", DOMAIN.to_string()),
        "acct:referee".to_string(),
        format!("acct:referee@{}", DOMAIN.to_string()),
    ];
    if valid_queries.contains(&resource) {
        return WebfingerApiResponse::Ok(Json(Webfinger {
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
    WebfingerApiResponse::NotFound("Not Found".to_string())
}

#[derive(Serialize, Deserialize)]
pub struct KeyInformation {
    id: String,
    owner: String,
    #[serde(rename = "publicKeyPem")]
    public_key_pem: String,
}

#[derive(Serialize, Deserialize)]
pub struct Actor {
    #[serde(rename = "@context")]
    context: Vec<String>,
    id: String,
    #[serde(rename = "type")]
    actor_type: String,
    name: String,
    summary: String,
    #[serde(rename = "preferredUsername")]
    preferred_username: String,
    inbox: String,
    outbox: String,
    #[serde(rename = "publicKey")]
    public_key: KeyInformation,
}

fn referee_profile() -> Actor {
    Actor {
        context: vec![AS_CONTEXT.to_string(), SEC_CONTEXT.to_string()],
        id: ACTOR_URL.to_string(),
        actor_type: "Service".to_string(),
        name: "Referee".to_string(),
        summary: "I'm a bot, hosting rock-paper-scissor games!".to_string(),
        preferred_username: "referee".to_string(),
        inbox: format!("{}/inbox", DOMAIN.to_string()),
        outbox: format!("{}/outbox", DOMAIN.to_string()),
        public_key: KeyInformation {
            id: format!("{}#main-key", ACTOR_URL.to_string()),
            owner: ACTOR_URL.to_string(),
            public_key_pem: PUBLIC_KEY.to_string(),
        },
    }
}

#[get("/@referee")]
pub fn referee() -> Result<Json<Actor>, Status> {
    Ok(referee_profile().into())
}
