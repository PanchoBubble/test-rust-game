use bevy::prelude::*;
use crate::components::*;

/// Type aliases for common query patterns to improve readability and reduce compilation time

/// Basic physics entity query - commonly used for physics systems
pub type PhysicsQuery<'w, 's> = Query<'w, 's, (&'static mut Transform, &'static mut LinearVelocity, &'static Acceleration, &'static Friction)>;

/// Player-specific queries
pub type PlayerQuery<'w, 's> = Query<'w, 's, (&'static Transform, &'static LinearVelocity), With<Player>>;
pub type PlayerMutQuery<'w, 's> = Query<'w, 's, (&'static mut Transform, &'static mut LinearVelocity), With<Player>>;

/// Read-only physics queries for analysis/display systems
pub type PhysicsReadQuery<'w, 's> = Query<'w, 's, (&'static Transform, &'static LinearVelocity, &'static Acceleration), With<Player>>;

/// Entities with optional components
pub type OptionalPhysicsQuery<'w, 's> = Query<'w, 's, (&'static Transform, Option<&'static LinearVelocity>, Option<&'static Acceleration>)>;

/// Advanced Query Patterns with Or Filters
/// Note: Or filters are available in Bevy but have performance implications

/// Entities with either velocity OR acceleration (but not necessarily both)
pub fn query_with_or_filter(
    // In Bevy 0.11, Or filters might not be directly available, showing the pattern
    query: Query<&Transform, Or<(With<LinearVelocity>, With<Acceleration>)>>
) {
    println!("=== Entities with Velocity OR Acceleration ===");
    for transform in query.iter() {
        println!("Entity at: {:?}", transform.translation);
    }
}

/// Optional Component Patterns
/// Use sparingly due to performance impact on large entity counts

pub fn query_optional_components(
    query: OptionalPhysicsQuery
) {
    println!("=== Optional Components Query ===");
    for (transform, velocity_opt, acceleration_opt) in query.iter() {
        match (velocity_opt, acceleration_opt) {
            (Some(vel), Some(accel)) => {
                println!("Full physics entity at {:?}: vel={:?}, accel={:?}", 
                        transform.translation, vel.0, accel.0);
            },
            (Some(vel), None) => {
                println!("Velocity-only entity at {:?}: vel={:?}", 
                        transform.translation, vel.0);
            },
            (None, Some(accel)) => {
                println!("Acceleration-only entity at {:?}: accel={:?}", 
                        transform.translation, accel.0);
            },
            (None, None) => {
                println!("Transform-only entity at {:?}", transform.translation);
            }
        }
    }
}

/// Query Iteration Optimization Patterns
/// Demonstrating different iteration approaches for performance

pub fn optimized_iteration_patterns(
    mut query: Query<(&Transform, &mut LinearVelocity), With<Player>>,
    _time: Res<Time>,
) {
    // Pattern 1: Using for_each for potential vectorization
    // This is the recommended approach in modern Bevy
    query.iter_mut().for_each(|(_transform, mut velocity)| {
        // Apply some physics calculation
        velocity.0 *= 0.99; // Simple damping
    });
    
    // Pattern 2: Traditional for loop (less optimal for simple operations)
    // for (transform, mut velocity) in query.iter_mut() {
    //     velocity.0 *= 0.99;
    // }
    
    // Pattern 3: Parallel iteration (when no conflicts exist)
    // query.par_for_each_mut(32, |(transform, mut velocity)| {
    //     velocity.0 *= 0.99;
    // });
}

/// Query Filtering Utilities
/// Helper functions for common filtering scenarios

pub fn count_entities_by_components(_world: &World) {
    // This would require direct world access in a real system
    println!("=== Entity Component Statistics ===");
    
    // In a real implementation, you'd use World::query or similar
    // to get component counts and archetype information
    println!("This would show entity counts per component combination");
}

/// Archetype Analysis Helpers
/// These help understand query performance characteristics

pub fn analyze_archetype_fragmentation(
    all_entities: Query<Entity>,
    transform_entities: Query<Entity, With<Transform>>,
    velocity_entities: Query<Entity, With<LinearVelocity>>,
    physics_entities: Query<Entity, (With<Transform>, With<LinearVelocity>, With<Acceleration>)>,
) {
    println!("=== Archetype Analysis ===");
    println!("Total entities: {}", all_entities.iter().count());
    println!("Entities with Transform: {}", transform_entities.iter().count());
    println!("Entities with Velocity: {}", velocity_entities.iter().count());
    println!("Entities with full physics: {}", physics_entities.iter().count());
    
    // This gives insight into how fragmented your archetypes are
    // More fragmentation = potentially slower queries
}

/// Query Building Helpers
/// Utilities for constructing complex queries programmatically

pub struct QueryBuilder {
    // In practice, this would use Bevy's actual QueryBuilder API
    // This is a simplified example showing the pattern
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn with_component<T: Component>(self) -> Self {
        // Add component requirement
        self
    }
    
    pub fn without_component<T: Component>(self) -> Self {
        // Add component exclusion
        self
    }
    
    // In practice, this would return a proper Query type
    pub fn build(self) -> String {
        "Dynamic query built".to_string()
    }
}

/// Performance Monitoring
/// Systems to track query performance characteristics

pub fn query_performance_monitor(
    physics_query: PhysicsQuery,
    player_query: PlayerQuery,
    _time: Res<Time>,
) {
    // In a real implementation, you'd use a performance monitoring system
    let start = std::time::Instant::now();
    
    // Perform query operations
    let physics_count = physics_query.iter().count();
    let player_count = player_query.iter().count();
    
    let duration = start.elapsed();
    
    if duration.as_millis() > 1 { // Only log if took more than 1ms
        println!("Query performance - Physics: {}, Players: {}, Time: {:?}", 
                physics_count, player_count, duration);
    }
}

/// Common Query Patterns Module
pub mod patterns {
    use super::*;
    
    /// Get all entities with a specific component combination
    pub fn entities_with_components<T1: Component, T2: Component>(
        query: Query<Entity, (With<T1>, With<T2>)>
    ) -> Vec<Entity> {
        query.iter().collect()
    }
    
    /// Check if any entities exist with a specific component combination
    pub fn has_entities_with_components<T1: Component, T2: Component>(
        query: Query<Entity, (With<T1>, With<T2>)>
    ) -> bool {
        !query.is_empty()
    }
    
    /// Count entities matching a specific pattern
    pub fn count_matching_entities<T1: Component, T2: Component>(
        query: Query<Entity, (With<T1>, With<T2>)>
    ) -> usize {
        query.iter().count()
    }
}

/// Bundle all utility systems for easy registration
pub struct QueryUtilitySystems;

impl QueryUtilitySystems {
    pub fn add_to_app(app: &mut App) -> &mut App {
        app.add_systems(Update, (
            optimized_iteration_patterns,
            analyze_archetype_fragmentation,
            query_performance_monitor,
        ))
    }
}
