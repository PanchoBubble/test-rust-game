use bevy::prelude::*;
use crate::components::*;

/// Basic component intersection queries demonstrating tuple-based entity selection
/// These examples show how Bevy automatically creates intersections when multiple 
/// components are requested in a single query

/// Example 1: Simple Intersection - Entities with both Player and LinearVelocity
pub fn query_player_with_velocity(
    query: Query<(&Player, &LinearVelocity)>
) {
    println!("=== Players with Velocity ===");
    for (_player, velocity) in query.iter() {
        println!("Player entity has velocity: {:?}", velocity.0);
    }
}

/// Example 2: Three-Component Intersection - Player entities with full physics
pub fn query_player_physics_full(
    query: Query<(&Player, &LinearVelocity, &Acceleration, &Friction)>
) {
    println!("=== Players with Full Physics ===");
    for (_player, velocity, acceleration, friction) in query.iter() {
        println!("Player physics - vel: {:?}, accel: {:?}, friction: {}", 
                velocity.0, acceleration.0, friction.0);
    }
}

/// Example 3: Mixed Data Access - Transform + Physics Components
pub fn query_transform_physics(
    query: Query<(&Transform, &LinearVelocity, &Acceleration)>
) {
    println!("=== Transform + Physics ===");
    for (transform, velocity, acceleration) in query.iter() {
        let pos = transform.translation;
        println!("Entity at ({:.2}, {:.2}) - vel: {:?}, accel: {:?}", 
                pos.x, pos.y, velocity.0, acceleration.0);
    }
}

/// Example 4: Entity ID Access with Components
pub fn query_entity_with_components(
    query: Query<(Entity, &Player, &Transform)>
) {
    println!("=== Entity IDs with Components ===");
    for (entity, _player, transform) in query.iter() {
        println!("Entity {:?} (Player) at position: {:?}", 
                entity, transform.translation);
    }
}

/// With/Without Filter Examples
/// These demonstrate how to filter entities based on component presence/absence

/// Example 5: With Filter - Only Players that have acceleration
pub fn query_players_with_acceleration(
    query: Query<&Transform, (With<Player>, With<Acceleration>)>
) {
    println!("=== Players WITH Acceleration ===");
    for transform in query.iter() {
        println!("Player with acceleration at: {:?}", transform.translation);
    }
}

/// Example 6: Without Filter - Entities with velocity but without Player marker
pub fn query_non_player_velocity(
    query: Query<&LinearVelocity, Without<Player>>
) {
    println!("=== Non-Player Entities with Velocity ===");
    for velocity in query.iter() {
        println!("Non-player velocity: {:?}", velocity.0);
    }
}

/// Example 7: Complex Filter Combinations - Multiple With/Without conditions
pub fn query_complex_filter(
    query: Query<(&Transform, &LinearVelocity), (With<Player>, With<Friction>, Without<Acceleration>)>
) {
    println!("=== Players with Friction but NO Acceleration ===");
    for (transform, velocity) in query.iter() {
        println!("Filtered player at {:?} with velocity {:?}", 
                transform.translation, velocity.0);
    }
}

/// Example 8: Mutable Access with Filters
pub fn query_mutable_filtered(
    mut query: Query<(&mut LinearVelocity, &Friction), With<Player>>
) {
    println!("=== Modifying Player Velocities ===");
    for (mut velocity, friction) in query.iter_mut() {
        // Example: Applying friction directly in query
        velocity.0 *= 1.0 - friction.0 * 0.1;
        println!("Applied friction, new velocity: {:?}", velocity.0);
    }
}

/// Example 9: Change Detection - Recently added or modified components
pub fn query_change_detection(
    query: Query<(Entity, &LinearVelocity), (Added<Player>, With<LinearVelocity>)>,
    changed_query: Query<(Entity, &LinearVelocity), Changed<LinearVelocity>>
) {
    println!("=== Change Detection ===");
    
    // Entities that just got the Player component
    for (entity, velocity) in query.iter() {
        println!("Newly added Player entity {:?} with velocity: {:?}", 
                entity, velocity.0);
    }
    
    // Entities whose velocity recently changed
    for (entity, velocity) in changed_query.iter() {
        println!("Entity {:?} velocity changed to: {:?}", 
                entity, velocity.0);
    }
}

/// Example 10: Multiple Query Parameters in One System
pub fn query_multiple_types(
    players: Query<(&Transform, &LinearVelocity), With<Player>>,
    all_physics: Query<(&LinearVelocity, &Acceleration), Without<Player>>
) {
    println!("=== Multiple Query Types ===");
    
    println!("Player entities:");
    for (transform, velocity) in players.iter() {
        println!("  Player at {:?}: {:?}", transform.translation, velocity.0);
    }
    
    println!("Non-player physics entities:");
    for (velocity, acceleration) in all_physics.iter() {
        println!("  Non-player vel: {:?}, accel: {:?}", velocity.0, acceleration.0);
    }
}

/// Bundle all example systems for easy registration
pub struct QueryExampleSystems;

impl QueryExampleSystems {
    pub fn add_to_app(app: &mut App) -> &mut App {
        app.add_systems(Update, (
            query_player_with_velocity,
            query_player_physics_full,
            query_transform_physics,
            query_entity_with_components,
            query_players_with_acceleration,
            query_non_player_velocity,
            query_complex_filter,
            query_mutable_filtered,
            query_change_detection,
            query_multiple_types,
        ))
    }
}
