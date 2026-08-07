use bevy::prelude::*;
use crate::player::PlayerPlugin;

mod player;
mod map;

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
        .add_plugins(PlayerPlugin)
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
