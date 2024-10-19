mod helpers;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::{helpers::square_grid::neighbors::Neighbors, prelude::*};
use helpers::{
    background::setup_background,
    machine::{Pipe, PipeKind},
};

pub const MAP_SIZE: TilemapSize = TilemapSize { x: 60, y: 60 };
pub const PIXEL_CELL_SIZE: TilemapTileSize = TilemapTileSize { x: 128.0, y: 128.0 };
pub const START_POS: TilePos = TilePos {
    x: MAP_SIZE.x / 2,
    y: MAP_SIZE.y / 2,
};

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // HDR is required for the bloom effect
                ..default()
            },

            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scale: 3.,
                ..default()
            },

            ..default()
        },
        BloomSettings::NATURAL,
    ));

    let texture_handle: Handle<Image> = asset_server.load("pipes.png");
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands.spawn_empty().id();

    bevy_ecs_tilemap::helpers::filling::fill_tilemap(
        TileTextureIndex(0),
        MAP_SIZE,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    commands
        .entity(tile_storage.get(&START_POS).unwrap())
        .insert(Pipe::new(PipeKind::Cross));

    commands.entity(tilemap_entity).insert((TilemapBundle {
        grid_size: PIXEL_CELL_SIZE.into(),
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: PIXEL_CELL_SIZE,
        transform: get_tilemap_center_transform(
            &MAP_SIZE,
            &PIXEL_CELL_SIZE.into(),
            &TilemapType::Square,
            2.0,
        ),
        ..Default::default()
    },));
}

pub fn advance_pipes(
    mut map_query: Query<&TileStorage>,
    pipe_query: Query<(Entity, &PipeKind, &TileFlip, &TilePos)>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let storage = map_query.single_mut();
        if pipe_query.iter().count() >= 1600 {
            for (ent, _kind, _, _pos) in pipe_query.iter() {
                commands
                    .entity(ent)
                    .remove::<PipeKind>()
                    .insert(TileTextureIndex(0));
            }
            commands
                .entity(storage.get(&START_POS).unwrap())
                .insert(Pipe::new(PipeKind::Cross));

            return;
        }

        pipe_query.iter().for_each(|(_ent, kind, flip, pos)| {
            let neighbors = Neighbors::get_square_neighboring_positions(&pos, &MAP_SIZE, false);
            use PipeKind::*;

            match *kind {
                Cross => neighbors.iter_with_direction().for_each(|(d, pos)| {
                    // we know that this pos is a valid entity
                    // we want to check the next entity over in the dir.
                    // if both this pos and the next pos are empty, we can replace them with a
                    // straight pipe each.

                    if let Some(next_pos) = pos.square_offset(&d, &MAP_SIZE) {
                        commands
                            .entity(storage.get(&pos).unwrap())
                            .insert(Pipe::new(Straight).with_flip(d));
                        commands
                            .entity(storage.get(&next_pos).unwrap())
                            .insert(Pipe::new(Straight).with_flip(d));
                    }
                }),
                Straight => {
                    neighbors
                        .entities(storage)
                        .iter_with_direction()
                        .for_each(|(d, ent)| {
                            // make sure that the direction of this neighbor, is the same as the direction
                            // the straight pipe is facing, then and only then can we place this.
                            if pipe_query.get(*ent).is_err() {
                                if Pipe::new(Straight).with_flip(d).flip == *flip {
                                    commands.entity(*ent).insert(Pipe::new(Elbow).with_flip(d));
                                }
                            }
                        })
                }

                Elbow => {
                    let connections = kind.connections_with_flip(*flip);
                    neighbors
                        .entities(storage)
                        .iter_with_direction()
                        .for_each(|(d, ent)| {
                            // make sure that the direction of this neighbor, is the same as the direction
                            // the straight pipe is facing, then and only then can we place this.

                            if pipe_query.get(*ent).is_err() && connections.contains(&d) {
                                commands
                                    .entity(*ent)
                                    .insert(Pipe::new(Cross).with_flip(d));
                            }
                        });
                }
                T => {
                    let connections = kind.connections_with_flip(*flip);
                    neighbors
                        .entities(storage)
                        .iter_with_direction()
                        .for_each(|(d, ent)| {
                            // make sure that the direction of this neighbor, is the same as the direction
                            // the straight pipe is facing, then and only then can we place this.
                            //
                            if pipe_query.get(*ent).is_err() && connections.contains(&d) {
                                commands.entity(*ent).insert(Pipe::new(Straight));
                            }
                        });
                }
            }
        });
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
        .add_systems(Startup, setup_background)
        .add_systems(Update, helpers::camera::movement)
        .add_systems(Update, advance_pipes)
        .run();
}
