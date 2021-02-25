use chrono::{DateTime, TimeZone, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Time;

impl Time {
    pub fn current_time() -> Duration {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch
    }

    pub fn time_from_ms(ms: SystemTime) -> String {
        let dt = Utc.timestamp(ms.tv_sec, ms.tv_nsec).to_string();
        dt
    }
}
