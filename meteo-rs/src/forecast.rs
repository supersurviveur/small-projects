use std::fmt::Display;

use tabled::{
    builder::Builder,
    settings::{Alignment, Settings, Style, object::Rows},
};

use crate::location::Location;

#[derive(Debug)]
pub enum ForecastError {
    ReqwestError(reqwest::Error),
    ScrappingError(String),
    CityNotFound,
}

impl From<reqwest::Error> for ForecastError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

pub trait Frequency {}

macro_rules! frequencies {
    ($($frequency:ident),*) => {
        $(
            pub struct $frequency;

            impl Frequency for $frequency {}
        )*
    };
}

frequencies! {
    Daily,
    Hourly,
    Every3h,
    Every6h
}

pub struct ForecastList<T: Forecast>(Box<[T]>);

impl<T: Forecast> ForecastList<T> {
    pub fn new<L: Into<Box<[T]>>>(forcast_list: L) -> Self {
        Self(forcast_list.into())
    }
}

impl<T: Forecast> Display for ForecastList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = Builder::new();
        let settings = Settings::default().with(Style::rounded());
        builder.push_record(T::HEADER.iter().copied());
        for forecast in &self.0 {
            builder.push_record(forecast.get_printed_array());
        }
        let mut table = builder.build();
        table.with(settings);
        table.with(Alignment::right());
        table.modify(Rows::first(), Alignment::center());
        write!(f, "{table}")?;
        Ok(())
    }
}

pub trait ForecastProvider<T: Forecast> {
    fn get_forecast(
        location: Location,
    ) -> impl Future<Output = Result<ForecastList<T>, ForecastError>>;
}

pub trait Forecast {
    const HEADER: &[&str];

    fn get_printed_array(&self) -> Box<[String]>;
}
