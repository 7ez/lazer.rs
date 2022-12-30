use std::collections::HashMap;

use axum::{ 
    routing::post,
    response::{IntoResponse, Response},
    extract::{Multipart, Extension},
    http::{HeaderMap, StatusCode},
    Router,
};

use serde_json::json;
use regex::Regex;

use crate::Context;
use crate::repositories;
use crate::usecases;
use crate::models::registerinfo::RegisterInfo;

pub fn router() -> Router {
    Router::new()
        .route("/users", post(register))
}

async fn parse_register(mut multipart: Multipart) -> RegisterInfo {
    let mut register_info = RegisterInfo {
        username:  String::new(),
        password:  String::new(),
        email:     String::new(),
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_owned();
        let value = field.text().await.unwrap().to_owned();

        match name.as_str() {
            // what are these namings peppy fuck you
            "user[username]" => register_info.username = value,
            "user[user_email]" => register_info.email = value,
            "user[password]" => register_info.password = value,
            _ => {}
        }
    }

    register_info
}

async fn register(
    headers: HeaderMap,
    ctx: Extension<Context>, 
    multipart: Multipart,
) -> Response {
    assert_eq!(headers.get("User-Agent").unwrap(), "osu!");

    let register_info = parse_register(multipart).await;
    let mut errors: HashMap<String, Vec<String>> = HashMap::new();
    
    errors.insert("username".to_string(), vec![]);
    errors.insert("user_email".to_string(), vec![]);
    errors.insert("password".to_string(), vec![]);

    if register_info.username.len() == 0 {
        errors.get_mut("username").unwrap().push("required".to_string());
    } else {
        let username_regex = Regex::new(r"^[\w \[\]-]{3,32}$").unwrap();
        
        if !username_regex.is_match(&register_info.username) {
            errors.get_mut("username").unwrap().push("Usernames must be between 3 and 32 characters".to_string());
        }

        if register_info.username.contains("_") && register_info.username.contains(" ") {
            errors.get_mut("username").unwrap().push("Usernames cannot contain an underscore and space".to_string());
        }

        if let Some(_) = repositories::players::fetch_by_name(&ctx.database, &register_info.username).await {
            errors.get_mut("username").unwrap().push("Username must not be in use".to_string());
        }
    }

    if register_info.email.len() == 0 {
        errors.get_mut("user_email").unwrap().push("required".to_string());
    } else {
        let email_regex = Regex::new(r"^[^@\s]{1,200}@[^@\s\.]{1,30}(?:\.[^@\.\s]{2,24})+$").unwrap();
        
        if !email_regex.is_match(&register_info.email) {
            errors.get_mut("user_email").unwrap().push("Email must be valid".to_string());
        }

        if let Some(_) = repositories::players::fetch_by_email(&ctx.database, &register_info.email).await {
            errors.get_mut("user_email").unwrap().push("Email must not be in use".to_string());
        }
    }

    if register_info.password.len() == 0 {
        errors.get_mut("password").unwrap().push("required".to_string());
    } else {
        if &register_info.password.len() >= &32 || &register_info.password.len() <= &8 {
            errors.get_mut("password").unwrap().push("Passwords must be between 8 and 32 characters".to_string());
        }
    }

    if errors.get("username").unwrap().len() > 0 || errors.get("user_email").unwrap().len() > 0 || errors.get("password").unwrap().len() > 0 {
        return (StatusCode::BAD_REQUEST, json!({
            "form_error": {"user": errors}
        }).to_string()) .into_response();
    }

    usecases::players::create(&ctx.database, &register_info).await;
    (StatusCode::OK, "ok".to_string()).into_response()
}