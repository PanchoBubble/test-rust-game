use bevy::prelude::*;
use bevy_wasm_game::{
    components::*,
    resources::*,
    input::*,
    systems::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Physics Cube Game".to_string(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WorldBounds::default_bounds())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(FixedUpdate, (physics_integration, boundary_collision).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Spawn player cube with physics components
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Player,
        LinearVelocity::zero(),
        Acceleration::zero(),
        Friction::default(),
    ));
}
