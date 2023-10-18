use crate::models::candle::BidAskCandle;
use crate::models::candle_type::CandleType;
use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug)]
pub struct CandlePager {
    pub instrument: String,
    pub candle_type: CandleType,
    pub from_date: DateTime<Utc>,
    pub to_date: DateTime<Utc>,
    pub page_id: Option<String>,
    pub limit: usize,
    pub last_item_no: usize,
}

impl CandlePager {
    pub fn get_next_page_id(&self) -> Option<String> {
        if self.is_desc() {
            panic!("Desc ordering not supported")
        }

        let total_items_count = self
            .candle_type
            .get_dates_count(self.from_date, self.to_date);

        if self.limit > total_items_count { // there is only one page
            return None;
        }

        let remaining_item_count = self.limit - self.last_item_no;
        let candle_duration = self.candle_type.get_duration(self.from_date);
        let total_duration = candle_duration * remaining_item_count as i32;
        let from_date = self.from_date + total_duration + candle_duration;

        Some(from_date.timestamp_millis().to_string())
    }

    pub fn move_page_id(&mut self) -> Option<String> {
        let next_page_id = self.get_next_page_id()?;
        let date = Utc
            .timestamp_millis_opt(next_page_id.parse().unwrap())
            .unwrap();
        self.from_date = date;

        Some(next_page_id)
    }

    pub fn move_candle_id(&mut self) -> Option<String> {
        if self.is_desc() {
            panic!("Desc ordering not supported")
        }

        if self.last_item_no >= self.limit {
            return None;
        }

        if let Some(page_id) = self.page_id.as_ref() {
            let page_id = page_id.parse::<i64>().expect("Failed to parse page_id");
            self.from_date = Utc.timestamp_millis_opt(page_id).unwrap()
        }

        if self.from_date >= self.to_date {
            return None;
        }

        let id = BidAskCandle::generate_id(&self.instrument, &self.candle_type, self.from_date);
        self.last_item_no += 1;
        self.from_date = self.from_date + self.candle_type.get_duration(self.from_date);

        Some(id)
    }

    pub fn get_page_candle_ids(&self) -> Vec<String> {
        if self.is_desc() {
            panic!("Desc ordering not supported")
        }

        if self.last_item_no >= self.limit {
            return vec![];
        }
        let mut from_date = self.from_date;

        if let Some(page_id) = self.page_id.as_ref() {
            let page_id = page_id.parse::<i64>().expect("Failed to parse page_id");
            from_date = Utc.timestamp_millis_opt(page_id).unwrap()
        }

        let dates_count = self
            .candle_type
            .get_dates_count(self.from_date, self.to_date);
        let limit = if self.limit > dates_count {
            dates_count
        } else {
            self.limit
        };
        let mut ids = Vec::with_capacity(limit);

        for _ in 0..limit {
            if self.from_date >= self.to_date {
                return ids;
            }

            let id = BidAskCandle::generate_id(&self.instrument, &self.candle_type, from_date);
            ids.push(id);

            from_date = from_date + self.candle_type.get_duration(from_date);
        }

        ids
    }

    pub fn is_asc(&self) -> bool {
        self.from_date < self.to_date
    }

    pub fn is_desc(&self) -> bool {
        self.from_date > self.to_date
    }
}

#[cfg(test)]
mod tests {
    use crate::models::candle_pager::CandlePager;
    use crate::models::candle_type::CandleType;
    use chrono::{TimeZone, Utc};

    #[tokio::test]
    async fn get_next_candle_id() {
        let mut pager = CandlePager {
            instrument: "test".to_string(),
            candle_type: CandleType::Minute,
            from_date: Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(),
            to_date: Utc.with_ymd_and_hms(2000, 1, 2, 0, 0, 0).unwrap(),
            page_id: None,
            limit: 2,
            last_item_no: 0,
        };

        assert_eq!(pager.move_candle_id(), Some("0test946684800".to_string()));
        assert_eq!(1, pager.last_item_no);

        assert_eq!(pager.move_candle_id(), Some("0test946684860".to_string()));
        assert_eq!(2, pager.last_item_no);

        let id = pager.move_candle_id();
        assert_eq!(id, None);
    }

    #[tokio::test]
    async fn get_next_page_id() {
        let mut pager = CandlePager {
            instrument: "test".to_string(),
            candle_type: CandleType::Minute,
            from_date: Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(),
            to_date: Utc.with_ymd_and_hms(2000, 1, 2, 0, 0, 0).unwrap(),
            page_id: None,
            limit: 3,
            last_item_no: 0,
        };

        assert_eq!(pager.get_next_page_id(), Some("946685040000".to_string()));
        _ = pager.move_candle_id();
        assert_eq!(pager.get_next_page_id(), Some("946685040000".to_string()));
        _ = pager.move_candle_id();
        _ = pager.move_candle_id();
        assert_eq!(pager.get_next_page_id(), Some("946685040000".to_string()));
        let id = pager.move_candle_id();

        println!("{:?}", id);
    }

    #[tokio::test]
    async fn get_page_candle_ids() {
        let pager = CandlePager {
            instrument: "test".to_string(),
            candle_type: CandleType::Minute,
            from_date: Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(),
            to_date: Utc.with_ymd_and_hms(2000, 1, 2, 0, 0, 0).unwrap(),
            page_id: None,
            limit: 5,
            last_item_no: 0,
        };

        let ids = pager.get_page_candle_ids();

        assert_eq!(ids.len(), pager.limit);
    }

    #[tokio::test]
    async fn test_1() {
        let pager = CandlePager {
            instrument: "BTCUSDT".to_string(),
            candle_type: CandleType::Hour,
            from_date: Utc.timestamp_millis_opt("1696547451000".parse().unwrap()).unwrap(),
            to_date: Utc.timestamp_millis_opt("1697627451000".parse().unwrap()).unwrap(),
            page_id: None,
            limit: 1500,
            last_item_no: 0,
        };

        let ids = pager.get_page_candle_ids();

        assert_eq!(300, ids.len());
        assert_eq!(None, pager.get_next_page_id());

    }
}
