use chrono::{ Utc, Duration };
use redis::Commands;
use serde_json::json;
use uuid::Uuid;

use crate::Context;
use crate::models::session::Session;

pub fn create(ctx: &Context, user_id: i32) -> Session {
    let access_token = Uuid::new_v4().to_string();
    let refresh_token = Uuid::new_v4().to_string();
    let session = Session {
        user_id,
        token_type: "Bearer".to_string(),
        access_token,
        refresh_token,
        expires_in: Utc::now() + Duration::days(1),
    };

    let _: String = ctx.redis
        .get_connection()
        .unwrap()
        .set_ex( // TODO: figure out how to not assign this lmao
        format!("lazer:sessions:{}", &session.access_token),
        json!(&session).to_string(),
        86400, // 1 day
        ).unwrap(); 

    session
}


// as always, thank you github copilot.
pub fn get(ctx: &Context, token: String) -> Option<Session> {
    let session: Option<String> = ctx.redis.get_connection().unwrap().get(format!("lazer:sessions:{}", token)).unwrap();
    
    if session.is_none() {
        return None;
    }
    
    
    let session = session.unwrap();
    let session: Session = serde_json::from_str(&session).unwrap();

    Some(session)
}