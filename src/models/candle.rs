use chrono::{DateTime, Utc};
use super::{candle_type::CandleType, candle_price::CandlePrice};

#[derive(Clone)]
pub struct BidAskCandle {
    pub candle_type: CandleType,
    pub datetime: DateTime<Utc>,
    pub instrument: String,
    pub bid_price: CandlePrice,
    pub ask_price: CandlePrice,
}

impl BidAskCandle {
    pub fn update(&mut self, datetime: DateTime<Utc>, bid: f64, ask: f64) {
        self.bid_price.update(datetime, bid);
        self.ask_price.update(datetime, ask);
    }

    pub fn generate_id(
        instrument: &str,
        candle_type: &CandleType,
        datetime: DateTime<Utc>,
    ) -> String {
        format!(
            "{}{}{}",
            candle_type.to_owned() as u8,
            instrument.to_string(),
            candle_type.get_start_date(datetime).timestamp(),
        )
    }

    pub fn get_id(&self) -> String {
        BidAskCandle::generate_id(&self.instrument, &self.candle_type, self.datetime)
    }
}