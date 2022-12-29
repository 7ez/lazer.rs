use serde::Serialize;

#[derive(Serialize)]
pub struct UserCover {
    custom_url:     Option<String>,
    url:            Option<String>,
    id:             Option<u32>,
}