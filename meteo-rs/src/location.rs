#[derive(Clone)]
pub struct Location<'a> {
    pub(crate) name: &'a str,
    pub(crate) zip: u16,
    pub(crate) longitude: f32,
    pub(crate) latitude: f32,
}

impl<'a> Location<'a> {
    pub fn new(name: &'a str, zip: u16, longitude: f32, latitude: f32) -> Self {
        Self {
            name,
            zip,
            longitude,
            latitude,
        }
    }
}
