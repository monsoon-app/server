use std::f32::consts::PI;

#[derive(Clone)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}


impl Location {
    pub fn new(latitude: f32, longitude: f32) -> Location {
        Location { latitude, longitude }
    }

    pub fn update(&mut self, latitude: f32, longitude: f32) {
        self.latitude = latitude;
        self.longitude = longitude;
    }

    pub fn distance_to(&self, other: &Location) -> f32 {
        // https://en.wikipedia.org/wiki/Haversine_formula
        let dlat = &other.latitude.to_radians() - &self.latitude.to_radians();
        let dlon = &other.longitude.to_radians() - &self.longitude.to_radians();

        let a = (dlat / 2.0).sin().powi(2) + &self.latitude.to_radians().cos() * &other.latitude.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        // radius of earth
        6366707.0195 * c
    }
}

