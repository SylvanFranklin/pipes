use bevy::{math::vec3, prelude::*, sprite::MaterialMesh2dBundle};

use crate::{MAP_SIZE, PIXEL_CELL_SIZE};

pub fn setup_background(
    mut commands: Commands,
    mut materials_color: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        transform: Transform {
            translation: vec3(0., 0., 0.5),
            ..default()
        },
        mesh: meshes
            .add(Rectangle::new(
                (MAP_SIZE.x as f32) * PIXEL_CELL_SIZE.x,
                (MAP_SIZE.y as f32) * PIXEL_CELL_SIZE.y,
            ))
            .into(),
        material: materials_color.add(Color::srgb(1.1, 1.1, 1.1)), // RGB values exceed 1 to achieve a bright color for the bloom effect
        // material: materials_color.add(Color::srgb(0.01, 0.01, 0.01)), // RGB values exceed 1 to achieve a bright color for the bloom effect
        ..default()
    });

    // commands.spawn(ColorMesh2dBundle {
    //     transform: Transform {
    //         translation: vec3(0., 0., 1.),
    //         ..default()
    //     },
    //     mesh: meshes.add(Circle::new(200.)).into(),
    //     material: materials_color.add(Color::srgb(5., 5., 5.)), // RGB values exceed 1 to achieve a bright color for the bloom effect
    //     ..default()
    // });
}
