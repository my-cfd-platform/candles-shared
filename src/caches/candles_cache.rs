use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::models::{candle_price::CandlePrice, candle::BidAskCandle, candle_type::CandleType};

pub struct CandlesCache {
    candles_by_ids: HashMap<String, BidAskCandle>,
    pub candle_types: [CandleType; 4],
}

impl CandlesCache {
    pub fn new() -> Self {
        Self {
            candles_by_ids: HashMap::new(),
            candle_types: [
                CandleType::Minute,
                CandleType::Hour,
                CandleType::Day,
                CandleType::Month,
            ],
        }
    }

    pub fn contains(&self, candle_id: &str) -> bool {
        self.candles_by_ids.contains_key(candle_id)
    }

    pub fn insert(&mut self, candle: BidAskCandle) {
        println!(
            "insert candle {}: {} {}; {} total candles",
            candle.instrument,
            candle.datetime.to_rfc3339(),
            candle.get_id(),
            self.candles_by_ids.len() + 1
        );

        self.candles_by_ids.insert(candle.get_id(), candle);
    }

    pub fn create_or_update(
        &mut self,
        datetime: DateTime<Utc>,
        instrument: &str,
        bid: f64,
        ask: f64,
    ) {
        for candle_type in self.candle_types.iter() {
            let candle_date = candle_type.candle_date(datetime);
            let id = BidAskCandle::generate_id(instrument, candle_type, candle_date);
            let candle = self.candles_by_ids.get_mut(&id);

            if let Some(candle) = candle {
                candle.update(datetime, bid, ask);
            } else {
                println!(
                    "create candle {}: {} {}; {} total count",
                    instrument.to_owned(),
                    datetime.to_rfc3339(),
                    id,
                    self.candles_by_ids.len() + 1
                );

                self.candles_by_ids.insert(
                    id,
                    BidAskCandle {
                        ask_price: CandlePrice::new(datetime, ask),
                        bid_price: CandlePrice::new(datetime, bid),
                        candle_type: candle_type.clone(),
                        instrument: instrument.to_owned(),
                        datetime: candle_date,
                    },
                );
            }
        }
    }

    pub fn get_after(&self, date: DateTime<Utc>) -> Option<Vec<BidAskCandle>> {
        if self.candles_by_ids.len() == 0 {
            return None;
        }

        let candle_dates = self.calculate_candle_dates(date);

        let candles = self
            .candles_by_ids
            .iter()
            .filter_map(|(_id, candle)| {
                let current_date = candle_dates[candle.candle_type.to_owned() as usize];

                if candle.datetime >= current_date {
                    Some(candle.clone())
                } else {
                    None
                }
            })
            .collect();

        Some(candles)
    }

    pub fn remove_after(&mut self, date: DateTime<Utc>) -> i32 {
        let dates = self.calculate_candle_dates(date);
        let mut removed_count = 0;

        self.candles_by_ids.retain(|_id, candle| {
            let current_date = dates[candle.candle_type.to_owned() as usize];

            if candle.datetime <= current_date {
                removed_count += 1;
                false
            } else {
                true
            }
        });

        removed_count
    }

    fn calculate_candle_dates(&self, datetime: DateTime<Utc>) -> Vec<DateTime<Utc>> {
        let mut dates = Vec::with_capacity(self.candle_types.len());

        for candle_type in self.candle_types.iter() {
            let candle_date = candle_type.candle_date(datetime);
            let index = candle_type.to_owned() as usize;
            dates.insert(index, candle_date);
        }

        return dates;
    }
}