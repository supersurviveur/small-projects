use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use futures::future::join_all;
use serde::Deserialize;
use serde_big_array::BigArray;
use serde_json::Value;
use serde_json::Value::Object;

use crate::{
    forecast::{Forecast, ForecastError, ForecastList, ForecastProvider},
    location::Location,
};

const BASE_URL: &str = "https://data0.meteo-parapente.com";

pub struct MeteoParapente;

const DATA_COUNT: usize = 40;

#[derive(Debug)]
pub struct MeteoParapenteForecast {
    pub date: DateTime<Local>,
    pub z: [f32; DATA_COUNT],
    pub wind_speed: [f32; DATA_COUNT],
    pub wind_direction: [f32; DATA_COUNT],
    pub ter: f32,
    pub pblh: f32,
    pub rain_level: f32,
    pub cloud_opacity_low: f32,
    pub cloud_opacity_middle: f32,
    pub cloud_opacity_high: f32,
    pub cloud_opacity: [f32; DATA_COUNT],
    pub ths: [f32; DATA_COUNT],
    pub thr: [u32; DATA_COUNT],
}

#[derive(Debug, Deserialize)]
struct MeteoParapenteForecastInner {
    #[serde(with = "BigArray")]
    z: [f32; DATA_COUNT],
    #[serde(with = "BigArray")]
    umet: [f32; DATA_COUNT],
    #[serde(with = "BigArray")]
    vmet: [f32; DATA_COUNT],
    ter: f32,
    pblh: f32,
    raintot: f32,
    cfracl: f32,
    cfracm: f32,
    cfrach: f32,
    #[serde(with = "BigArray")]
    cldfra: [f32; DATA_COUNT],
    #[serde(with = "BigArray")]
    ths: [f32; DATA_COUNT],
    #[serde(with = "BigArray")]
    thr: [u32; DATA_COUNT],
}

impl Forecast for MeteoParapenteForecast {
    const HEADER: &[&str] = &["Date"];

    fn get_printed_array(&self) -> Box<[String]> {
        Box::new([self.date.format("%d-%m-%Y %H:%M").to_string()])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MeteoParapenteRun {
    run: u32,
    date: DateTime<Local>,
    pub update: DateTime<Local>,
}

impl MeteoParapente {
    async fn get_runs() -> Result<Vec<MeteoParapenteRun>, ForecastError> {
        let body = reqwest::get(format!("{BASE_URL}/status.php"))
            .await?
            .text()
            .await?;
        Ok(serde_json::from_str::<Value>(&body).unwrap()["france"]
            .take()
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|run| {
                if run.get("status").unwrap().as_str().unwrap() == "complete" {
                    Some(MeteoParapenteRun {
                        run: run["run"].as_str().unwrap().parse().unwrap(),
                        date: Local
                            .from_local_datetime(
                                &NaiveDate::parse_from_str(run["day"].as_str().unwrap(), "%Y%m%d")
                                    .unwrap()
                                    .and_time(NaiveTime::MIN),
                            )
                            .single()
                            .unwrap(),
                        update: Local
                            .from_local_datetime(
                                &NaiveDateTime::parse_from_str(
                                    run["update"].as_str().unwrap(),
                                    "%Y-%m-%dT%H:%M:%SZ",
                                )
                                .unwrap(),
                            )
                            .single()
                            .unwrap(),
                    })
                } else {
                    None
                }
            })
            .collect())
    }
    async fn get_last_run() -> Result<MeteoParapenteRun, ForecastError> {
        Ok(Self::get_runs().await?.into_iter().next().unwrap())
    }
    async fn get_latest_runs() -> Result<Vec<MeteoParapenteRun>, ForecastError> {
        let mut current_date = Default::default();
        let mut res = Self::get_runs().await?;
        res.retain(|run| {
            if run.date == current_date {
                false
            } else {
                current_date = run.date;
                true
            }
        });
        Ok(res)
    }
}
impl ForecastProvider<MeteoParapenteForecast> for MeteoParapente {
    async fn get_forecast<'a>(
        location: Location<'a>,
    ) -> Result<ForecastList<MeteoParapenteForecast>, ForecastError> {
        let runs = Self::get_latest_runs().await?;

        let mut forecast = vec![];

        let mut tasks = Vec::new();
        for run in runs {
            let current_date = run.date;
            let body = reqwest::get(format!(
                "{BASE_URL}/data.php?run={}&date={}&location={},{}&plot=windgram",
                run.run,
                run.date.format("%Y%m%d"),
                location.longitude,
                location.latitude
            ));
            let get_run_forecast =
                async |current_date: DateTime<Local>| -> Result<_, ForecastError> {
                    let body = body.await?.text().await?;
                    let json = serde_json::from_str::<Value>(&body).unwrap()["data"].take();
                    let res = match json {
                        Object(map) => map,
                        _ => unreachable!(),
                    };
                    let mut result = Vec::new();
                    for (time, data) in res.into_iter() {
                        let time = NaiveTime::parse_from_str(&time, "%H:%M").unwrap();
                        let current_date = current_date.with_time(time).single().unwrap();

                        let inner: MeteoParapenteForecastInner =
                            serde_json::from_value(data).unwrap();

                        let mut wind_speed = [0.; DATA_COUNT];
                        let mut wind_direction = [0.; DATA_COUNT];
                        for i in 0..DATA_COUNT {
                            let u = inner.umet[i];
                            let v = inner.vmet[i];
                            wind_speed[i] = (u * u + v * v).sqrt();
                            wind_direction[i] = (u / v).atan();
                        }

                        result.push(MeteoParapenteForecast {
                            date: current_date,
                            z: inner.z,
                            wind_speed,
                            wind_direction,
                            ter: inner.ter,
                            pblh: inner.pblh,
                            rain_level: inner.raintot,
                            cloud_opacity_low: inner.cfracl,
                            cloud_opacity_middle: inner.cfracm,
                            cloud_opacity_high: inner.cfrach,
                            cloud_opacity: inner.cldfra,
                            ths: inner.ths,
                            thr: inner.thr,
                        })
                    }
                    Ok(result)
                };
            tasks.push(tokio::spawn(get_run_forecast(current_date)));
        }

        for task in join_all(tasks).await {
            forecast.extend(task.unwrap()?)
        }

        Ok(ForecastList::new(forecast))
    }
}
