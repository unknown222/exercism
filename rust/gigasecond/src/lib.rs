use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigasecond = (10 as i64).pow(9);
    return start + Duration::seconds(gigasecond);
}
