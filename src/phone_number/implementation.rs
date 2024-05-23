use crate::phone_number::PhoneNumber;
use crate::InOutFuncs;
use std::ffi::CStr;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

impl InOutFuncs for PhoneNumber {
    fn input(input: &CStr) -> Self
    where
        Self: Sized,
    {
        input
            .to_str()
            .expect("input is not UTF8")
            .parse()
            .expect("invalid number")
    }

    fn output(&self, buffer: &mut pgrx::StringInfo) {
        buffer.push_str(&format!("{}", self))
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "+{:0width_country$} {:0width$} {:0width$}-{:0width_number$}-{:0width_number$}",
            self.country_code,
            self.city_code,
            self.first_code,
            self.second_code,
            self.third_code,
            width_country = 1,
            width = 3,
            width_number = 2,
        )
    }
}

#[derive(Debug)]
pub enum PhoneNumberError {
    InvalidCountryCode,
    InvalidCityCode,
    InvalidFirstCode,
    InvalidSecondCode,
    InvalidThirdCode,
    InvalidFormat,
}

impl FromStr for PhoneNumber {
    type Err = PhoneNumberError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn validate<F: FnOnce() -> PhoneNumberError>(
            input: Option<&str>,
            digits: usize,
            err_func: F,
        ) -> Result<u16, PhoneNumberError> {
            match input {
                Some(value) if value.len() == digits => match value.parse() {
                    Ok(value) => Ok(value),
                    Err(_) => Err(err_func()),
                },
                Some(_) => Err(err_func()),
                None => Err(PhoneNumberError::InvalidFormat),
            }
        }
        let mut parts = input.split(' ');
        let country_code = validate(parts.next(), 1, || PhoneNumberError::InvalidCountryCode)?;
        let city_code = validate(parts.next(), 3, || PhoneNumberError::InvalidCityCode)?;
        let first_code = validate(parts.next(), 3, || PhoneNumberError::InvalidFirstCode)?;
        let second_code = validate(parts.next(), 2, || PhoneNumberError::InvalidSecondCode)?;
        let third_code = validate(parts.next(), 2, || PhoneNumberError::InvalidThirdCode)?;

        if parts.next().is_some() {
            Err(PhoneNumberError::InvalidFormat)
        } else {
            Ok(PhoneNumber {
                country_code,
                city_code,
                first_code,
                second_code,
                third_code,
            })
        }
    }
}
