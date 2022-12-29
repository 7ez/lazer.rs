use serde::Serialize;

#[derive(Serialize)]
pub struct UserPage {
    pub html:   String,
    pub raw:    String,
}