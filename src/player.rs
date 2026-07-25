use bevy::prelude::*;

const TILE_SIZE: u32 = 64; // 64 x 64 tiles
const NUM_WALK_FRAMES: usize = 9;
const MOVE_SPEED: f32 = 140.0; // Pixels per second
const ANIMATION_DELTA_TIME: f32 = 0.1; // Seconds per frame (~10 fps)

#[derive(Component)]
struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationState {
    facing: Facing,
    moving: bool,
    was_moving: bool,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the spritesheet and build a grid layout (64x64-size tiles, 9 columns, 12 rows)
    let texture = asset_server.load("male_spritesheet.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        NUM_WALK_FRAMES as u32,
        12,
        None,
        None,
    ));
    let facing = Facing::Down;
    let start_index = atlas_index_for(facing, 0);

    commands.spawn((
        Player,
        Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout,
                    index: start_index,
                },
        ),
        Transform::from_translation(Vec3::ZERO),
        AnimationState { facing, moving: false, was_moving: false },
        AnimationTimer(Timer::from_seconds(ANIMATION_DELTA_TIME, TimerMode::Repeating)),
    ));
}
