use super::location::Location;

pub struct Player {
    pub name: String,
    pub location: Location,
    pub state: i32,
    pub gamemode: i32
}

impl Player {
    pub fn new(name: String, location: Location, state: i32, gamemode: i32) -> Player {
        Player { name, location, state, gamemode }
    }
    pub fn update(&mut self, latitude: f32, longitude: f32) {
        self.location.update(latitude, longitude)
        // todo: check in relation to the storm
    }
}

