mod helpers;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    prelude::*,
};
use helpers::{
    background::setup_background,
    machine::{Pipe, PipeKind},
};

#[derive(Component)]
struct LastUpdate(f64);

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

    let texture_handle: Handle<Image> = asset_server.load("new_pipes.png");
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

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
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
        },
        LastUpdate(0.0),
    ));
}

fn advance_pipes(
    time: ResMut<Time>,
    mut map_query: Query<(&mut LastUpdate, &TileStorage)>,
    pipe_query: Query<(Entity, &PipeKind, &TileFlip, &TilePos)>,
    mut commands: Commands,
) {
    let (mut last_update, storage) = map_query.single_mut();
    let current_time = time.elapsed_seconds_f64();
    if (current_time - last_update.0) > 0.5 {
        if pipe_query.iter().count() >= 1000 {
            // remove all pipes and start over
            for (ent, kind, _, pos) in pipe_query.iter() {
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

        for (_ent, kind, _flip, pos) in pipe_query.iter() {
            Neighbors::get_square_neighboring_positions(&pos, &MAP_SIZE, false)
                .entities(storage)
                .iter_with_direction()
                .for_each(|(d, e)| {
                    if pipe_query.get(*e).is_ok() {
                        return;
                    }
                    let neighbors =
                        Neighbors::get_square_neighboring_positions(&pos, &MAP_SIZE, false)
                            .entities(storage)
                            .and_then(|n| pipe_query.get(n).ok());

                    commands
                        .entity(*e)
                        .insert(Pipe::next_generation(*kind, d, neighbors));
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
        .add_systems(Startup, setup_background)
        .add_systems(Update, helpers::camera::movement)
        .add_systems(Update, advance_pipes)
        .run();
}
