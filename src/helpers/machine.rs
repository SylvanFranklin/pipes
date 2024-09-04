use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    tiles::{TileFlip, TilePos, TileTextureIndex},
};

#[allow(dead_code)]
#[derive(Component, Clone, Copy)]
pub enum PipeKind {
    Straight,
    Elbow,
    Cross,
    T,
}

#[allow(dead_code)]
impl PipeKind {
    fn default() -> Self {
        PipeKind::Straight
    }

    fn texture_index(&self) -> TileTextureIndex {
        match self {
            PipeKind::Straight => TileTextureIndex(1),
            PipeKind::Elbow => TileTextureIndex(3),
            PipeKind::Cross => TileTextureIndex(2),
            PipeKind::T => TileTextureIndex(4),
        }
    }
}

#[allow(dead_code)]
#[derive(Bundle)]
pub struct Pipe {
    pub kind: PipeKind,
    pub texture_index: TileTextureIndex,
    pub flip: TileFlip,
}

#[allow(dead_code)]
impl Pipe {
    pub fn new(kind: PipeKind) -> Self {
        Pipe {
            texture_index: kind.texture_index(),
            kind,
            flip: TileFlip::default(),
        }
    }

    pub fn next_generation(
        pipe: Pipe,
        neighbors: Neighbors<Entity>,
        pipe_query: &Query<(&PipeKind, &TileFlip, &TilePos)>,
        commands: &mut Commands,
    ) -> Self {
        if pipe.kind == PipeKind::Cross {
            if neighbors.west.is_none() {
                commands
                    .entity(neighbors.west.unwrap())
                    .insert(Pipe::new(PipeKind::Straight));
            }
        }

        return pipe;
    }
}
