use serde::Serialize;
use chrono::{ DateTime, Utc };

use crate::models::date_format::date_format;

use super::{
    kudosu::UserKudosu, 
    country::UserCountry, 
    cover::UserCover, 
    account_history::UserAccountHistory, 
    badge::UserBadge, 
    group::UserGroup, 
    playcount::UserPlaycount, 
    page::UserPage, 
    replays_watched_count::UserReplaysWatchedCount, 
    stats::UserStatistics, 
    stats_rulesets::UserStatisticsRulesets, achievement::UserAchievement, rank_history::UserRankHistory
};


#[derive(Serialize)]
pub struct User {
    pub id:                                     i32,
    pub username:                               String,
    pub profile_colour:                         Option<String>,
    pub avatar_url:                             String,
    pub country_code:                           String,
    pub is_active:                              bool,
    pub is_bot:                                 bool,
    pub is_deleted:                             bool,
    pub is_online:                              bool,
    pub is_supporter:                           bool,
    
    #[serde(with = "date_format")]
    pub last_visit:                             DateTime<Utc>,

    pub pm_friends_only:                        bool,

    pub cover_url:                              Option<String>,
    pub default_group:                          String,
    pub has_supported:                          bool,
    pub interests:                              Option<String>,

    #[serde(with = "date_format")]
    pub join_date:                              DateTime<Utc>,
    
    pub kudosu:                                 UserKudosu,
    pub location:                               Option<String>,
    pub max_blocks:                             u32,
    pub max_friends:                            u32,
    pub playmode:                               Option<String>,
    pub playstyle:                              Option<String>, // i think?
    pub post_count:                             u32,
    pub profile_order:                          Vec<String>,
    pub title:                                  Option<String>,
    pub title_url:                              Option<String>,
    pub twitter:                                Option<String>,
    pub website:                                Option<String>,
    pub country:                                UserCountry,
    pub cover:                                  UserCover,
    pub is_restricted:                          bool,
    pub account_history:                        Vec<UserAccountHistory>,
    pub active_tournament_banner:               Option<String>,
    pub badges:                                 Vec<UserBadge>,
    pub beatmap_playcounts_count:               u32,
    pub comments_count:                         u32,
    pub favourite_beatmapset_count:             u32,
    pub follower_count:                         u32,
    pub graveyard_beatmapset_count:             u32,
    pub groups:                                 Vec<UserGroup>,
    pub loved_beatmapset_count:                 u32,
    pub mapping_follower_count:                 u32,
    pub monthly_playcounts:                     Vec<UserPlaycount>,
    pub page:                                   UserPage,
    pub pending_beatmapset_count:               u32,
    pub previous_usernames:                     Vec<String>,
    pub ranked_beatmapset_count:                u32,
    pub replays_watched_counts:                 Vec<UserReplaysWatchedCount>,
    pub scores_best_count:                      u32,
    pub scores_first_count:                     u32,
    pub scores_recent_count:                    u32,
    pub statistics:                             UserStatistics,
    pub statistics_rulesets:                    UserStatisticsRulesets,
    pub support_level:                          u32,
    pub user_achievements:                      Vec<UserAchievement>,
    pub rank_history:                           Option<UserRankHistory>,
    pub ranked_and_approved_beatmapset_count:   u32,
    pub unranked_beatmapset_count:              u32,
}