#[allow(unused_imports)]
use meteo_rs::{
    forecast::{Every3h, Every6h, ForecastProvider},
    impls::meteo_parapente::MeteoParapente,
    impls::meteociel::{Arome, GFS, IconD2, IconEU, MeteoCiel, WRF},
    location::Location,
};

#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    // let res = MeteoCiel::<Every6h, GFS>::get_forecast(Location::new("", 38000))
    //     .await
    //     .unwrap();
    let res = MeteoParapente::get_forecast(Location::new("", 38000, 45.3275, 5.2748))
        .await
        .unwrap();
    println!("{res}");
    Ok(())
}
