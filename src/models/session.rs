use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::date_format::date_format;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub user_id:        i32,
    pub token_type:     String,
    pub access_token:   String,
    pub refresh_token:  String,

    #[serde(with = "date_format")]
    pub expires_in:     DateTime<Utc>,
}