use chrono::{DateTime, NaiveDateTime};

pub fn test_uuid() -> uuid::Uuid {
    uuid::Uuid::parse_str("7fb305c1-9cb2-4cd9-a57a-7508ec07ecce").unwrap()
}

pub fn test_time() -> NaiveDateTime {
    DateTime::parse_from_rfc3339("2022-10-10T04:24:42.995338+00:00")
        .unwrap()
        .naive_utc()
}

pub fn test_actor() -> uuid::Uuid {
    uuid::Uuid::parse_str("78554741-b71a-4ce6-8061-2ac77c7fff0c").unwrap()
}
