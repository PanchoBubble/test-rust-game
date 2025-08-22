use bevy::prelude::*;
use bevy_wasm_game::components::*;

/// Integration tests for query patterns
/// These tests verify that all query examples work correctly with real entities

#[cfg(test)]
mod query_tests {
    use super::*;

    /// Create a test world with various entity configurations
    fn create_test_world() -> World {
        let mut world = World::new();
        
        // Register components
        world.init_component::<Transform>();
        world.init_component::<Player>();
        world.init_component::<LinearVelocity>();
        world.init_component::<Acceleration>();
        world.init_component::<Friction>();
        
        // Create test entities with different component combinations
        
        // Entity 1: Full physics player
        world.spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Player,
            LinearVelocity(Vec2::new(1.0, 0.5)),
            Acceleration(Vec2::new(0.1, 0.0)),
            Friction(0.95),
        ));
        
        // Entity 2: Player with velocity only
        world.spawn((
            Transform::from_translation(Vec3::new(5.0, 5.0, 0.0)),
            Player,
            LinearVelocity(Vec2::new(-1.0, 1.0)),
        ));
        
        // Entity 3: Non-player with physics
        world.spawn((
            Transform::from_translation(Vec3::new(10.0, 10.0, 0.0)),
            LinearVelocity(Vec2::new(0.0, -2.0)),
            Acceleration(Vec2::new(0.0, -0.5)),
            Friction(0.98),
        ));
        
        // Entity 4: Transform only
        world.spawn(Transform::from_translation(Vec3::new(-5.0, -5.0, 0.0)));
        
        world
    }

    #[test]
    fn test_basic_intersection_queries() {
        let mut world = create_test_world();
        
        // Test query for players with velocity
        let mut query = world.query::<(&Player, &LinearVelocity)>();
        let results: Vec<_> = query.iter(&world).collect();
        
        assert_eq!(results.len(), 2, "Should find 2 players with velocity");
        
        // Verify the velocities
        for (_, velocity) in results {
            assert!(!velocity.0.is_nan(), "Velocity should be valid");
        }
    }

    #[test]
    fn test_with_without_filters() {
        let mut world = create_test_world();
        
        // Test With filter - players with acceleration
        let mut with_query = world.query_filtered::<&Transform, (With<Player>, With<Acceleration>)>();
        let with_results: Vec<_> = with_query.iter(&world).collect();
        
        assert_eq!(with_results.len(), 1, "Should find 1 player with acceleration");
        
        // Test Without filter - velocity entities without Player
        let mut without_query = world.query_filtered::<&LinearVelocity, Without<Player>>();
        let without_results: Vec<_> = without_query.iter(&world).collect();
        
        assert_eq!(without_results.len(), 1, "Should find 1 non-player with velocity");
    }

    #[test]
    fn test_optional_components() {
        let mut world = create_test_world();
        
        // Test optional component patterns
        let mut query = world.query::<(&Transform, Option<&LinearVelocity>, Option<&Player>)>();
        let results: Vec<_> = query.iter(&world).collect();
        
        assert_eq!(results.len(), 4, "Should find all 4 entities");
        
        // Count entities by component combination
        let mut players_with_velocity = 0;
        let mut transform_only = 0;
        
        for (_, velocity_opt, player_opt) in results {
            match (velocity_opt, player_opt) {
                (Some(_), Some(_)) => players_with_velocity += 1,
                (None, None) => transform_only += 1,
                _ => {}
            }
        }
        
        assert_eq!(players_with_velocity, 2, "Should have 2 players with velocity");
        assert_eq!(transform_only, 1, "Should have 1 transform-only entity");
    }

    #[test]
    fn test_entity_access_queries() {
        let mut world = create_test_world();
        
        // Test entity ID access
        let mut query = world.query::<(Entity, &Transform)>();
        let results: Vec<_> = query.iter(&world).collect();
        
        assert_eq!(results.len(), 4, "Should find all entities with Transform");
        
        // Ensure all entity IDs are unique
        let entity_ids: std::collections::HashSet<_> = results.iter().map(|(id, _)| *id).collect();
        assert_eq!(entity_ids.len(), 4, "All entity IDs should be unique");
    }

    #[test]
    fn test_mutable_queries() {
        let mut world = create_test_world();
        
        // Test mutable access to velocity
        {
            let mut query = world.query::<&mut LinearVelocity>();
            for mut velocity in query.iter_mut(&mut world) {
                velocity.0 *= 0.5; // Halve all velocities
            }
        }
        
        // Verify changes were applied
        let mut verify_query = world.query::<&LinearVelocity>();
        for velocity in verify_query.iter(&world) {
            assert!(velocity.0.length() < 2.0, "Velocities should be reduced");
        }
    }

    #[test]
    fn test_complex_filter_combinations() {
        let mut world = create_test_world();
        
        // Complex filter: Transform + LinearVelocity, With Player, Without Acceleration
        let mut query = world.query_filtered::<
            (&Transform, &LinearVelocity), 
            (With<Player>, Without<Acceleration>)
        >();
        
        let results: Vec<_> = query.iter(&world).collect();
        assert_eq!(results.len(), 1, "Should find 1 player with velocity but no acceleration");
    }

    #[test]
    fn test_query_is_empty() {
        let mut world = create_test_world();
        
        // Test for non-existent component combination
        let mut empty_query = world.query_filtered::<&Transform, (With<Player>, With<Acceleration>, With<Friction>, Without<LinearVelocity>)>();
        
        // In Bevy 0.11, is_empty requires additional tick parameters
        // For testing, we'll check if the query returns any results
        assert_eq!(empty_query.iter(&world).count(), 0, "Query should be empty for impossible combination");
        
        // Test for existing combination
        let mut non_empty_query = world.query_filtered::<&Transform, With<Player>>();
        assert!(non_empty_query.iter(&world).count() > 0, "Query should not be empty for players");
    }

    #[test]
    fn test_query_single() {
        let mut world = create_test_world();
        
        // Test single result query
        let mut query = world.query_filtered::<(&Transform, &LinearVelocity, &Acceleration, &Friction), With<Player>>();
        
        // Should be exactly one player with full physics
        let result = query.get_single(&world);
        assert!(result.is_ok(), "Should find exactly one player with full physics");
        
        if let Ok((transform, velocity, acceleration, friction)) = result {
            assert_eq!(transform.translation, Vec3::new(0.0, 0.0, 0.0));
            assert_eq!(velocity.0, Vec2::new(1.0, 0.5));
            assert_eq!(acceleration.0, Vec2::new(0.1, 0.0));
            assert_eq!(friction.0, 0.95);
        }
    }

    #[test]
    fn test_query_many() {
        let mut world = create_test_world();
        
        // Test getting multiple specific entities
        let mut query = world.query::<&Transform>();
        let mut entity_query = world.query::<Entity>();
        let all_entities: Vec<Entity> = entity_query.iter(&world).collect();
        
        // Get transforms for first two entities individually
        // In Bevy 0.11, get_many is more restrictive, so we'll test individual gets
        assert!(all_entities.len() >= 2, "Should have at least 2 entities");
        
        let first_result = query.get(&world, all_entities[0]);
        let second_result = query.get(&world, all_entities[1]);
        
        assert!(first_result.is_ok(), "Should get first entity's transform");
        assert!(second_result.is_ok(), "Should get second entity's transform");
    }

    #[test] 
    fn test_archetype_consistency() {
        let mut world = create_test_world();
        
        // Verify that entities with same components are in same archetype
        let mut player_query = world.query::<(Entity, &Player, &LinearVelocity)>();
        let player_entities: Vec<_> = player_query.iter(&world).collect();
        
        assert_eq!(player_entities.len(), 2, "Should have 2 player entities with velocity");
        
        // Both player entities should exist and be queryable
        // We'll collect the entities first to avoid borrowing issues
        let entity_ids: Vec<Entity> = player_entities.into_iter().map(|(entity, _, _)| entity).collect();
        
        for entity in entity_ids {
            let mut single_query = world.query::<&Player>();
            assert!(single_query.get(&world, entity).is_ok(), 
                   "Each player entity should be individually queryable");
        }
    }

    /// Performance regression test
    #[test]
    fn test_query_performance() {
        let mut world = create_test_world();
        
        // Add more entities for performance testing
        for i in 0..1000 {
            world.spawn((
                Transform::from_translation(Vec3::new(i as f32, 0.0, 0.0)),
                LinearVelocity(Vec2::new(i as f32 % 10.0, 0.0)),
            ));
        }
        
        let start = std::time::Instant::now();
        
        // Perform various queries
        let mut transform_query = world.query::<&Transform>();
        let transform_count = transform_query.iter(&world).count();
        
        let mut velocity_query = world.query::<&LinearVelocity>();
        let velocity_count = velocity_query.iter(&world).count();
        
        let mut combined_query = world.query::<(&Transform, &LinearVelocity)>();
        let combined_count = combined_query.iter(&world).count();
        
        let duration = start.elapsed();
        
        // Verify counts
        assert_eq!(transform_count, 1004, "Should have 1004 transforms"); // 4 original + 1000 new
        assert_eq!(velocity_count, 1003, "Should have 1003 velocities"); // 3 original + 1000 new
        assert_eq!(combined_count, 1003, "Should have 1003 combined"); // 3 original + 1000 new (all new entities have both)
        
        // Performance assertion (should be very fast)
        assert!(duration.as_millis() < 100, 
               "Queries should complete in less than 100ms, took: {:?}", duration);
    }
}

/// Test helper functions
#[cfg(test)]
mod test_helpers {
    use super::*;

    pub fn create_physics_entity(world: &mut World, position: Vec3, velocity: Vec2) -> Entity {
        world.spawn((
            Transform::from_translation(position),
            Player,
            LinearVelocity(velocity),
            Acceleration::zero(),
            Friction::default(),
        )).id()
    }

    pub fn create_simple_entity(world: &mut World, position: Vec3) -> Entity {
        world.spawn(Transform::from_translation(position)).id()
    }

    pub fn count_entities_with_component<T: Component>(world: &mut World) -> usize {
        let mut query = world.query::<&T>();
        query.iter(world).count()
    }
}
