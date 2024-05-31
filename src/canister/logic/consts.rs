use std::time::Duration;

pub static DAY_IN_NANOS: u64 = Duration::from_secs(24 * 60 * 60).as_nanos() as u64;
pub static MIN_WHITELISTED: usize = 3;
pub static MAX_WHITELISTED: usize = 3;
