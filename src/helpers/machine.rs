use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    tiles::{TileFlip, TilePos, TileTextureIndex},
};

#[allow(dead_code)]
#[derive(Component, Clone, Copy)]
pub enum PipeKind {
    None,
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
            PipeKind::None => TileTextureIndex(0),
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

    pub fn next_generation(
        kind: PipeKind,
        dir: SquareDirection,
        neighbors: Neighbors<(Entity, &PipeKind, &TileFlip, &TilePos)>,
    ) -> Self {
        // currently there is an overlap with neighbors.dir and kind

        use PipeKind::*;
        match kind {
            Cross => Pipe::new(Straight).with_flip(dir),
            Straight => {
                // randomy one in three chance for an L piece
                if rand::random::<f32>() < 0.33 {
                    return Pipe::new(T).with_flip(dir);
                }

                if neighbors.north.is_none() || neighbors.south.is_some() {
                    return Pipe::new(Cross);
                }

                return Pipe::new(Elbow).with_flip(dir);
            }
            T => {
                if neighbors.north.is_none() {
                    return Pipe::new(Straight).with_flip(SquareDirection::North);
                }

                if neighbors.east.is_none() {
                    return Pipe::new(Straight).with_flip(SquareDirection::East);
                }

                if neighbors.south.is_none() {
                    return Pipe::new(Straight).with_flip(SquareDirection::South);
                }

                if neighbors.west.is_none() {
                    return Pipe::new(Straight).with_flip(SquareDirection::West);
                }

                return Pipe::new(Cross);
            }
            Elbow => Pipe::new(Straight),
            _ => Pipe::new(None),
        }
    }
}
