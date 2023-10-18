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
    pub fn get_next_page_id(&self) -> String {
        if self.is_desc() {
            panic!("Desc ordering not supported")
        }

        if self.from_date >= self.to_date || self.last_item_no >= self.limit {
            return self.from_date.timestamp_millis().to_string();
        }

        let remaining_item_count = self.limit - self.last_item_no;
        let candle_duration = self.candle_type.get_duration(self.from_date);
        let total_duration = candle_duration * remaining_item_count as i32;
        let mut from_date = self.from_date;
        from_date += total_duration + candle_duration;

        from_date.timestamp_millis().to_string()
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
        let pager = CandlePager {
            instrument: "test".to_string(),
            candle_type: CandleType::Minute,
            from_date: Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(),
            to_date: Utc.with_ymd_and_hms(2000, 1, 2, 0, 0, 0).unwrap(),
            page_id: None,
            limit: 3,
            last_item_no: 0,
        };

        assert_eq!(pager.get_next_page_id(), "946685040000");
    }
}
