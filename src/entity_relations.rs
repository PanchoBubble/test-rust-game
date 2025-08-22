use bevy::prelude::*;
use crate::components::*;
use std::collections::HashMap;

/// Entity relationship components for creating hierarchies and references
/// These enable complex entity relationships beyond simple component composition

/// Parent-child relationship components
#[derive(Component, Debug)]
pub struct Parent(pub Entity);

#[derive(Component, Debug, Default)]
pub struct Children(pub Vec<Entity>);

/// Reverse lookup for child-to-parent relationships
#[derive(Component, Debug)]
pub struct ChildOf(pub Entity);

/// Target reference component - points to another entity
#[derive(Component, Debug)]
pub struct Target(pub Entity);

/// Owner component - tracks who owns this entity
#[derive(Component, Debug)]
pub struct Owner(pub Entity);

/// Group membership component - entities can belong to groups
#[derive(Component, Debug)]
pub struct GroupMember {
    pub group_id: u32,
    pub role: String,
}

/// Group component - represents a collection of entities
#[derive(Component, Debug)]
pub struct Group {
    pub id: u32,
    pub name: String,
    pub member_count: usize,
}

/// Entity Relationship Query Examples
/// These demonstrate how to work with entity references and hierarchies

/// Example 1: Parent-Child Hierarchy Queries
pub fn query_parent_child_relationships(
    parents: Query<(Entity, &Children), With<Parent>>,
    children: Query<(Entity, &ChildOf)>,
) {
    println!("=== Parent-Child Relationships ===");
    
    // Find all parents and their children
    for (parent_entity, children_component) in parents.iter() {
        println!("Parent {:?} has {} children:", parent_entity, children_component.0.len());
        for &child_entity in &children_component.0 {
            println!("  - Child: {:?}", child_entity);
        }
    }
    
    // Find all children and their parents
    println!("\nChild-to-Parent lookup:");
    for (child_entity, child_of) in children.iter() {
        println!("Child {:?} belongs to parent {:?}", child_entity, child_of.0);
    }
}

/// Example 2: Two-Phase Queries for Entity References
pub fn query_entity_references_two_phase(
    // Phase 1: Get entities with target references
    targeting_entities: Query<(Entity, &Target, &Transform)>,
    // Phase 2: Get the actual targets
    all_transforms: Query<&Transform>,
) {
    println!("=== Entity Reference Queries (Two-Phase) ===");
    
    for (entity, target, transform) in targeting_entities.iter() {
        // First phase: we have the targeting entity
        println!("Entity {:?} at {:?} targets {:?}", 
                entity, transform.translation, target.0);
        
        // Second phase: lookup the target entity
        if let Ok(target_transform) = all_transforms.get(target.0) {
            let distance = transform.translation.distance(target_transform.translation);
            println!("  Target is at {:?}, distance: {:.2}", 
                    target_transform.translation, distance);
        } else {
            println!("  Warning: Target {:?} not found or has no Transform!", target.0);
        }
    }
}

/// Example 3: Owner-Owned Relationships
pub fn query_ownership_relationships(
    owners: Query<(Entity, &Transform), (With<Player>, Without<Owner>)>,
    owned_items: Query<(Entity, &Owner, &Transform)>,
) {
    println!("=== Ownership Relationships ===");
    
    // Build ownership map for efficient lookup
    let mut ownership_map: HashMap<Entity, Vec<(Entity, Vec3)>> = HashMap::new();
    for (item_entity, owner, transform) in owned_items.iter() {
        ownership_map.entry(owner.0).or_default().push((item_entity, transform.translation));
    }
    
    // Display ownership relationships
    for (owner_entity, owner_transform) in owners.iter() {
        if let Some(owned_items_data) = ownership_map.get(&owner_entity) {
            println!("Player {:?} at {:?} owns {} items:", 
                    owner_entity, owner_transform.translation, owned_items_data.len());
            
            for (item_entity, item_position) in owned_items_data {
                println!("  - Item {:?} at {:?}", item_entity, item_position);
            }
        }
    }
}

/// Example 4: Group Membership Queries
pub fn query_group_memberships(
    groups: Query<(Entity, &Group)>,
    members: Query<(Entity, &GroupMember, &Transform)>,
) {
    println!("=== Group Memberships ===");
    
    // Build groups map
    let mut groups_map: HashMap<u32, (Entity, &Group)> = HashMap::new();
    for (entity, group) in groups.iter() {
        groups_map.insert(group.id, (entity, group));
    }
    
    // Display members by group
    let mut group_members: HashMap<u32, Vec<(Entity, &GroupMember, &Transform)>> = HashMap::new();
    for member in members.iter() {
        group_members.entry(member.1.group_id).or_default().push(member);
    }
    
    for (&group_id, members_list) in group_members.iter() {
        if let Some((group_entity, group)) = groups_map.get(&group_id) {
            println!("Group '{}' ({:?}) has {} members:", 
                    group.name, group_entity, members_list.len());
            
            for (member_entity, member, transform) in members_list {
                println!("  - {} {:?} at {:?}", 
                        member.role, member_entity, transform.translation);
            }
        }
    }
}

