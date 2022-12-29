use serde::Serialize;

use super::stats::UserStatistics;

#[derive(Serialize)]
pub struct UserStatisticsRulesets {
    pub osu:        UserStatistics,
    pub taiko:      UserStatistics,
    pub fruits:     UserStatistics,
    pub mania:      UserStatistics,
}