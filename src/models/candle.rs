use chrono::{DateTime, Utc};
use super::{candle_type::CandleType, candle_data::CandleData};

#[derive(Clone)]
pub struct BidAskCandle {
    pub candle_type: CandleType,
    pub datetime: DateTime<Utc>,
    pub instrument: String,
    pub bid_data: CandleData,
    pub ask_data: CandleData,
}

impl BidAskCandle {
    pub fn update(&mut self, datetime: DateTime<Utc>, bid: f64, ask: f64, bid_vol: f64, ask_vol: f64) {
        self.bid_data.update(datetime, bid, bid_vol);
        self.ask_data.update(datetime, ask, ask_vol);
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