/// Example 5: Hierarchical Traversal - Find All Descendants
pub fn find_all_descendants(
    root_entity: Entity,
    children_query: &Query<&Children>,
    descendants: &mut Vec<Entity>,
) {
    if let Ok(children) = children_query.get(root_entity) {
        for &child in &children.0 {
            descendants.push(child);
            // Recursively find descendants of this child
            find_all_descendants(child, children_query, descendants);
        }
    }
}

/// Example 6: Validation System for Entity References
pub fn validate_entity_references(
    _commands: Commands,
    // Check all entities with target references
    targets: Query<(Entity, &Target)>,
    // Check all entities with parent references
    parents: Query<(Entity, &Parent)>,
    // Check all entities with owner references
    owners: Query<(Entity, &Owner)>,
    // Get all valid entities
    all_entities: Query<Entity>,
) {
    println!("=== Entity Reference Validation ===");
    
    let valid_entities: std::collections::HashSet<Entity> = all_entities.iter().collect();
    let mut invalid_references = Vec::new();
    
    // Validate target references
    for (entity, target) in targets.iter() {
        if !valid_entities.contains(&target.0) {
            println!("Invalid target reference: {:?} -> {:?}", entity, target.0);
            invalid_references.push(entity);
        }
    }
    
    // Validate parent references
    for (entity, parent) in parents.iter() {
        if !valid_entities.contains(&parent.0) {
            println!("Invalid parent reference: {:?} -> {:?}", entity, parent.0);
            invalid_references.push(entity);
        }
    }
    
    // Validate owner references
    for (entity, owner) in owners.iter() {
        if !valid_entities.contains(&owner.0) {
            println!("Invalid owner reference: {:?} -> {:?}", entity, owner.0);
            invalid_references.push(entity);
        }
    }
    
    // Clean up invalid references (in a real system, you might want more sophisticated handling)
    for entity in invalid_references {
        println!("Cleaning up entity with invalid references: {:?}", entity);
        // commands.entity(entity).despawn(); // Uncomment if you want to despawn
    }
    
    println!("Reference validation complete.");
}

/// Utility Systems for Managing Entity Relationships

/// Create parent-child relationship
pub fn create_parent_child_relationship(
    mut commands: Commands,
    parent: Entity,
    child: Entity,
) {
    // Add child to parent's children list
    // In Bevy 0.11, we use a simpler approach
    commands.entity(parent).insert(Children(vec![child]));
    
    // Set parent reference on child
    commands.entity(child).insert(ChildOf(parent));
}

/// Remove parent-child relationship
pub fn remove_parent_child_relationship(
    mut commands: Commands,
    parent: Entity,
    child: Entity,
    mut parents: Query<&mut Children>,
) {
    // Remove child from parent's children list
    if let Ok(mut children) = parents.get_mut(parent) {
        children.0.retain(|&c| c != child);
    }
    
    // Remove parent reference from child
    commands.entity(child).remove::<ChildOf>();
}

/// Example 7: Complex Relationship Query - Find Related Entities
pub fn query_related_entities(
    // Find entities that are both children and have physics
    child_physics: Query<(Entity, &ChildOf, &LinearVelocity), With<Player>>,
    // Find their parents
    parents: Query<&Transform, (With<Parent>, Without<Player>)>,
) {
    println!("=== Complex Relationship Query ===");
    
    for (child_entity, child_of, velocity) in child_physics.iter() {
        if let Ok(parent_transform) = parents.get(child_of.0) {
            println!("Player child {:?} (vel: {:?}) has parent at {:?}", 
                    child_entity, velocity.0, parent_transform.translation);
        }
    }
}

/// Bundle all entity relationship systems for easy registration
pub struct EntityRelationSystems;

impl EntityRelationSystems {
    pub fn add_to_app(app: &mut App) -> &mut App {
        app.add_systems(Update, (
            query_parent_child_relationships,
            query_entity_references_two_phase,
            query_ownership_relationships,
            query_group_memberships,
            validate_entity_references,
            query_related_entities,
        ))
    }
}
