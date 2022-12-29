use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use serde_with::{serde_as, TimestampSecondsWithFrac};

use super::candle_type::CandleType;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlePrice {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    pub datetime: DateTime<Utc>,
}

impl CandlePrice {
    pub fn new(datetime: DateTime<Utc>, price: f64) -> Self {
        Self {
            open: price,
            close: price,
            high: price,
            low: price,
            datetime,
        }
    }

    pub fn update(&mut self, datetime: DateTime<Utc>, price: f64) {
        self.close = price;
        self.datetime = datetime;

        if self.open == 0.0 {
            self.open = price;
        }

        if self.high < price || self.high == 0.0 {
            self.high = price;
        }

        if self.low > price || self.low == 0.0 {
            self.low = price;
        }
    }

    pub fn get_candle_date(&self, candle_type: CandleType) -> DateTime<Utc> {
        candle_type.get_start_date(self.datetime)
    }
}
