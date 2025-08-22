use bevy::prelude::*;
use crate::components::{LinearVelocity, Acceleration, Friction};
use crate::resources::WorldBounds;

/// Physics integration system that applies acceleration and friction to velocity,
/// then applies velocity to transform position
pub fn physics_integration(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut LinearVelocity, &mut Acceleration, &Friction)>,
) {
    let delta = time.delta_seconds();
    
    for (mut transform, mut velocity, mut acceleration, friction) in query.iter_mut() {
        // Apply acceleration to velocity
        velocity.0 += acceleration.0 * delta;
        
        // Apply friction to velocity (exponential decay for smooth feel)
        velocity.0 *= (1.0 - friction.0).powf(delta);
        
        // Apply velocity to position
        transform.translation.x += velocity.0.x * delta;
        transform.translation.y += velocity.0.y * delta;
        
        // Reset acceleration (will be set by input system next frame)
        acceleration.0 = Vec2::ZERO;
    }
}

/// Boundary collision system that handles collisions with world bounds
pub fn boundary_collision(
    bounds: Res<WorldBounds>,
    mut query: Query<(&mut Transform, &mut LinearVelocity)>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        let position = Vec2::new(transform.translation.x, transform.translation.y);
        
        // Check X boundaries
        if position.x <= bounds.min.x || position.x >= bounds.max.x {
            velocity.0.x = -velocity.0.x; // Reverse X velocity
            // Clamp position to prevent escaping bounds
            transform.translation.x = transform.translation.x.clamp(bounds.min.x, bounds.max.x);
        }
        
        // Check Y boundaries
        if position.y <= bounds.min.y || position.y >= bounds.max.y {
            velocity.0.y = -velocity.0.y; // Reverse Y velocity
            // Clamp position to prevent escaping bounds
            transform.translation.y = transform.translation.y.clamp(bounds.min.y, bounds.max.y);
        }
    }
}
