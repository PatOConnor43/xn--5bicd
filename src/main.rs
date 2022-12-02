#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use ::webfinger::{Link, Webfinger};
use rocket::{
    fairing::AdHoc, figment::providers::Serialized, http::Status, response::content::RawJson,
    serde::json::Json, State,
};
use serde::{Deserialize, Serialize};

lazy_static! {
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

fn subject_template(domain: &str) -> String {
    format!("acct:referee@{}", domain)
}
fn actor_url_template(scheme: &str, domain: &str) -> String {
    format!("{}{}/@referee", scheme, domain)
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    domain: String,
    scheme: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            domain: "xn--5bicd.fly.dev".to_string(),
            scheme: "https://".to_string(),
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(Serialized::defaults(Config::default()));

    let mut r = rocket::custom(figment)
        .mount("/", routes![ping, webfinger, referee])
        .attach(AdHoc::config::<Config>());

    if r.figment().profile() == "debug" {
        r = r.mount("/meta", routes![config]);
    }

    r
}

#[get("/ping")]
fn ping() -> RawJson<&'static str> {
    RawJson(
        r#"{
    "pong": true  
}"#,
    )
}

#[get("/config")]
fn config(config: &State<Config>) -> Json<&Config> {
    Json(config.inner())
}

#[derive(Responder)]
pub enum WebfingerApiResponse<T> {
    #[response(status = 200, content_type = "application/jrd+json")]
    Ok(T),
    #[response(status = 404)]
    NotFound(String),
}

#[get("/.well-known/webfinger?<resource>")]
pub fn webfinger(
    config: &State<Config>,
    resource: String,
) -> WebfingerApiResponse<Json<Webfinger>> {
    info!("{:?}", resource);
    let domain = &config.domain;
    let scheme = &config.scheme;
    let valid_queries = vec![
        "referee".to_string(),
        format!("referee@{}", domain.to_string()),
        "acct:referee".to_string(),
        format!("acct:referee@{}", domain.to_string()),
    ];
    if valid_queries.contains(&resource) {
        return WebfingerApiResponse::Ok(Json(Webfinger {
            subject: subject_template(domain),
            aliases: vec![],
            links: vec![Link {
                rel: "self".to_string(),
                href: Some(actor_url_template(scheme, domain)),
                template: None,
                mime_type: Some(
                    "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\""
                        .to_string(),
                ),
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
    #[serde(rename = "publicKey")]
    public_key: KeyInformation,
}

fn referee_profile(scheme: &str, domain: &str) -> Actor {
    Actor {
        context: vec![AS_CONTEXT.to_string(), SEC_CONTEXT.to_string()],
        id: actor_url_template(scheme, domain),
        actor_type: "Service".to_string(),
        name: "Referee".to_string(),
        summary: "I'm a bot, hosting rock-paper-scissor games!".to_string(),
        preferred_username: "referee".to_string(),
        inbox: format!("{}/inbox", domain),
        public_key: KeyInformation {
            id: format!("{}#public_key", actor_url_template(scheme, domain)),
            owner: actor_url_template(scheme, domain),
            public_key_pem: PUBLIC_KEY.to_string(),
        },
    }
}

#[get("/@referee")]
pub fn referee(config: &State<Config>) -> Result<Json<Actor>, Status> {
    let domain = &config.domain;
    let scheme = &config.scheme;
    Ok(referee_profile(scheme, domain).into())
}
