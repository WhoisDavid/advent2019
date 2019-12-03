pub mod day1;
pub mod day2;
pub mod day3;

use csv;
use reqwest;
use std::{error, fmt, num, str::FromStr};

#[derive(Debug)]
pub enum AdventError {
    Request(reqwest::Error),
    Parse(std::num::ParseIntError),
    InvalidValue,
    Infallible(std::convert::Infallible),
}

impl error::Error for AdventError {}

impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdventError::Request(ref err) => write!(f, "Request failed with error: {}", err),
            AdventError::Parse(ref err) => write!(f, "Invalid Int: {}", err),
            AdventError::InvalidValue => write!(f, "Invalid value"),
            AdventError::Infallible(_) => write!(f, "Impossible!"),
        }
    }
}

impl From<reqwest::Error> for AdventError {
    fn from(err: reqwest::Error) -> AdventError {
        AdventError::Request(err)
    }
}

impl From<num::ParseIntError> for AdventError {
    fn from(err: num::ParseIntError) -> AdventError {
        AdventError::Parse(err)
    }
}

impl From<std::convert::Infallible> for AdventError {
    fn from(err: std::convert::Infallible) -> AdventError {
        AdventError::Infallible(err)
    }
}

pub type AdventResult<T> = std::result::Result<T, AdventError>;

pub fn get_input<T>(day: u8) -> AdventResult<Vec<Vec<T>>>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
    AdventError: std::convert::From<<T as std::str::FromStr>::Err>,
{
    let url = &format!("https://adventofcode.com/2019/day/{}/input", day);
    let input = reqwest::Client::new()
    .get(url)
    .header("cookie", "session=[SESSION_ID]")
    .send()?
    .text()?;

    let records: AdventResult<Vec<Vec<T>>> = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(input.as_bytes())
        .records()
        .map(|r| r.expect("csv record"))
        .map(|r| {
            r.iter()
                .map(|d| d.parse::<T>().map_err(|err| AdventError::from(err)))
                .collect::<AdventResult<Vec<T>>>()
        })
        .collect();

    records
}

pub fn get_input_single_row<T>(day: u8) -> AdventResult<Vec<T>>
where
    T: FromStr + std::clone::Clone,
    <T as FromStr>::Err: fmt::Debug,
    AdventError: std::convert::From<<T as std::str::FromStr>::Err>,
{
    let input = get_input::<T>(day)?;
    Ok(input[0].to_vec())
}

pub fn get_input_single_col<T>(day: u8) -> AdventResult<Vec<T>>
where
    T: FromStr + std::clone::Clone,
    <T as FromStr>::Err: fmt::Debug,
    AdventError: std::convert::From<<T as std::str::FromStr>::Err>,
{
    let input = get_input::<T>(day)?;
    Ok(input.iter().map(|v| v[0].clone()).collect())
}
