use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use crate::GAMES;
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

    pub fn tick (code: &str) {
        let code = code.to_owned();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(50));
                let mut map = GAMES.write().unwrap();
                if let Some(game)  = map.get_mut(&code) {
                    for player in &mut game.players {
                        if game.center.distance_to(&player.location) > game.size {
                            player.state = 1; // out
                            // maybe check if they are close to the hunter as well?
                        }
                    }
                    game.size = game.size - (game.speed/1200.0);
                } else {
                    break
                }
            }
        });
    }


    pub fn add_player(&mut self, player: Player) {
        self.players.push(player)
    }

    pub fn get_player(&mut self, name: &str) -> Option<&mut Player> {
        for player in &mut self.players {
            if player.name == name {
                return Some(player);
            }
        }
        None
    }
}
