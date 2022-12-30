use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct UserLevel {
    pub current:    u32,
    pub progress:   u32,
}