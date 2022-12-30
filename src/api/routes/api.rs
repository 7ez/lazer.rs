use axum::{ 
    extract::{ Path, Query },
    routing::get,
    http::HeaderMap,
    Router, Json, Extension,
};

use serde::Deserialize;
use serde_json::{ 
    json, 
    Value
};
use sqlx::mysql::MySqlRow;

use crate::{
    Context, 
    repositories,
    usecases,
    models::{osu::user_compact::UserCompact, session::Session},
};


pub fn router() -> Router {
    Router::new()
        .route("/api/v2/me/", get(me))
        .route("/api/v2/friends", get(friends))
        .route("/api/v2/users/:value/", get(user))
}

fn get_auth_token(headers: &HeaderMap) -> Option<String> {
    if let Some(auth) = headers.get("Authorization") {
        let auth = auth.to_str().unwrap();

        if auth.starts_with("Bearer ") {
            return Some(auth[7..].to_string());
        }
    }

    None
}

async fn get_session_from_token(ctx: &Context, headers: &HeaderMap) -> Option<Session> {
    if let Some(token) = get_auth_token(headers) {
        let session = repositories::sessions::get(&ctx, token);
        return session;
    }

    None
}

async fn friends(
    headers: HeaderMap,
    ctx: Extension<Context>
) -> Json<Value> {
    if let Some(session) = get_session_from_token(&ctx, &headers).await {
        let mut friends: Vec<UserCompact> = Vec::new();
        let user_friends = repositories::players::fetch_friends(&ctx.database, &session.user_id).await;

        for friend in user_friends {
            let friend = repositories::players::fetch_by_id(&ctx.database, &friend).await;
            
            if let Some(friend) = friend {
                friends.push(usecases::players::user_short_from_row(&friend));
            }
        }

        return Json(json!(friends));
    }

    Json(json!({
        "status": 400,
        "message": "invalid/missing token"
    }))
}

#[derive(Deserialize)]
struct UserQuery {
    key: String,
}

async fn user(
    headers: HeaderMap,
    ctx: Extension<Context>,
    value: Path<String>,
    query: Query<UserQuery>,
) -> Json<Value> {
    if let Some(_session) = get_session_from_token(&ctx, &headers).await {
        let user: Option<MySqlRow>;

        match query.key.as_str() {
            "id" => {
                let id = value.parse::<i32>().unwrap();
                user = repositories::players::fetch_by_id(&ctx.database, &id).await;
            },
            "username" => {
                user = repositories::players::fetch_by_name(&ctx.database, &value).await;
            },
            _ => { return Json(json!({ "status": 400, "message": "invalid key" })); }
        }

        if let Some(user) = user {
            let lazer_user = usecases::players::user_from_row(&user);

            return Json(json!(lazer_user));
        }

        return Json(json!({
            "status": 400,
            "message": "user not found"
        }));
    }

    Json(json!({
        "status": 400,
        "message": "missing/invalid token"
    }))
}

async fn me(
    headers: HeaderMap,
    ctx: Extension<Context>,
) -> Json<Value> {
    if let Some(session) = get_session_from_token(&ctx, &headers).await {
        let user = repositories::players::fetch_by_id(&ctx.database, &session.user_id).await.unwrap();
        let lazer_user = usecases::players::user_from_row(&user);

        return Json(json!(lazer_user));
    }

    Json(json!({
        "status": 400,
        "message": "missing/invalid token"
    }))
}