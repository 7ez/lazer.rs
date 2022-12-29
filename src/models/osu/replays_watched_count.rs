use serde::Serialize;

#[derive(Serialize)]
pub struct UserReplaysWatchedCount {
    pub start_date: String,
    pub count:      u32,
}