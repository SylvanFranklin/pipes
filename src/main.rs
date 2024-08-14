mod helpers;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use bevy_ecs_tilemap::helpers::square_grid::neighbors::{
    Neighbors, SquareDirection, CARDINAL_SQUARE_DIRECTIONS,
};
use bevy_ecs_tilemap::prelude::*;
use helpers::machine::{
    get_connections, get_texture_index, Connections, Pipe, PipeKind, ALL_PIPES,
};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Component)]
struct LastUpdate(f64);

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let texture_handle: Handle<Image> = asset_server.load("pipes.png");
    let map_size = TilemapSize { x: 32, y: 32 };
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

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let start_pos = TilePos { x: 16, y: 16 };
    commands
        .entity(tile_storage.get(&start_pos).unwrap())
        .insert((
            Pipe::new(PipeKind::Cross),
            get_texture_index(PipeKind::Cross),
        ));

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
    pipes_query: Query<(&TilePos, &Pipe, &TileTextureIndex)>,
) {
    let (mut last_update, tile_storage, map_size) = tilemap_query.single_mut();
    let current_time = time.elapsed_seconds_f64();

    if current_time - last_update.0 > 0.5 {
        for (pos, pipe, texture_index) in pipes_query.iter() {
            let neighbors = Neighbors::get_square_neighboring_positions(pos, map_size, false)
                .entities(tile_storage);

            neighbors.iter().for_each(|neighbor_entity| {
                // maybe make the conversion back to a vector or dirs
            });

            pipe.connections.iter().for_each(|dir| {
                if let Some(neighbor) = neighbors.get(*dir) {
                    let nconn = neighbors_query.get(*neighbor).unwrap();

                    let kind: PipeKind = match rand::thread_rng().gen_range(0..100) {
                        0..=50 => PipeKind::Straight,
                        51..=75 => PipeKind::Elbow,
                        76..=90 => PipeKind::T,
                        _ => PipeKind::Cross,
                    };

                    commands.entity(*neighbor).insert((
                        Pipe(kind),
                        get_texture_index(kind),
                        // TileFlip {
                        //     d: true,
                        //     ..default()
                        // },
                        Connections(
                            get_connections(&kind)
                                .iter()
                                .filter(|d| *d != dir)
                                .copied()
                                .collect(),
                        ),
                    ));
                }
            });

            // commands
            //     .entity(tile_storage.get(pos).unwrap())
            //     .remove::<OpenEnd>();
        }

        // commands
        //     .entity(tile_storage.get(pos).unwrap())
        //     .remove::<OpenEnd>();

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
