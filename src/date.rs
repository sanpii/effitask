pub fn today() -> chrono::NaiveDate {
    chrono::Local::now().date_naive()
}

pub fn from_glib(value: gtk::glib::DateTime) -> chrono::NaiveDate {
    let y = value.year();
    let m = value.month() as u32;
    let d = value.day_of_month() as u32;

    chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap()
}
