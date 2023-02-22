use crate::models::{candle_type::CandleType, candle_data::CandleData};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tokio::sync::RwLock;
use super::candle_types_cache::CandleTypesCache;

pub struct CandleBidAsksCache {
    pub bid_candles: RwLock<HashMap<String, CandleTypesCache>>,
    pub ask_candles: RwLock<HashMap<String, CandleTypesCache>>,
}

impl CandleBidAsksCache {
    pub fn new() -> Self {
        Self {
            bid_candles: RwLock::new(HashMap::new()),
            ask_candles: RwLock::new(HashMap::new()),
        }
    }

    pub async fn update(&self, datetime: DateTime<Utc>, instrument: &str, bid: f64, ask: f64, bid_vol: f64, ask_vol: f64) {
        self.update_bid_or_ask(true, datetime, instrument, bid, ask, bid_vol, ask_vol)
            .await;
        self.update_bid_or_ask(false, datetime, instrument, bid, ask, bid_vol, ask_vol)
            .await;
    }

    async fn update_bid_or_ask(
        &self,
        is_bid: bool,
        datetime: DateTime<Utc>,
        instrument: &str,
        bid: f64,
        ask: f64,
        bid_vol: f64,
        ask_vol: f64,
    ) {
        let mut write_lock = match is_bid {
            true => self.bid_candles.write().await,
            false => self.ask_candles.write().await,
        };

        let target_instruments_cache = write_lock.get_mut(instrument);
        let rarget_rate = match is_bid {
            true => bid,
            false => ask,
        };
        let target_vol = match is_bid {
            true => bid_vol,
            false => ask_vol,
        };

        match target_instruments_cache {
            Some(cache) => {
                cache.update(rarget_rate, target_vol, datetime);
            }
            None => {
                let mut cache = CandleTypesCache::new(instrument.to_owned());
                cache.update(rarget_rate, target_vol, datetime);
                write_lock.insert(instrument.to_owned(), cache);
            }
        }
    }

    pub async fn init(
        &self,
        instument_id: String,
        is_bid: bool,
        candle_type: CandleType,
        candle: CandleData,
    ) {
        let mut target_cache = match is_bid {
            true => self.bid_candles.write().await,
            false => self.ask_candles.write().await,
        };

        let instrument_cache = target_cache.get_mut(&instument_id);

        match instrument_cache {
            Some(cache) => {
                cache.init(candle, candle_type);
            }
            None => {
                let mut cache = CandleTypesCache::new(instument_id.clone());
                cache.init(candle, candle_type);
                target_cache.insert(instument_id, cache);
            }
        }
    }

    pub async fn get_by_date_range(
        &self,
        instument_id: String,
        candle_type: CandleType,
        is_bid: bool,
        date_from: DateTime<Utc>,
        date_to: DateTime<Utc>,
    ) -> Vec<CandleData> {
        let target_cache = match is_bid {
            true => self.bid_candles.read().await,
            false => self.ask_candles.read().await,
        };

        let instrument_cache = target_cache.get(&instument_id);

        match instrument_cache {
            Some(cache) => cache.get_by_date_range(candle_type, date_from, date_to),
            None => {
                vec![]
            }
        }
    }

