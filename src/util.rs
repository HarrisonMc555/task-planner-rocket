use rocket::request::FromFormValue;
use rocket::http::RawStr;
use std::ops::Deref;

use chrono::NaiveDate as ChronoNaiveDate;

pub struct NaiveDate(pub ChronoNaiveDate);

impl<'v> FromFormValue<'v> for NaiveDate {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) ->
        Result<NaiveDate, &'v RawStr> {
            eprintln!("Parsing form_value {:?} into NaiveDate", form_value);
            let form_string = &form_value
                .percent_decode()
                .map_err(|_| form_value)?;
            let date = ChronoNaiveDate::parse_from_str(form_string, "%Y-%m-%d")
                .map_err(|_| form_value)?;
            Ok(NaiveDate(date))
    }
}

impl Deref for NaiveDate {
    type Target = ChronoNaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
