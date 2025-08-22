use crate::components::*;
use bevy::ecs::component::ComponentId;
use bevy::prelude::*;
use std::any::TypeId;
use std::collections::HashMap;

/// Dynamic Query Construction Examples
/// These demonstrate runtime query building for flexible entity inspection
/// Note: Dynamic queries have performance overhead and reduced type safety

/// Example 1: Dynamic Query Building Concept
/// Note: Bevy 0.11 has limited QueryBuilder support, showing conceptual approach
pub fn dynamic_query_example(_world: &World) {
    println!("=== Dynamic Query Construction (Conceptual) ===");

    // In Bevy 0.11, QueryBuilder is not available in the same way
    // This shows the conceptual approach that would be used

    // Build a query dynamically based on runtime conditions
    let include_transform = true;
    let include_velocity = true;

    println!("Would build query with:");
    if include_transform {
        println!("  - Transform component");
    }

    if include_velocity {
        println!("  - LinearVelocity component");
    }

    // Note: In practice, building and executing dynamic queries
    // requires careful handling of the type system
    println!("Dynamic query concept demonstrated");
}

/// Example 2: Component Type Registry
/// Maintains a registry of component types for dynamic operations

#[derive(Resource, Default)]
pub struct ComponentRegistry {
    type_names: HashMap<ComponentId, String>,
    type_ids: HashMap<TypeId, ComponentId>,
}

impl ComponentRegistry {
    pub fn register_component<T: Component>(&mut self, world: &World, name: &str) {
        let component_id = world.components().component_id::<T>();
        if let Some(id) = component_id {
            self.type_names.insert(id, name.to_string());
            self.type_ids.insert(TypeId::of::<T>(), id);
        }
    }

    pub fn get_component_name(&self, id: ComponentId) -> Option<&String> {
        self.type_names.get(&id)
    }

    pub fn get_component_id<T: 'static>(&self) -> Option<ComponentId> {
        self.type_ids.get(&TypeId::of::<T>()).copied()
    }
}

/// Initialize component registry with game components
pub fn setup_component_registry(world: &mut World) {
    let mut registry = ComponentRegistry::default();

    registry.register_component::<Transform>(world, "Transform");
    registry.register_component::<Player>(world, "Player");

    world.insert_resource(registry);
}

/// Example 3: Dynamic Entity Inspection
/// Inspect any entity's components at runtime

pub struct EntityInspector {
    registry: ComponentRegistry,
}

impl EntityInspector {
    pub fn new(registry: ComponentRegistry) -> Self {
        Self { registry }
    }

    pub fn inspect_entity(&self, world: &World, entity: Entity) {
        println!("=== Inspecting Entity {:?} ===", entity);

        if let Some(entity_ref) = world.get_entity(entity) {
            let archetype = entity_ref.archetype();

            println!("Archetype ID: {:?}", archetype.id());
            println!("Component count: {}", archetype.components().count());

            // List all components on this entity
            for component_id in archetype.components() {
                if let Some(name) = self.registry.get_component_name(component_id) {
                    println!("  - {}", name);
                } else {
                    println!("  - Unknown component {:?}", component_id);
                }
            }
        } else {
            println!("Entity {:?} not found", entity);
        }
    }
}

/// Example 4: Conditional Query System
/// Build queries based on component availability

pub fn conditional_query_system(_world: &World, registry: Res<ComponentRegistry>) {
    println!("=== Conditional Query System ===");

    // Check what components are available
    let has_player = registry.get_component_id::<Player>().is_some();
    let has_transform = registry.get_component_id::<Transform>().is_some();

    if has_player && has_transform {
        println!("Full physics components available - using complex physics query");
        // In practice, you'd build and execute the appropriate query
    } else if has_transform {
        println!("Only transform available - using simple movement query");
    } else {
        println!("No physics components - using entity-only query");
    }
}

/// Example 5: Scripting Interface Helper
/// Provides a script-friendly interface for entity queries

pub struct ScriptQueryInterface {
    world: *const World,
}

impl ScriptQueryInterface {
    pub fn new(world: &World) -> Self {
        Self {
            world: world as *const World,
        }
    }

    /// Find entities with specific components (by name)
    pub fn find_entities_with_components(&self, component_names: &[&str]) -> Vec<Entity> {
        // This would implement actual component name resolution and querying
        // For now, return empty vec
        println!(
            "Searching for entities with components: {:?}",
            component_names
        );
        Vec::new()
    }

