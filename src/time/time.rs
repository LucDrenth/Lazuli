use std::time::{SystemTime, UNIX_EPOCH};

pub const TICK_RATE: u32 = 60; // per second
pub const DELTA: f32 = 1.0 / TICK_RATE as f32;

pub fn now_millis() -> u128 {
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn now_seconds() -> u64 {
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
