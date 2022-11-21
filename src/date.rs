pub fn today() -> chrono::naive::NaiveDate {
    chrono::Local::now().date_naive()
}
