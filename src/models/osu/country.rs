use serde::Serialize;

#[derive(Serialize)]
pub struct UserCountry {
    code: String,
    name: String,
}