
#[derive(Debug)]
pub struct ForecastError;

pub trait ForecastList<T: Forecast> {
    fn get_forecast(&self, date: Date) -> Result<T, ForecastError>;
}

pub trait Forecast {
    
}
