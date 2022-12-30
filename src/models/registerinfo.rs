use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct RegisterInfo {
    pub username: String,
    pub password: String,
    pub email:    String,
}