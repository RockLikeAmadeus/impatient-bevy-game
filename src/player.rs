use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, player_animation));
    }
}

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

fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        anim.moving = true;

        // Update facing based on dominant direction
        if direction.x.abs() > direction.y.abs() {
            anim.facing = if direction.x > 0.0 { Facing::Right } else { Facing::Left };
        } else {
            anim.facing = if direction.y > 0.0 { Facing::Up } else { Facing::Down };
        }
    } else {
        anim.moving = false;
    }
}

fn player_animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    let Ok((mut anim, mut timer, mut sprite)) = query.single_mut() else {
        return;
    };

    let atlas = match sprite.texture_atlas.as_mut() {
        Some(a) => a,
        None => return,
    };

    // Compute the target row and current position in the atlas (column/row within the 9-column row)
    let target_row = row_zero_based(anim.facing);
    let mut current_col = atlas.index % NUM_WALK_FRAMES;
    let current_row = atlas.index / NUM_WALK_FRAMES;

    // If the facing changed (or we weren't on a walking row), snap to the first frame of the target row
    if current_row != target_row {
        atlas.index = row_start_index(anim.facing);
        current_col = 0;
        // current_row = target_row;
        timer.reset();
    }

    let just_started = anim.moving && !anim.was_moving;
    let just_stopped = !anim.moving && anim.was_moving;

    if anim.moving {
        if just_started {
            // On tap or movement start, immediately advance one frame for visible feedback
            let row_start = row_start_index(anim.facing);
            let next_col = (current_col + 1) % NUM_WALK_FRAMES;
            atlas.index = row_start + next_col;
            // Restart the timer so the next advance uses a full interval
            timer.reset();
        } else {
            // Continuous movement, advanced based on timer cadence
            timer.tick(time.delta());
            if timer.just_finished() {
                let row_start = row_start_index(anim.facing);
                let next_col = (current_col + 1) & NUM_WALK_FRAMES;
                atlas.index = row_start + next_col;
            }
        }
    } else if just_stopped {
        // Not moving: keep current frame to avoid snap. Reset timer on transition to idle.
        timer.reset();
    }

    // Update previous movement state
    anim.was_moving = anim.moving;
}

// Returns the starting atlas index for the given facing row
fn row_start_index(facing: Facing) -> usize {
    row_zero_based(facing) * NUM_WALK_FRAMES
}

fn atlas_index_for(facing: Facing, frame_in_row: usize) -> usize {
    row_start_index(facing) + frame_in_row.min(NUM_WALK_FRAMES - 1)
}

fn row_zero_based(facing: Facing) -> usize {
    match facing {
        Facing::Up => 8,
        Facing::Left => 9,
        Facing::Down => 10,
        Facing::Right => 11,
    }
}
