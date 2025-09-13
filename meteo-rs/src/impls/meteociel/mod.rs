use crate::forecast::ForecastList;
use core::slice;
use std::{iter::Skip, marker::PhantomData};

use chrono::{DateTime, Datelike, Days, Local, NaiveTime};
use scraper::{ElementRef, Html, Selector};

use crate::{
    forecast::{Every3h, Every6h, Forecast, ForecastError, ForecastProvider, Hourly},
    location::Location,
};

const BASE_URL: &str = "https://www.meteociel.fr";

const CITIES_SELECTOR: &str = "tr.texte > td:nth-child(2) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(1) > center:nth-child(1) > center:nth-child(10) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(1) > li > a";
const TABLE_SELECTOR: &str = "tr.texte > td:nth-child(2) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(1) > center:nth-child(4) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(1) > table:nth-child(1) > tbody";

pub struct MeteoCiel<Frequency, Provider> {
    _phantom: PhantomData<(Frequency, Provider)>,
}
pub struct WRF;
pub struct GFS;
pub struct Arome;
pub struct Arpege;
pub struct IconEU;
pub struct IconD2;

#[derive(Debug)]
pub struct MCSimpleForecast {
    pub date: DateTime<Local>,
    pub temperature: u8,
    pub wind_chill: u16,
    pub wind_direction: u16,
    pub wind_average_speed: u8,
    pub wind_max_speed: u8,
    pub rain_level: f32,
    pub humidity: u8,
    pub pressure: u16,
}

impl Forecast for MCSimpleForecast {
    const HEADER: &[&str] = &[
        "Date",
        "Temp.",
        "Wind\nchill",
        "Wind\ndir.",
        "Wind av.\n speed",
        "Wind max\n speed",
        "Rain\nlevel",
        "Humidity",
        "Pressure",
    ];
    fn get_printed_array(&self) -> Box<[String]> {
        Box::new([
            self.date.format("%d-%m-%Y %H:%M").to_string(),
            format!("{}°C", self.temperature),
            format!("{}°C", self.wind_chill),
            format!("{}°", self.wind_direction),
            format!("{} km/h", self.wind_average_speed),
            format!("{} km/h", self.wind_max_speed),
            format!("{} mm", self.rain_level),
            format!("{}%", self.humidity),
            format!("{} hPa", self.pressure),
        ])
    }
}

