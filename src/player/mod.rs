pub mod camera;
pub mod health;

pub use crate::player::camera::*;
pub use crate::player::health::*;

pub struct Player {
    health: Health,
}

impl Player {
    pub fn new(initial_health: u32) -> Self {
        Player { health: Health::new(initial_health) }
    }
}
