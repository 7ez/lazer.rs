use std::collections::HashMap;

pub struct Cache {
    pub users:     HashMap<String, u32>, // username -> user_id   
}