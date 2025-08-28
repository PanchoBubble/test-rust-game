use bevy::prelude::*;

/// World boundaries resource defining the playable area
#[derive(Resource, Debug)]
pub struct WorldBounds {
    pub min: Vec2,
    pub max: Vec2,
    pub friction: f32,
    pub bounce_factor: f32,
}

const DEFAULT_FRICTION: f32 = 0.1;
impl WorldBounds {
    /// Create world bounds from window dimensions with optional margin
    pub fn from_window_size(width: f32, height: f32, margin: f32) -> Self {
        let half_width = (width / 2.0) - margin;
        let half_height = (height / 2.0) - margin;

        Self {
            min: Vec2::new(-half_width, -half_height),
            max: Vec2::new(half_width, half_height),
            friction: DEFAULT_FRICTION,
            bounce_factor: 2.0,
        }
    }

    /// Create default bounds for a typical game window
    pub fn default_bounds() -> Self {
        Self::from_window_size(1280.0, 720.0, 50.0)
    }

    /// Check if a position is within bounds
    pub fn contains(&self, position: Vec2) -> bool {
        position.x >= self.min.x
            && position.x <= self.max.x
            && position.y >= self.min.y
            && position.y <= self.max.y
    }

    /// Clamp a position to be within bounds
    pub fn clamp_position(&self, position: Vec2) -> Vec2 {
        Vec2::new(
            position.x.clamp(self.min.x, self.max.x),
            position.y.clamp(self.min.y, self.max.y),
        )
    }
}

impl Default for WorldBounds {
    fn default() -> Self {
        Self::default_bounds()
    }
}