impl<Frequency, Provider> MeteoCiel<Frequency, Provider> {
    async fn get_city<'a>(city: Location<'a>) -> Result<String, ForecastError> {
        let body = reqwest::get(if !city.name.is_empty() {
            format!(
                "{BASE_URL}/prevville.php?action=getville&ville={}",
                city.name
            )
        } else {
            format!(
                "{BASE_URL}/prevville.php?action=getville&ville={}",
                city.zip
            )
        })
        .await?
        .text()
        .await?;
        if body.contains("Transfert sur la page pr&eacute;visions ...") {
            let redirect = body
                .rsplit_once("Transfert sur la page pr&eacute;visions ...")
                .unwrap()
                .1
                .split_once("location.href=")
                .unwrap()
                .1
                .split_once(";")
                .unwrap()
                .0;
            let redirect = redirect[12..redirect.len() - 1].to_string();
            return Ok(redirect);
        } else if !city.name.is_empty() && city.zip != 0 {
            let page = Html::parse_document(&body);
            let selector = Selector::parse(CITIES_SELECTOR).unwrap();
            let cities = page.select(&selector);
            for city_elem in cities {
                let content = city_elem.inner_html();
                if content.contains(&city.zip.to_string()) && content.contains(city.name) {
                    let redirect = city_elem.attr("href").unwrap();
                    return Ok(redirect[11..].to_string());
                }
            }
        }
        Err(ForecastError::CityNotFound)
    }

    async fn get_forecast_impl<'a>(
        location: Location<'a>,
        url: &str,
    ) -> Result<ForecastList<MCSimpleForecast>, ForecastError> {
        let mut result = vec![];
        Self::exec_on_rows(location, url, |current_date, mut children| {
            let mut temperature = children.next().unwrap().inner_html();
            temperature.retain(|c| c.is_ascii_digit());
            let temperature = temperature.parse().unwrap();

            let wind_chill = children.next().unwrap().text().next().unwrap();
            let wind_chill = wind_chill.parse().unwrap();

            let mut wind_direction = children
                .next()
                .unwrap()
                .first_child()
                .unwrap()
                .value()
                .as_element()
                .unwrap()
                .attr("alt")
                .unwrap()
                .to_string();
            wind_direction.retain(|c| c.is_ascii_digit());
            let wind_direction = wind_direction.parse().unwrap();

            let wind_average_speed = children.next().unwrap().inner_html();
            let wind_average_speed = wind_average_speed.parse().unwrap();

            let wind_max_speed = children.next().unwrap().inner_html();
            let wind_max_speed = wind_max_speed.parse().unwrap();

            let mut rain_level = children.next().unwrap().inner_html();
            rain_level.retain(|c| c.is_ascii_digit() || c == '.');
            let rain_level = if !rain_level.is_empty() {
                rain_level.parse().unwrap()
            } else {
                0.0
            };

            let mut humidity = children.next().unwrap().inner_html();
            humidity.retain(|c| c.is_ascii_digit());
            let humidity = humidity.parse().unwrap();

            let mut pression = children.next().unwrap().inner_html();
            pression.retain(|c| c.is_ascii_digit());
            let pression = pression.parse().unwrap();

            result.push(MCSimpleForecast {
                date: current_date,
                temperature,
                wind_chill,
                wind_direction,
                wind_average_speed,
                wind_max_speed,
                rain_level,
                humidity,
                pressure: pression,
            });
        })
        .await?;

        Ok(ForecastList::new(result))
    }
    async fn exec_on_rows<'a, F: FnMut(DateTime<Local>, Skip<slice::Iter<ElementRef>>)>(
        location: Location<'a>,
        url: &str,
        mut row_fn: F,
    ) -> Result<(), ForecastError> {
        let mc_city = Self::get_city(location).await?;

        let body = reqwest::get(format!("{BASE_URL}{url}{mc_city}"))
            .await?
            .text()
            .await?;

        let page = Html::parse_document(&body);
        let selector = Selector::parse(TABLE_SELECTOR).unwrap();

        let table = page.select(&selector).next().unwrap();

        let mut current_date = Local::now();

        let day = table
            .child_elements()
            .nth(2)
            .unwrap()
            .children()
            .next()
            .unwrap();
        let day_number = day.children().nth(2).unwrap().value().as_text().unwrap();
        let day_number = day_number.parse().unwrap();

        current_date = current_date.with_day(day_number).unwrap();

        let mut first = true;

        for element in table.child_elements().skip(2) {
            let children: Vec<_> = element.child_elements().collect();
            let mut children = children.iter().skip(if children.len() == 11 {
                if !first {
                    current_date = current_date.checked_add_days(Days::new(1)).unwrap();
                } else {
                    first = false;
                }
                1
            } else {
                0
            });

            let time = children.next().unwrap().inner_html();

            current_date = current_date
                .with_time(NaiveTime::parse_from_str(&time, "%H:%M").unwrap())
                .unwrap();

            row_fn(current_date, children);
        }

        Ok(())
    }
}

macro_rules! meteociel_providers_impl {
    ($freq:ident, $provider:ident, $url:expr) => {
        impl ForecastProvider<MCSimpleForecast> for MeteoCiel<$freq, $provider> {
            #[inline(always)]
            async fn get_forecast<'a>(
                location: Location<'a>,
            ) -> Result<ForecastList<MCSimpleForecast>, ForecastError> {
                Self::get_forecast_impl(location, $url).await
            }
        }
    };
}
meteociel_providers_impl! { Every3h, GFS, "/previsions" }
meteociel_providers_impl! { Every6h, GFS, "/tendances" }
meteociel_providers_impl! { Hourly, WRF, "/previsions-wrf-1h" }
meteociel_providers_impl! { Every3h, WRF, "/previsions-wrf" }
meteociel_providers_impl! { Hourly, Arome, "/previsions-arome-1h" }
meteociel_providers_impl! { Every3h, Arome, "/previsions-arome" }
meteociel_providers_impl! { Hourly, Arpege, "/previsions-arpege-1h" }
meteociel_providers_impl! { Hourly, IconD2, "/previsions-icond2" }
meteociel_providers_impl! { Hourly, IconEU, "/previsions-iconeu" }