    pub async fn clear(&mut self) {
        let mut bids = self.bid_candles.write().await;
        bids.clear();
        let mut asks = self.ask_candles.write().await;
        asks.clear();
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use crate::models::candle_type::CandleType;

    use super::CandleBidAsksCache;

    #[tokio::test]
    async fn test_sinle_quote() {
        let cache = CandleBidAsksCache::new();
        let instument = String::from("EURUSD");
        let date = Utc.timestamp_millis_opt(1662559404 * 1000).unwrap();
        let bid = 25.55;
        let ask = 36.55;

        cache.update(date, &instument, bid, ask, 0.0, 0.0).await;

        let result_bid_minute = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Minute,
                true,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;
        let result_ask_minute = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Minute,
                false,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;

        let result_bid_hour = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Hour,
                true,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;
        let result_ask_hour = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Hour,
                false,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;

        let result_bid_day = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Day,
                true,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;
        let result_ask_day = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Day,
                false,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;

        let result_bid_mount = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Month,
                true,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;
        let result_ask_mount = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Month,
                false,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;

        assert_eq!(result_bid_minute.len(), 1);
        assert_eq!(result_ask_minute.len(), 1);

        assert_eq!(result_bid_hour.len(), 1);
        assert_eq!(result_ask_hour.len(), 1);

        assert_eq!(result_bid_day.len(), 1);
        assert_eq!(result_ask_day.len(), 1);

        assert_eq!(result_bid_mount.len(), 1);
        assert_eq!(result_ask_mount.len(), 1);
    }

    #[tokio::test]
    async fn test_date_rotation_minute() {
        let cache = CandleBidAsksCache::new();
        let instument = String::from("EURUSD");
        let date = Utc.timestamp_millis_opt(1662559404 * 1000).unwrap();
        let bid = 25.55;
        let ask = 36.55;

        cache.update(date, &instument, bid, ask, 0.0, 0.0).await;

        let date = Utc.timestamp_millis_opt(1662559474 * 1000).unwrap();
        let bid = 25.55;
        let ask = 36.55;

        cache.update(date, &instument, bid, ask, 0.0, 0.0).await;

        let result_bid_minute = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Minute,
                true,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;
        let result_ask_minute = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Minute,
                false,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;

        let result_bid_hour = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Hour,
                true,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;
        let result_ask_hour = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Hour,
                false,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;

        let result_bid_day = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Day,
                true,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;
        let result_ask_day = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Day,
                false,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;

        let result_bid_mount = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Month,
                true,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;
        let result_ask_mount = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Month,
                false,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;

        assert_eq!(result_bid_minute.len(), 2);
        assert_eq!(result_ask_minute.len(), 2);

        assert_eq!(result_bid_hour.len(), 1);
        assert_eq!(result_ask_hour.len(), 1);

        assert_eq!(result_bid_day.len(), 1);
        assert_eq!(result_ask_day.len(), 1);

        assert_eq!(result_bid_mount.len(), 1);
        assert_eq!(result_ask_mount.len(), 1);
    }

    #[tokio::test]
    async fn test_calculation() {
        let cache = CandleBidAsksCache::new();
        let instument = String::from("EURUSD");
        let date = Utc.timestamp_millis_opt(1662559404 * 1000).unwrap();
        let bid = 25.55;
        let ask = 36.55;

        cache.update(date, &instument, bid, ask, 0.0, 0.0).await;

        let date = Utc.timestamp_millis_opt(1662559406 * 1000).unwrap();
        let bid = 60.55;
        let ask = 31.55;

        cache.update(date, &instument, bid, ask, 0.0, 0.0).await;

        let date = Utc.timestamp_millis_opt(1662559407 * 1000).unwrap();
        let bid = 50.55;
        let ask = 62.55;

        cache.update(date, &instument, bid, ask, 0.0, 0.0).await;

        let result_bid_minute = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Minute,
                true,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;
        let result_ask_minute = cache
            .get_by_date_range(
                instument.clone(),
                CandleType::Minute,
                false,
                Utc.timestamp_millis_opt(1660559404 * 1000).unwrap(),
                Utc.timestamp_millis_opt(2660559404 * 1000).unwrap(),
            )
            .await;

        assert_eq!(result_bid_minute.len(), 1);
        assert_eq!(result_ask_minute.len(), 1);

        let first_bid = result_bid_minute.first().unwrap();
        let first_ask = result_ask_minute.first().unwrap();

        assert_eq!(first_bid.open, 25.55);
        assert_eq!(first_bid.close, 50.55);
        assert_eq!(first_bid.high, 60.55);
        assert_eq!(first_bid.low, 25.55);

        assert_eq!(first_ask.open, 36.55);
        assert_eq!(first_ask.close, 62.55);
        assert_eq!(first_ask.high, 62.55);
        assert_eq!(first_ask.low, 31.55);

        let json = serde_json::to_string(first_bid);

        println!("{}", json.unwrap());

        assert_eq!(first_bid.get_candle_date(CandleType::Minute).timestamp(), 1662559380);
        assert_eq!(first_ask.get_candle_date(CandleType::Minute).timestamp(), 1662559380);
    }
}
