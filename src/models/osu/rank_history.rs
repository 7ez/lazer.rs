use serde::Serialize;

#[derive(Serialize)]
pub struct UserRankHistory {
    pub mode:       String,
    pub data:       Vec<u32>, // Rank for each month, starting from the oldest
}