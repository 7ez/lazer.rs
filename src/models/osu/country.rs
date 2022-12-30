use serde::Serialize;

#[derive(Serialize)]
pub struct UserCountry {
    pub code: String,
    pub name: String,
}