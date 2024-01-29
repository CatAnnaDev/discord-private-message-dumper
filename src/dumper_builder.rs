pub struct Dump {
    pub user_id: String,
    pub channel_id: String,
}

impl Dump {
    pub fn new(user_id: String, channel_id: String) -> Self {
        Self {
            user_id,
            channel_id,
        }
    }
}