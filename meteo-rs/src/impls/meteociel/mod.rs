pub struct MeteoCielCity<'a> {
    pub(crate) name: &'a str,
    pub(crate) postal_code: u16,
}

impl<'a> MeteoCielCity<'a> {
    pub fn new(name: &'a str, postal_code: u16) -> Self {
        Self {
            name,
            postal_code,
        }
    }
}
