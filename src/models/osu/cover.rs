use serde::Serialize;

#[derive(Serialize)]
pub struct UserCover {
    pub custom_url:     Option<String>,
    pub url:            Option<String>,
    pub id:             Option<u32>,
}