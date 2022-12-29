use serde::Serialize;

#[derive(Serialize)]
pub struct UserGroup {
    pub todo: String,
}