    /// Get component data as string (for debugging/scripting)
    pub fn get_component_data_string(
        &self,
        entity: Entity,
        component_name: &str,
    ) -> Option<String> {
        println!("Getting {} component data for {:?}", component_name, entity);
        // This would implement actual component data serialization
        Some(format!("{}(data)", component_name))
    }
}

/// Example 6: Query Performance Profiler
/// Dynamically profiles different query patterns

#[derive(Resource, Default)]
pub struct QueryProfiler {
    query_times: HashMap<String, Vec<std::time::Duration>>,
}

impl QueryProfiler {
    pub fn profile_query<F>(&mut self, query_name: &str, query_fn: F) -> std::time::Duration
    where
        F: FnOnce(),
    {
        let start = std::time::Instant::now();
        query_fn();
        let duration = start.elapsed();

        self.query_times
            .entry(query_name.to_string())
            .or_default()
            .push(duration);

        duration
    }

    pub fn get_average_time(&self, query_name: &str) -> Option<std::time::Duration> {
        self.query_times.get(query_name).map(|times| {
            let total: std::time::Duration = times.iter().sum();
            total / times.len() as u32
        })
    }

    pub fn print_profile_report(&self) {
        println!("=== Query Performance Profile ===");
        for (query_name, times) in &self.query_times {
            if let Some(avg) = self.get_average_time(query_name) {
                let min = times.iter().min().unwrap();
                let max = times.iter().max().unwrap();
                println!(
                    "{}: avg={:?}, min={:?}, max={:?}, samples={}",
                    query_name,
                    avg,
                    min,
                    max,
                    times.len()
                );
            }
        }
    }
}

/// Example 7: Dynamic Filter System
/// Apply filters based on runtime configuration

pub struct DynamicFilter {
    include_components: Vec<String>,
    exclude_components: Vec<String>,
}

impl DynamicFilter {
    pub fn new() -> Self {
        Self {
            include_components: Vec::new(),
            exclude_components: Vec::new(),
        }
    }

    pub fn with_component(mut self, component_name: &str) -> Self {
        self.include_components.push(component_name.to_string());
        self
    }

    pub fn without_component(mut self, component_name: &str) -> Self {
        self.exclude_components.push(component_name.to_string());
        self
    }

    pub fn describe(&self) -> String {
        format!(
            "Filter: with={:?}, without={:?}",
            self.include_components, self.exclude_components
        )
    }
}

/// Example 8: Archetype Explorer
/// Dynamically explore and analyze archetypes in the world

pub fn explore_archetypes(world: &World, registry: Res<ComponentRegistry>) {
    println!("=== Archetype Explorer ===");

    let archetypes = world.archetypes();
    println!("Total archetypes: {}", archetypes.len());

    for archetype in archetypes.iter() {
        println!("\nArchetype {:?}:", archetype.id());
        println!("  Entity count: {}", archetype.len());
        println!("  Components:");

        for component_id in archetype.components() {
            if let Some(name) = registry.get_component_name(component_id) {
                println!("    - {}", name);
            } else {
                println!("    - Unknown {:?}", component_id);
            }
        }
    }
}

/// Example 9: Entity Search System
/// Search entities by component patterns with fuzzy matching

pub struct EntitySearcher {
    // In a real implementation, this would contain search indices
}

impl EntitySearcher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn search_by_components(&self, patterns: &[&str]) -> Vec<Entity> {
        println!("Searching entities with component patterns: {:?}", patterns);
        // This would implement actual pattern matching and entity lookup
        Vec::new()
    }

    pub fn search_by_archetype_similarity(&self, reference_entity: Entity) -> Vec<Entity> {
        println!("Finding entities similar to {:?}", reference_entity);
        // This would find entities with similar component combinations
        Vec::new()
    }
}

/// Bundle all dynamic query systems for easy registration
pub struct DynamicQuerySystems;

impl DynamicQuerySystems {
    pub fn add_to_app(app: &mut App) -> &mut App {
        app.init_resource::<ComponentRegistry>()
            .init_resource::<QueryProfiler>()
            .add_systems(Startup, setup_component_registry)
            .add_systems(Update, (conditional_query_system, explore_archetypes))
    }
}

/// Helper trait for components to support dynamic operations
pub trait DynamicComponent {
    fn type_name() -> &'static str;
    fn as_debug_string(&self) -> String;
}

// Implement for our game components
impl DynamicComponent for Player {
    fn type_name() -> &'static str {
        "Player"
    }
    fn as_debug_string(&self) -> String {
        format!(
            "Player(vel: {:?}, accel: {:?}, friction: {})",
            self.velocity, self.acceleration, self.friction
        )
    }
}
