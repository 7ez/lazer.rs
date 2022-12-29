use sqlx::mysql::MySqlRow;
use crate::Context;

pub async fn fetch_by_name(ctx: &Context, username: &String) -> Option<MySqlRow> {
    let player = sqlx::query("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(&ctx.database)
        .await
        .unwrap();

    if player.is_none() {
        return None;
    }
    
    player
}
pub async fn fetch_by_id(ctx: &Context, user_id: &u32) -> Option<MySqlRow> {
    let player = sqlx::query("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(&ctx.database)
        .await
        .unwrap();

    if player.is_none() {
        return None;
    }
    
    player
}