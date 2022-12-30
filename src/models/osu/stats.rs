use serde::Serialize;

use super::{
    level::UserLevel, 
    grades::UserGrades
};

#[derive(Serialize, Clone)]
pub struct UserStatistics {
    pub level:                      UserLevel,
    pub global_rank:                i32,
    pub pp:                         i32,
    pub accuracy:                   f32,
    pub playcount:                  u32,
    pub ranked_score:               u64,
    pub total_score:                u64,
    pub hit_accuracy:               f32,
    pub play_time:                  u64,
    pub total_hits:                 u64,
    pub maximum_combo:              u32,
    pub replays_watched_by_others:  u64,
    pub is_ranked:                  bool,
    pub grade_counts:               UserGrades,
    pub country_rank:               i32,
}
