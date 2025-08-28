use crate::components::Player;
use bevy::prelude::*;

/// Input handling system for WASD movement
pub fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&mut Player, &mut Sprite)>,
) {
    let mut input_force: f32 = 3000.0; // Base acceleration force from input

    for (mut player, mut sprite) in query.iter_mut() {
        let mut input_vector = Vec2::ZERO;

        sprite.color = Color::rgb(0.25, 0.25, 0.75);
        if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight)
        {
            input_force = input_force * 3.0;
            sprite.color = Color::rgb(0.9, 0.25, 0.75);
        }
        if mouse_input.pressed(MouseButton::Left)
        {
            input_force = input_force * 3.0;
            sprite.color = Color::rgb(0.9, 0.9, 0.75);
        }

        if keyboard_input.pressed(KeyCode::L) || keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            input_vector.x += 1.0;
        }

        // Check WASD keys and build input vector
        if keyboard_input.pressed(KeyCode::K) || keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            input_vector.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::J) || keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            input_vector.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::H) || keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            input_vector.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::L) || keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            input_vector.x += 1.0;
        }


        // Normalize diagonal movement to prevent faster diagonal speed
        if input_vector != Vec2::ZERO {
            input_vector = input_vector.normalize();
        }

        // Apply input force to acceleration
        player.acceleration = input_vector * input_force;
    }
}
