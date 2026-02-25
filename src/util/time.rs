use chrono::{DateTime, Duration, Utc};
use rand::Rng;

/// Get Utc::now() with the specified random deviation
pub fn now_with_random_offset(max_offset_seconds: i64) -> DateTime<Utc> {
    let offset = rand::rng().random_range(-max_offset_seconds..max_offset_seconds);
    Utc::now() + Duration::seconds(offset)
}
