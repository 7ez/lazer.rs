use chrono::{ DateTime, Utc };
use serde::Serialize;

use crate::models::date_format::date_format;

#[derive(Serialize)]
pub struct UserAccountHistory {
    pub id:             u32,
    pub _type:          String, // note, restriction, silence
    
    #[serde(with = "date_format")]
    pub timestamp:      DateTime<Utc>,
    
    pub length:         u32,
    pub description:    Option<String>,
}