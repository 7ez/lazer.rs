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
    id:                                     u32,
    username:                               String,
    profile_colour:                         Option<String>,
    avatar_url:                             String,
    country_code:                           String,
    is_active:                              bool,
    is_bot:                                 bool,
    is_deleted:                             bool,
    is_online:                              bool,
    is_supporter:                           bool,
    
    #[serde(with = "date_format")]
    last_visit:                             DateTime<Utc>,
    
    pm_friends_only:                        bool,

    cover_url:                              Option<String>,
    default_group:                          String,
    has_supported:                          bool,
    interests:                              Option<String>,
    
    #[serde(with = "date_format")]
    join_date:                              DateTime<Utc>,
    
    kudosu:                                 UserKudosu,
    location:                               Option<String>,
    max_blocks:                             u32,
    max_friends:                            u32,
    playmode:                               Option<String>,
    playstyle:                              Option<String>, // i think?
    post_count:                             u32,
    profile_order:                          Vec<String>,
    title:                                  Option<String>,
    title_url:                              Option<String>,
    twitter:                                Option<String>,
    website:                                Option<String>,
    country:                                UserCountry,
    cover:                                  UserCover,
    is_restricted:                          bool,
    account_history:                        Vec<UserAccountHistory>,
    active_tournament_banner:               Option<String>,
    badges:                                 Vec<UserBadge>,
    beatmap_playcounts_count:               u32,
    comments_count:                         u32,
    favourite_beatmapset_count:             u32,
    follower_count:                         u32,
    graveyard_beatmapset_count:             u32,
    groups:                                 Vec<UserGroup>,
    loved_beatmapset_count:                 u32,
    mapping_follower_count:                 u32,
    monthly_playcounts:                     Vec<UserPlaycount>,
    page:                                   UserPage,
    pending_beatmapset_count:               u32,
    previous_usernames:                     Vec<String>,
    ranked_beatmapset_count:                u32,
    replays_watched_counts:                 Vec<UserReplaysWatchedCount>,
    scores_best_count:                      u32,
    scores_first_count:                     u32,
    scores_recent_count:                    u32,
    statistics:                             UserStatistics,
    statistics_rulesets:                    Vec<UserStatisticsRulesets>,
    support_level:                          u32,
    user_achievements:                      Vec<UserAchievement>,
    rank_history:                           Option<UserRankHistory>,
    ranked_and_approved_beatmapset_count:   u32,
    unranked_beatmapset_count:              u32,
}