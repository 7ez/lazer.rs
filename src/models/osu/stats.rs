use serde::Serialize;

use super::{
    level::UserLevel, 
    grades::UserGrades
};

#[derive(Serialize)]
pub struct UserStatistics {
    level:                      UserLevel,
    global_rank:                i32,
    pp:                         f32,
    accuracy:                   f32,
    playcount:                  u32,
    ranked_score:               u64,
    total_score:                u64,
    hit_accuracy:               f32,
    play_time:                  u64,
    total_hits:                 u64,
    maximum_combo:              u32,
    replays_watched_by_others:  u64,
    is_ranked:                  bool,
    grade_counts:               UserGrades,
    country_rank:               i32,
}