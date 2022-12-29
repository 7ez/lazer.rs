use std::sync::Arc;

use axum::{ 
    routing::post,
    Router, Json, Extension, http::HeaderMap
};

use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::Row;

use crate::{Context, models::cache::Cache, repositories};


pub fn router() -> Router {
    Router::new()
        .route("/api/v2/me", post(me))
}

fn get_auth_token(headers: HeaderMap) -> Option<String> {
    if let Some(auth) = headers.get("Authorization") {
        let auth = auth.to_str().unwrap();

        if auth.starts_with("Bearer ") {
            return Some(auth[7..].to_string());
        }
    }

    None
}

async fn me(
    headers: HeaderMap,
    ctx: Extension<Context>,
) -> Json<Value> {
    if let Some(token) = get_auth_token(headers) {
        log::debug!("Token: {}", token);

        let user_id = ctx.cache.users.get(&token).unwrap().clone();
        let user = repositories::players::fetch_by_id(&ctx, &user_id).await;

        if user.is_none() {
            return Json(json!({
                "status": 400,
                "message": "invalid token"
            }))
        }

        let user = user.unwrap();

        // TODO: return user
        return Json(json!({}));
    }

    Json(json!({
        "status": 400,
        "message": "missing token"
    }))
}