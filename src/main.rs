mod helpers;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::prelude::*;

#[allow(dead_code)]
#[derive(Component)]
struct LastUpdate(f64);

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_color: ResMut<Assets<ColorMaterial>>,
) {
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
    let texture_handle: Handle<Image> = asset_server.load("new_pipes.png");
    let map_size = TilemapSize { x: 60, y: 60 };
    let map_type = TilemapType::Square;
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let tile_size = TilemapTileSize { x: 128.0, y: 128.0 };
    let grid_size = tile_size.into();
    let start_pos = TilePos {
        x: map_size.x / 2,
        y: map_size.y / 2,
    };

    commands.spawn(MaterialMesh2dBundle {
        transform: Transform {
            translation: vec3(0., 0., 0.5),
            ..default()
        },
        mesh: meshes
            .add(Rectangle::new(
                (map_size.x as f32) * tile_size.x,
                (map_size.y as f32) * tile_size.y,
            ))
            .into(),
        material: materials_color.add(Color::srgb(0.02, 0.02, 0.02)), // RGB values exceed 1 to achieve a bright color for the bloom effect
        ..default()
    });

    commands.spawn(ColorMesh2dBundle {
        transform: Transform {
            translation: vec3(0., 0., 1.),
            ..default()
        },
        mesh: meshes.add(Circle::new(300.)).into(),
        material: materials_color.add(Color::srgb(5., 5., 5.)), // RGB values exceed 1 to achieve a bright color for the bloom effect
        ..default()
    });

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

    commands
        .entity(tile_storage.get(&start_pos).unwrap())
        .insert(TileTextureIndex(0));

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            size: map_size,
            storage: tile_storage,
            map_type,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 2.0),
            ..Default::default()
        },
        LastUpdate(0.0),
    ));
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
        .run();
}
