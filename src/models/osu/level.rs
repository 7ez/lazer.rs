use serde::Serialize;

#[derive(Serialize)]
pub struct UserLevel {
    pub current:    u32,
    pub progress:   u32,
}