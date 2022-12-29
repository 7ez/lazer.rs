use serde::Serialize;

#[derive(Serialize)]
pub struct UserPlaycount {
    pub start_date: String,
    pub count:      u32,
}