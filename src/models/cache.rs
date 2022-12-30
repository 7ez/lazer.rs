use std::sync::Arc;
use std::collections::HashMap;

use tokio::sync::Mutex;

pub struct Cache {
    pub passwords: Arc<Mutex<HashMap<String, String>>>, // pw_bcrypt -> pw_md5
}