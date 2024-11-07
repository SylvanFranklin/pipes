mod helpers;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::prelude::*;
use helpers::{
    background::setup_background,
    machine::{GenerationRule, Pipe, PipeClusterConstructor, PipeKind},
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
    pipe_query: Query<(Entity, &PipeKind, &TilePos)>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // let storage = map_query.single_mut();
        // if pipe_query.iter().count() >= 1600 {
        //     for (ent, _kind, _pos) in pipe_query.iter() {
        //         commands
        //             .entity(ent)
        //             .remove::<PipeKind>()
        //             .insert(TileTextureIndex(0));
        //     }
        //     commands
        //         .entity(storage.get(&START_POS).unwrap())
        //         .insert(Pipe::new(PipeKind::Cross));
        //
        //     return;
        // }

        let cluster = PipeClusterConstructor::new();
        // clusters.rules contains a list of patterns that we c a list of patterns that we can
        // match, for instance [Empty] --> [Straight]

        // get all matches
        let mut matches: Vec<(Entity, &GenerationRule)> = Vec::<(Entity, &GenerationRule)>::new();
        cluster.rules.iter().for_each(|rule| {
            for (ent, kind, _pos) in pipe_query.iter() {
                // handle matching logic. There are swatchs of tiles that we have to match against,
                // we have single long lines, if there is a match along any of them we have to
                // check if the neighbors all line up in a good way too.

                // this shows that we have a match in this strip
                if rule.pattern.strip.iter().any(|itm| itm == kind) {
                    // now we have to validate that the rest of the strip matches
                    matches.push((ent, rule));
                }

                // if
                // {
                // }
            }
        });

        let mut rng = rand::thread_rng();
        use rand::seq::SliceRandom;

        // In case there are no matches
        if let Some((ent, rule)) = matches.choose(&mut rng) {
            commands
                .entity(*ent)
                .insert(Pipe::new(*rule.replace.strip.get(0).unwrap()));
        } else {
            println!("out of matches");
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
