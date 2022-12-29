use axum::{ 
    routing::post,
    extract::{ Multipart, Extension },
    Router, Json, http::HeaderMap,
};

use serde::{ Deserialize };
use serde_json::{Value, json};
use sqlx::Row;
use uuid::Uuid;

use crate::Context;
use crate::repositories;

pub fn router() -> Router {
    Router::new()
        .route("/oauth/token", post(token))
}

#[derive(Deserialize)]
struct LoginInfo {
    client_id:      u8,
    client_secret:  String,
    username:       String,
    password:       String,
}

async fn parse_login(mut multipart: Multipart) -> LoginInfo {
    let mut login_info = LoginInfo {
        client_id:      0,
        client_secret:  String::new(),
        username:       String::new(),
        password:       String::new(),
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_owned();
        let value = field.text().await.unwrap().to_owned();
        
        log::debug!("{}: {}", name, value);

        match name.as_str() {
            "client_id" => login_info.client_id = value.parse().unwrap(),
            "client_secret" => login_info.client_secret = value,
            "username" => login_info.username = value,
            "password" => login_info.password = value,
            _ => {}
        }
    }

    login_info
}


async fn token(
    headers: HeaderMap,
    ctx: Extension<Context>, 
    multipart: Multipart,
) -> Json<Value> {
    assert_eq!(headers.get("User-Agent").unwrap(), "osu!");

    let login_info = parse_login(multipart).await;

    // osu!lazer has a static client id & secret
    assert_eq!(login_info.client_id, 5);
    assert!([
        "FGc9GAtyHzeQDshWP5Ah7dega8hJACAJpQtw6OXk", // Production
        "3LP2mhUrV89xxzD1YKNndXHEhWWCRLPNKioZ9ymT" // Development
        ].contains(&login_info.client_secret.as_str())
    );

    let user = repositories::players::fetch_by_name(&ctx, &login_info.username).await;

    if user.is_none() {
        return Json(json!({
            "hint": "The username or password is incorrect."
        }));
    }

    let user = user.unwrap();
    let id: i32 = user.try_get("id").unwrap();
    let username: &str = user.try_get("username").unwrap();
    log::debug!("<{} ({})>", username, id);

    let token = Uuid::new_v4().to_string();
    log::debug!("{}:{}", login_info.username, login_info.password);

    Json(json!({
        "token_type": "Bearer",
        "expires_in": 86331,
        "access_token": token,
    }))
}