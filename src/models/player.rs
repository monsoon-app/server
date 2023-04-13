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
}

