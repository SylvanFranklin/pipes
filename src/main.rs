mod helpers;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::Neighbors;
use bevy_ecs_tilemap::prelude::*;
use helpers::machine::{Pipe, PipeKind};
use rand::Rng;

#[derive(Component)]
struct LastUpdate(f64);

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let texture_handle: Handle<Image> = asset_server.load("new_pipes.png");
    let map_size = TilemapSize { x: 20, y: 20 };
    let map_type = TilemapType::Square;
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(0),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 128.0, y: 128.0 };
    let grid_size = tile_size.into();
    let start_pos = TilePos { x: 10, y: 10 };

    commands
        .entity(tile_storage.get(&start_pos).unwrap())
        .insert(Pipe::new(PipeKind::Cross).spread_to_tile());

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            size: map_size,
            storage: tile_storage,
            map_type,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        },
        LastUpdate(0.0),
    ));
}

fn update_map(
    time: Res<Time>,
    mut commands: Commands,
    mut tilemap_query: Query<(&mut LastUpdate, &TileStorage, &TilemapSize)>,
    pipes_query: Query<(&TilePos, &Pipe)>,
    tiles_query: Query<&TilePos, Without<Pipe>>,
) {
    let (mut last_update, tile_storage, map_size) = tilemap_query.single_mut();
    let current_time = time.elapsed_seconds_f64();

    if current_time - last_update.0 > 0.8 {
        for (pos, pipe) in pipes_query.iter() {
            let neighbors = Neighbors::get_square_neighboring_positions(pos, map_size, false)
                .entities(tile_storage);

            pipe.connections.to_vec().iter().for_each(|dir| {
                if let Some(neighbor) = neighbors.get(*dir) {
                    if tiles_query.get(*neighbor).is_ok() {
                        let kind: PipeKind = match rand::thread_rng().gen_range(0..100) {
                            0..=50 => PipeKind::Straight,
                            51..=75 => PipeKind::Elbow,
                            76..=90 => PipeKind::T,
                            _ => PipeKind::Cross,
                        };

                        commands
                            .entity(*neighbor)
                            .insert(Pipe::new().spread_to_tile());
                    }
                }
            });
        }

        last_update.0 = current_time;
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1600.0, 800.0),
                        decorations: false,
                        resizable: false,
                        // prevent_default_event_handling: false,
                        title: String::from(""),
                        // mode: WindowMode::Fullscreen,
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, helpers::camera::movement)
        .add_systems(Update, update_map)
        .run();
}
