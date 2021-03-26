/// Gets the time right now
fn current_time() -> NaiveDateTime {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let rightnow = NaiveDateTime::from_timestamp(since_the_epoch.as_secs() as i64, 0);
    rightnow
}
