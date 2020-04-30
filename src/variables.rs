use std::env;

pub struct Variables;

impl Variables {
    pub fn log_channel() -> u64 {
        env::var("ABB_LOG_CHANNEL")
            .expect("Log channel not found")
            .parse()
            .expect("Error parsing log channel")
    }

    pub fn token() -> String {
        env::var("ABB_TOKEN").expect("Token not found")
    }

    pub fn abb_user_id() -> u64 {
        env::var("ABB_USER_ID")
            .expect("AdmiralBumbleBee user ID not found")
            .parse()
            .expect("Error parsing ABB user ID")
    }

    pub fn admin_role() -> u64 {
        env::var("ABB_ADMIN_ROLE")
            .expect("Admin role ID not found")
            .parse()
            .expect("Error parsing Admin role ID")
    }

    pub fn announcement_channel() -> u64 {
        env::var("ABB_ANNOUNCE_CHANNEL")
            .expect("Announcement channel not found")
            .parse()
            .expect("Error parsing announcement channel")
    }
}
