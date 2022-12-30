use chrono::Utc;
use sqlx::{
    mysql::MySqlRow, 
    Row, MySqlPool
};

use crate::models::{
    osu::{
        user::User, 
        grades::UserGrades, 
        stats::UserStatistics, 
        level::UserLevel, 
        page::UserPage, 
        cover::UserCover, 
        country::UserCountry, 
        kudosu::UserKudosu,
        stats_rulesets::UserStatisticsRulesets,
        user_compact::UserCompact,
    }, 
    registerinfo::RegisterInfo
};

use crate::usecases;

pub async fn create(database: &MySqlPool, register_info: &RegisterInfo) {
    let pw_bcrypt = usecases::bcrypt::hash(&register_info.password);

    sqlx::query("INSERT INTO users (username, email, pw_bcrypt) VALUES (?, ?, ?)")
        .bind(&register_info.username)
        .bind(&register_info.email)
        .bind(&pw_bcrypt)
        .execute(database)
        .await
        .unwrap();
}

pub fn user_from_row(user: &MySqlRow) -> User {
    let user_stats = UserStatistics {
        level: UserLevel {
            current: 999,
            progress: 0
        },
        global_rank: 1,
        pp: 999,
        accuracy: 100.0,
        playcount: 999,
        ranked_score: 999,
        total_score: 999,
        hit_accuracy: 100.0,
        play_time: 999,
        total_hits: 999,
        maximum_combo: 999,
        replays_watched_by_others: 999,
        is_ranked: true,
        grade_counts: UserGrades {
            ss: 999,
            ssh: 999,
            s: 999,
            sh: 999,
            a: 999
        },
        country_rank: 1
    };
    // WHY MAN JUST WHY
    let osu_stats = user_stats.clone();
    let taiko_stats = user_stats.clone();
    let fruits_stats = user_stats.clone();
    let mania_stats = user_stats.clone();
    
    User {
        id: user.try_get("id").unwrap(),
        username: user.try_get("username").unwrap(),
        profile_colour: None,
        avatar_url: "https://a.ussr.pl/4812".to_string(),
        country_code: "GB".to_string(),
        is_active: true,
        is_bot: false,
        is_deleted: false,
        is_online: true,
        is_supporter: true,
        last_visit: Utc::now(),
        pm_friends_only: false,
        cover_url: None,
        default_group: "default".to_string(),
        has_supported: true,
        interests: Some("osu!".to_string()),
        join_date: user.try_get("join_date").unwrap(),
        kudosu: UserKudosu {
            total: 0,
            available: 0
        },
        location: Some("The USSR".to_string()),
        max_blocks: 0,
        max_friends: 0,
        playmode: None,
        playstyle: None,
        post_count: 0,
        profile_order: vec![],
        title: None,
        title_url: None,
        twitter: None,
        website: None,
        country: UserCountry {
            code: "GB".to_string(),
            name: "United Kingdom".to_string()
        },
        cover: UserCover {
            custom_url: None,
            url: None,
            id: None
        },
        is_restricted: false,
        account_history: vec![],
        active_tournament_banner: None,
        badges: vec![],
        beatmap_playcounts_count: 0,
        comments_count: 0,
        favourite_beatmapset_count: 0,
        follower_count: 0,
        graveyard_beatmapset_count: 0,
        groups: vec![],
        loved_beatmapset_count: 0,
        mapping_follower_count: 0,
        monthly_playcounts: vec![],
        page: UserPage {
            html: "".to_string(),
            raw: "".to_string()
        },
        pending_beatmapset_count: 0,
        previous_usernames: vec![],
        ranked_beatmapset_count: 0,
        replays_watched_counts: vec![],
        scores_best_count: 0,
        scores_first_count: 0,
        scores_recent_count: 0,
        statistics: user_stats,
        statistics_rulesets: UserStatisticsRulesets {
            osu: osu_stats,
            taiko: taiko_stats,
            fruits: fruits_stats,
            mania: mania_stats
        },
        support_level: 3,
        user_achievements: vec![],
        rank_history: None,
        ranked_and_approved_beatmapset_count: 0,
        unranked_beatmapset_count: 0,
    }
}

pub fn user_short_from_row(user: &MySqlRow) -> UserCompact {
    UserCompact {
        id: user.try_get("id").unwrap(),
        username: user.try_get("username").unwrap(),
        profile_colour: None,
        avatar_url: "https://a.ussr.pl/4812".to_string(),
        country_code: "GB".to_string(),
        is_active: true,
        is_bot: false,
        is_deleted: false,
        is_online: true,
        is_supporter: true,
        last_visit: Utc::now(),
        pm_friends_only: false,
    }
}