use meteo_rs::impls::meteociel::MeteoCielCity;



#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {body:?}");

    MeteoCielCity::new("test", 0);
    Ok(())
}
