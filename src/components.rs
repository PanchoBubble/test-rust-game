use bevy::prelude::*;

/// Marker component for the player cube
#[derive(Component, Debug)]
pub struct Player {
    pub acceleration: Vec2,
    pub velocity: Vec2,
    pub friction: f32,
}

impl Player {
    pub fn new(acceleration: Vec2, velocity: Vec2, friction: f32) -> Self {
        Self {
            acceleration,
            velocity,
            friction,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            acceleration: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            friction: 0.95,
        }
    }
}
