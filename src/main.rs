mod helpers;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::{helpers::square_grid::neighbors::Neighbors, prelude::*};
use helpers::{
    background::setup_background,
    machine::{GenerationRule, Pipe, PipeKind},
};

pub const MAP_SIZE: TilemapSize = TilemapSize { x: 20, y: 20 };
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
                scale: 4.,
                ..default()
            },

            ..default()
        },
        BloomSettings::NATURAL,
    ));

    let texture_handle: Handle<Image> = asset_server.load("pipes.png");
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..MAP_SIZE.x {
        for y in 0..MAP_SIZE.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .insert(Pipe::new(PipeKind::Empty))
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    commands
        .entity(tile_storage.get(&START_POS).unwrap())
        .insert(Pipe::new(PipeKind::Cross));

    commands.entity(tilemap_entity).insert((TilemapBundle {
        grid_size: PIXEL_CELL_SIZE.into(),
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
    pipe_query: Query<(Entity, &PipeKind, &TilePos)>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let storage: &TileStorage = map_query.single_mut();
        use PipeKind::*;
        let rules: Vec<GenerationRule> = vec![
            GenerationRule::new([Cross, Empty], [Cross, Straight]),
            GenerationRule::new([Empty, Straight], [Straight, Elbow]),
        ];

        'rules: for rule in rules.iter() {
            for (e, k, p) in pipe_query.iter() {
                if rule.pattern[0] == *k {
                    for (dir, np) in
                        Neighbors::get_square_neighboring_positions(p, &MAP_SIZE, false)
                            .iter_with_direction()
                    {
                        let (ne, nk, _) = pipe_query.get(storage.get(np).unwrap()).unwrap();
                        if *nk == rule.pattern[1] {
                            // figure out how to get out of any without any side effects
                            commands
                                .get_entity(e)
                                .unwrap()
                                .insert(Pipe::new(rule.replacement[0]));

                            commands
                                .get_entity(ne)
                                .unwrap()
                                .insert(Pipe::new(rule.replacement[1]).with_flip(dir));

                            break 'rules;
                        }
                    }
                }
            }
        }
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
