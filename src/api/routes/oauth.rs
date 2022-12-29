use axum::{ 
    routing::post,
    extract::Extension,
    Router, Json, Form,
};

use serde::{ Deserialize };
use serde_json::{Value, json};
use sqlx::Row;
use uuid::Uuid;

use crate::Context;

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


async fn token(
    ctx: Extension<Context>, 
    Form(login_info): Form<LoginInfo>
) -> Json<Value> {
    // osu!lazer has a static client id & secret
    assert_eq!(login_info.client_id, 5);
    assert!([
        "FGc9GAtyHzeQDshWP5Ah7dega8hJACAJpQtw6OXk", // Production
        "3LP2mhUrV89xxzD1YKNndXHEhWWCRLPNKioZ9ymT" // Development
        ].contains(&login_info.client_secret.as_str())
    );

    let user = sqlx::query("SELECT * FROM users WHERE username = ?")
        .bind(&login_info.username)
        .fetch_optional(&ctx.pool)
        .await
        .unwrap();

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