use super::location::Location;
use super::player::Player;

pub struct Game {
    pub id: String,
    pub name: String,
    pub center: Location,
    pub size: f32,
    pub speed: f32,
    pub created: i64,
    pub started: i64,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new(id: String, name: String, center: Location, size: f32, speed: f32, created: i64, started: i64, players: Vec<Player>) -> Game {
        Game { id, name, center, size, speed, created, started, players }
    }
}
