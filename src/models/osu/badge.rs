use serde::Serialize;


#[derive(Serialize)]
pub struct UserBadge {
    pub todo: String,
}