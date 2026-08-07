use bevy::{prelude::*, sprite::Anchor};
use bevy_procedural_tilemaps::prelude::*;
use crate::map::tilemap::TILEMAP;

#[derive(Clone)]
pub struct SpawnableAsset {
    // i.e. grass, tree, rock
    sprite_name: &'static str,
    // offset in grid coordinates
    grid_offset: GridDelta,
    // offset in world coordinates (px?)
    offset: Vec3,
    // Function to add custom components (like collision, physics, etc.)
    components_spawner: fn(&mut EntityCommands),
}

impl SpawnableAsset {
    pub fn new(sprite_name: &'static str) -> Self {
        Self {
            sprite_name,
            grid_offset: GridDelta::new(0, 0, 0),
            offset: Vec3::ZERO,
            components_spawner: |_| {}, // Default: no extra components
        }
    }

    pub fn with_grid_offset(mut self, offset: GridDelta) -> Self {
        self.grid_offset = offset;
        self
    }
}


#[derive(Clone)]
pub struct TilemapHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl TilemapHandles {
    pub fn sprite(&self, atlas_index: usize) -> Sprite {
        Sprite::from_atlas_image(
            self.image.clone(),
            TextureAtlas::from(self.layout.clone()).with_index(atlas_index),
        )
    }
}


pub fn prepare_tilemap_handles(
    asset_server: Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    assets_directory: &str,
    tilemap_file: &str,
) -> TilemapHandles {
    let image = asset_server.load::<Image>(format!("{assets_directory}/{tilemap_file}"));
    let mut layout = TextureAtlasLayout::new_empty(TILEMAP.atlas_size());
    for index in 0..TILEMAP.sprites.len() {
        layout.add_texture(TILEMAP.sprite_rect(index));
    }
    let layout = atlas_layouts.add(layout);

    TilemapHandles { image, layout }
}

pub fn load_assets(
    tilemap_handles: &TilemapHandles,
    asset_definitions: Vec<Vec<SpawnableAsset>>
) -> ModelsAssets<Sprite> {
    let mut model_assets = ModelAssets::<Sprite>::new();
    // for each type of tile...
    for (model_index, assets) in asset_definitions.into_iter().enumerate() {
        // for each sprite that tile needs...
        for asset_def in assets {
            let SpawnableAsset {
                sprite_name,
                grid_offset,
                offset,
                components_spawner,
            } = asset_def;

            let Some(atlas_index) = TILEMAP.sprite_index(sprite_name) else {
                panic!("Uknown atlas sprite '{}'", sprite_name);
            };

            model_assets.add(
                model_index,
                ModelAsset {
                    assets_bundle: tilemap_handles.sprite(atlas_index),
                    grid_offset,
                    world_offset: offset,
                    spawn_commands: components_spawner,
                },
            )
        }
    }
    model_assets
}
