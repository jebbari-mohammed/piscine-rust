use chrono::{NaiveDate, Weekday};
use chrono::Datelike;

pub fn middle_day(year: u32) -> Option<Weekday> {
    let mut is_leap = false;
    if year % 4 == 0 {
        is_leap = true;
        if year % 100 == 0 {
            is_leap = false;
            if year % 400 == 0 {
                is_leap = true;
            }
        }
    }

    if is_leap {
        return None;
    }
    let date = NaiveDate::from_ymd_opt(year as i32, 7, 2).unwrap();
    Some(date.weekday())
}
