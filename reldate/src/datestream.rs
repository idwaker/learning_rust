extern crate chrono;

#[cfg(test)]
mod test {
    use chrono::{Datelike, Duration, NaiveDate, Weekday};

    #[test]
    fn can_detect_day_of_week() {
        let date = NaiveDate::from_ymd(2016, 3, 24);

        assert!(Weekday::Thu == date.weekday());
    }

}