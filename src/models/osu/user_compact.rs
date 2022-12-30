use serde::Serialize;
use chrono::{ DateTime, Utc };

use crate::models::date_format::date_format;

#[derive(Serialize)]
pub struct UserCompact {
    pub id:                 u32,
    pub username:           String,
    pub profile_colour:     Option<String>,
    pub avatar_url:         String,
    pub country_code:       String,
    pub is_active:          bool,
    pub is_bot:             bool,
    pub is_deleted:         bool,
    pub is_online:          bool,
    pub is_supporter:       bool,
    
    #[serde(with = "date_format")]
    pub last_visit:         DateTime<Utc>,
    
    pub pm_friends_only:    bool,
}