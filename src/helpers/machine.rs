use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::SquareDirection,
    tiles::{TileFlip, TileTextureIndex},
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
    flip: TileFlip,
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

    pub fn with_flip(mut self, flip: SquareDirection) -> Self {
        use SquareDirection::*;

        self.flip = match flip {
            North | South => TileFlip {
                d: true,
                ..default()
            },
            _ => TileFlip::default(),
        };
        self
    }

    pub fn next_generation(kind: PipeKind, dir: SquareDirection) -> Self {
        match kind {
            PipeKind::Cross => Pipe::new(PipeKind::Straight).with_flip(dir),
            PipeKind::Straight => Pipe::new(PipeKind::Elbow).with_flip(dir),
            PipeKind::Elbow => Pipe::new(PipeKind::Cross).with_flip(dir),
            _ => Pipe::new(kind),
        }
    }
}
