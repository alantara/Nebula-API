use chrono::Utc;

pub fn generate_id() -> i64
{
    let timestamp = Utc::now().timestamp_millis() << 10;

    return timestamp;
}