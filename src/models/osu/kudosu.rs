use serde::Serialize;

#[derive(Serialize)]
pub struct UserKudosu {
    pub total:      u32,
    pub available:  u32
}