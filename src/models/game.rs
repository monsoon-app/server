use std::time::{SystemTime, UNIX_EPOCH};
use super::location::Location;
use super::player::Player;

pub struct Game {
    pub id: String,
    pub name: String,
    pub center: Location,
    pub size: f32,
    pub speed: f32,
    pub created: i64,
    pub phase: i64,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new(id: String, name: String, center: Location, size: f32, speed: f32, created: i64, phase: i64, players: Vec<Player>) -> Game {
        Game { id, name, center, size, speed, created, phase, players }
    }

    pub fn start(&mut self) {
        self.phase = 1;
        self.created = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player)
    }
}
