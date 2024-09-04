use chrono::NaiveDate;

pub fn naive_date_2023_11_23() -> NaiveDate {
    NaiveDate::from_ymd_opt(2023, 11, 23).expect("unable to create NaiveDate from year, month, day")
}
