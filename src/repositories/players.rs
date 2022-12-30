
use sqlx::{
    mysql::{ MySqlPool, MySqlRow }, 
    Row
};

pub async fn fetch_by_name(database: &MySqlPool, username: &String) -> Option<MySqlRow> {
    let player = sqlx::query("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(database)
        .await
        .unwrap();

    if player.is_none() {
        return None;
    }
    
    player
}

pub async fn fetch_by_email(database: &MySqlPool, email: &String) -> Option<MySqlRow> {
    let player = sqlx::query("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(database)
        .await
        .unwrap();

    if player.is_none() {
        return None;
    }
    
    player
}

pub async fn fetch_by_id(database: &MySqlPool, user_id: &i32) -> Option<MySqlRow> {
    let player = sqlx::query("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(database)
        .await
        .unwrap();

    if player.is_none() {
        return None;
    }
    
    player
}

pub async fn fetch_friends(database: &MySqlPool, user_id: &i32) -> Vec<i32> {
    let mut friends: Vec<i32> = Vec::new();

    let rows = sqlx::query("SELECT user2 FROM friends WHERE user1 = ?")
        .bind(user_id)
        .fetch_all(database)
        .await
        .unwrap();

    for row in rows {
        let friend_id: i32 = row.try_get("user2").unwrap();
        friends.push(friend_id);
    }

    friends
}