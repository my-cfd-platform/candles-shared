use chrono::{DateTime, Datelike, Utc};
use chrono::{TimeZone};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, IntoPrimitive, TryFromPrimitive, Hash, Eq, PartialEq)]
#[repr(i32)]
pub enum CandleType {
    Minute = 0,
    Hour = 1,
    Day = 2,
    Month = 3,
}

impl CandleType {
    pub fn candle_date(&self, datetime: DateTime<Utc>) -> DateTime<Utc> {
        let timestamp_sec = datetime.timestamp();

        match self {
            CandleType::Minute => Utc.timestamp_millis_opt((timestamp_sec - timestamp_sec % 60) * 1000).unwrap(),
            CandleType::Hour => Utc.timestamp_millis_opt((timestamp_sec - timestamp_sec % 3600) * 1000).unwrap(),
            CandleType::Day => Utc.timestamp_millis_opt((timestamp_sec - timestamp_sec % 86400) * 1000).unwrap(),
            CandleType::Month => {
                let date = Utc.timestamp_millis_opt(timestamp_sec * 1000).unwrap();
                let start_of_month: DateTime<Utc> = Utc
                    .with_ymd_and_hms(date.year(), date.month(), 1, 0, 0, 0)
                    .unwrap();

                return start_of_month;
            }
        }
    }
}