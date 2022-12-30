use axum::{ 
    routing::post,
    extract::{ Multipart, Extension },
    http::{ HeaderMap, StatusCode },
    Router, Json, 
};

use serde::{ Deserialize };
use serde_json::{Value, json};
use sqlx::Row;

use crate::Context;
use crate::repositories;
use crate::usecases;

pub fn router() -> Router {
    Router::new()
        .route("/oauth/token", post(token))
}

#[derive(Deserialize, Debug)]
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
) -> (StatusCode, Json<Value>) {
    assert_eq!(headers.get("User-Agent").unwrap(), "osu!");

    let login_info = parse_login(multipart).await;

    // osu!lazer has a static client id & secret
    assert_eq!(login_info.client_id, 5);
    assert!([
        "FGc9GAtyHzeQDshWP5Ah7dega8hJACAJpQtw6OXk", // Production
        "3LP2mhUrV89xxzD1YKNndXHEhWWCRLPNKioZ9ymT" // Development
        ].contains(&login_info.client_secret.as_str())
    );

    let user = repositories::players::fetch_by_name(&ctx.database, &login_info.username).await;

    if user.is_none() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "hint": "The username or password is incorrect."
        })));
    }

    let user = user.unwrap();
    let id: i32 = user.try_get("id").unwrap();
    let username: String = user.try_get("username").unwrap();
    let pw_md5: String = usecases::bcrypt::get_md5(&login_info.password);
    let pw_bcrypt: String = user.try_get("pw_bcrypt").unwrap();

    if !ctx.cache.passwords.lock().await.contains_key(&pw_md5) {
        if !usecases::bcrypt::verify(&pw_md5, &pw_bcrypt) {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "hint": "The username or password is incorrect."
            })));
        }

        ctx.cache.passwords.lock().await.insert(pw_bcrypt, pw_md5);
    } else {
        if ctx.cache.passwords.lock().await.get(&pw_bcrypt).unwrap() != &pw_md5 {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "hint": "The username or password is incorrect."
            })));
        }
    }

    let session = repositories::sessions::create(&ctx, id.clone());
    log::info!("<{} ({})> logged in!", username, id);
    
    (StatusCode::OK, Json(json!({
        "token_type": session.token_type,
        "expires_in": 86400,
        "access_token": session.access_token,
    })))
}