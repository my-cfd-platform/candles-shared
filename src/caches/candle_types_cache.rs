use chrono::{DateTime, Utc};
use crate::models::{candle_type::CandleType, candle_data::CandleData};
use super::candle_prices_cache::CandlePricesCache;

#[derive(Debug, Clone)]
pub struct CandleTypesCache {
    pub instrument_id: String,
    pub minutes: CandlePricesCache,
    pub hours: CandlePricesCache,
    pub days: CandlePricesCache,
    pub months: CandlePricesCache,
}

impl CandleTypesCache {
    pub fn new(instrument_id: String) -> Self {
        Self {
            instrument_id: instrument_id,
            minutes: CandlePricesCache::new(CandleType::Minute),
            hours: CandlePricesCache::new(CandleType::Hour),
            days: CandlePricesCache::new(CandleType::Day),
            months: CandlePricesCache::new(CandleType::Month),
        }
    }

    pub fn init(&mut self, candle: CandleData, candle_type: CandleType) {
        match candle_type {
            CandleType::Minute => self.minutes.init(candle),
            CandleType::Hour => self.hours.init(candle),
            CandleType::Day => self.days.init(candle),
            CandleType::Month => self.months.init(candle),
        };
    }

    pub fn get_by_date_range(
        &self,
        candle_type: CandleType,
        date_from: DateTime<Utc>,
        date_to: DateTime<Utc>,
    ) -> Vec<CandleData> {
        match candle_type {
            CandleType::Minute => self.minutes.get_by_date_range(date_from, date_to),
            CandleType::Hour => self.hours.get_by_date_range(date_from, date_to),
            CandleType::Day => self.days.get_by_date_range(date_from, date_to),
            CandleType::Month => self.months.get_by_date_range(date_from, date_to),
        }
    }

    pub fn update(&mut self, rate: f64, volume: f64, datetime: DateTime<Utc>) {
        self.minutes.update(datetime, rate, volume);
        self.hours.update(datetime, rate, volume);
        self.days.update(datetime, rate, volume);
        self.months.update(datetime, rate, volume);
    }

    pub fn clear(&mut self) {
        self.days.clear();
        self.hours.clear();
        self.minutes.clear();
        self.months.clear();
    }
}
