use bevy::prelude::*;

mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
                file_path: "src/assets".into(),
                ..default()
            }),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(Update, listen_for_quit)
        // .add_systems(Update, player_movement)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn listen_for_quit(
    input: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

// fn setup(mut commands: Commands) {
//     commands.spawn((
//         Player,
//         Text2d::new("@"),
//         TextFont {
//             font_size: FontSize::Px(12.0),
//             font: default(),
//             ..default()
//         },
//         TextColor(Color::BLACK),
//         Transform::from_translation(Vec3::ZERO),
//     ));
// }


// fn player_movement(
//     input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//     mut player_transform: Single<&mut Transform, With<Player>>,
// ) {
//     let mut direction = Vec2::ZERO;
//     if input.pressed(KeyCode::KeyA) {
//         direction.x -= 1.0;
//     }
//     if input.pressed(KeyCode::KeyD) {
//         direction.x += 1.0;
//     }
//     if input.pressed(KeyCode::KeyW) {
//         direction.y += 1.0;
//     }
//     if input.pressed(KeyCode::KeyS) {
//         direction.y -= 1.0;
//     }

//     if direction != Vec2::ZERO {
//         let speed = 300.0; // pixels per second
//         let delta = direction.normalize() * speed * time.delta_secs();
//         player_transform.translation.x += delta.x;
//         player_transform.translation.y += delta.y;
//     }
// }
