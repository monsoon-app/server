#[derive(Clone)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}


impl Location {
    pub fn new(latitude: f32, longitude: f32) -> Location {
        Location { latitude, longitude }
    }
}

