//! Library for generating relative datestream
//!
//! This is some long documentation

extern crate chrono;

use chrono::{Datelike, Duration, NaiveDate, Weekday};

/// Allows iteration of arbitary range
///
/// This is some long description of below code.
pub struct DateRangeIterator<F> {
    d: NaiveDate,
    f: F,
}

// what is where F: FnMut .... ?
impl<F> Iterator for DateRangeIterator<F>
    where F: FnMut(&NaiveDate) -> Option<NaiveDate>
{
    type Item = NaiveDate;

    fn next(&mut self) -> Option<NaiveDate> {
        let ret = (self.f)(&self.i);
        if ret.is_some() { self.i = ret.unwrap(); }
        ret
    }
}


#[cfg(test)]
mod test {
    use chrono::{Datelike, Duration, NaiveDate, Weekday};

    #[test]
    fn can_detect_day_of_week() {
        let date = NaiveDate::from_ymd(2016, 3, 24);

        assert!(Weekday::Thu == date.weekday());
    }

}