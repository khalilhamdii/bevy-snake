use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
#[derive(Component)]
struct SnakeHead;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 2.;
        }
    }
}

fn main() {
    App::new()
        .add_systems(Startup, setup_camera) // <--
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, snake_movement) // <--
        .add_plugins(DefaultPlugins)
        .run();
}
