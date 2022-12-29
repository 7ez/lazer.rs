use serde::Serialize;
use chrono::{ DateTime, Utc };

use crate::models::date_format::date_format;

#[derive(Serialize)]
pub struct UserCompact {
    id:                 u32,
    username:           String,
    profile_colour:     String,
    avatar_url:         String,
    country_code:       String,
    is_active:          bool,
    is_bot:             bool,
    is_deleted:         bool,
    is_online:          bool,
    is_supporter:       bool,
    
    #[serde(with = "date_format")]
    last_visit:         DateTime<Utc>,
    
    pm_friends_only:    bool,
}