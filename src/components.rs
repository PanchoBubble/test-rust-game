use bevy::prelude::*;

/// Marker component for the player cube
#[derive(Component, Default, Debug)]
pub struct Player;

/// Linear velocity component for physics entities
#[derive(Component, Default, Debug)]
pub struct LinearVelocity(pub Vec2);

impl LinearVelocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
    
    pub fn zero() -> Self {
        Self(Vec2::ZERO)
    }
}

/// Acceleration component for physics entities
#[derive(Component, Default, Debug)]
pub struct Acceleration(pub Vec2);

impl Acceleration {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
    
    pub fn zero() -> Self {
        Self(Vec2::ZERO)
    }
}

/// Friction coefficient component (0.0 = no friction, 1.0 = maximum friction)
#[derive(Component, Debug)]
pub struct Friction(pub f32);

impl Default for Friction {
    fn default() -> Self {
        Self(0.95) // Default friction value for good game feel
    }
}

impl Friction {
    pub fn new(friction: f32) -> Self {
        Self(friction.clamp(0.0, 1.0))
    }
}
