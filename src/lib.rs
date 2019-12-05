pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

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

pub fn download_input(day: u8) -> AdventResult<String> {
    let url = &format!("https://adventofcode.com/2019/day/{}/input", day);
    reqwest::Client::new()
    .get(url)
    .header("cookie", "session=[SESSION_ID]")
    .send()?
    .text()
    .map_err(AdventError::from)
}

pub fn parse_csv<T>(mut reader: csv::Reader<&[u8]>) -> AdventResult<Input<T>>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
    AdventError: std::convert::From<<T as std::str::FromStr>::Err>,
{
    let data: AdventResult<_> = reader
        .records()
        .map(|r| r.expect("csv record"))
        .map(|r| {
            r.iter()
                .map(|d| d.parse::<T>().map_err(AdventError::from))
                .collect::<AdventResult<Vec<T>>>()
        })
        .collect();
    Ok(Input::<T> { data: data? })
}

pub fn get_input<T>(day: u8) -> AdventResult<Input<T>>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
    AdventError: std::convert::From<<T as std::str::FromStr>::Err>,
{
    let input = download_input(day)?;
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(input.as_bytes());
    parse_csv::<T>(reader)
}

pub fn get_input_with_params<T>(
    day: u8,
    has_header: bool,
    delimiter: char,
) -> AdventResult<Input<T>>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
    AdventError: std::convert::From<<T as std::str::FromStr>::Err>,
{
    let input = download_input(day)?;

    let reader = csv::ReaderBuilder::new()
        .has_headers(has_header)
        .delimiter(delimiter as u8)
        .from_reader(input.as_bytes());

    parse_csv::<T>(reader)
}

pub struct Input<T> {
    data: Vec<Vec<T>>,
}

impl<T: Clone> Input<T> {
    fn first_row(self: Self) -> Vec<T> {
        self.data[0].to_vec()
    }

    fn first_column(self: Self) -> Vec<T> {
        self.data.iter().map(|v| v[0].clone()).collect()
    }
}
