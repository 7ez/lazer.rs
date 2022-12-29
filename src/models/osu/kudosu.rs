use serde::Serialize;

#[derive(Serialize)]
pub struct UserKudosu {
    total:      u32,
    available:  u32
}