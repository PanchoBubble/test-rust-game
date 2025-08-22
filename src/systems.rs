use crate::components::Player;
use crate::resources::WorldBounds;
use bevy::prelude::*;

/// World friction system that applies global friction to all entities
pub fn world_friction(time: Res<Time>, bounds: Res<WorldBounds>, mut query: Query<&mut Player>) {
    let delta = time.delta_seconds();

    for mut player in query.iter_mut() {
        // Apply world friction
        player.velocity *= (1.0 - bounds.friction).powf(delta);
    }
}

/// Physics integration system that applies acceleration and entity friction to velocity,
/// then applies velocity to transform position
pub fn player_physics_integration(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let delta = time.delta_seconds();

    for (mut transform, mut player) in query.iter_mut() {
        // Apply acceleration to velocity
        let player_acceleration = player.acceleration;
        player.velocity += player_acceleration * delta;

        // Apply entity-specific friction to velocity
        let player_friction = player.friction;
        player.velocity *= (1.0 - player_friction).powf(delta);

        // Apply velocity to position
        transform.translation.x += player.velocity.x * delta;
        transform.translation.y += player.velocity.y * delta;

        // Reset acceleration (will be set by input system next frame)
        player.acceleration = Vec2::ZERO;
    }
}

/// Boundary collision system that handles collisions with world bounds
pub fn boundary_collision(
    bounds: Res<WorldBounds>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        let position = Vec2::new(transform.translation.x, transform.translation.y);

        // Check X boundaries
        if position.x <= bounds.min.x || position.x >= bounds.max.x {
            player.velocity.x = player.velocity.x * bounds.bounce_factor;
            player.velocity.x = -player.velocity.x; // Reverse X velocity

            transform.translation.x = transform.translation.x.clamp(bounds.min.x, bounds.max.x);
        }

        // Check Y boundaries
        if position.y <= bounds.min.y || position.y >= bounds.max.y {
            player.velocity.y = player.velocity.y * bounds.bounce_factor;
            player.velocity.y = -player.velocity.y; // Reverse Y velocity
            transform.translation.y = transform.translation.y.clamp(bounds.min.y, bounds.max.y);
        }
    }
}
