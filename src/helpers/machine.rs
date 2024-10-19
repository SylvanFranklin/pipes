use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    tiles::{TileFlip, TilePos, TileTextureIndex},
};

#[allow(dead_code)]
#[derive(Component, Clone, Copy, Eq, PartialEq)]
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
            PipeKind::Cross => TileTextureIndex(2),
            PipeKind::Elbow => TileTextureIndex(4),
            PipeKind::T => TileTextureIndex(3),
        }
    }

    pub fn connections_with_flip(&self, flip: TileFlip) -> Vec<SquareDirection> {
        use SquareDirection::*;
        let mut connections = self.connections();
        if flip.d {
            connections = connections
                .iter()
                .map(|d| match d {
                    North => West,
                    South => East,
                    East => South,
                    West => North,
                    _ => *d,
                })
                .collect();
        }

        connections
    }

    pub fn connections(&self) -> Vec<SquareDirection> {
        use PipeKind::*;
        match self {
            Straight => vec![SquareDirection::East, SquareDirection::West],
            Elbow => vec![SquareDirection::South, SquareDirection::West],
            Cross => vec![
                SquareDirection::North,
                SquareDirection::East,
                SquareDirection::South,
                SquareDirection::West,
            ],
            T => vec![
                SquareDirection::North,
                SquareDirection::East,
                SquareDirection::South,
            ],
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

    pub fn connections(&self) -> Vec<SquareDirection> {
        self.kind
            .connections()
            .iter()
            .map(|d| {
                if self.flip.d {
                    match d {
                        SquareDirection::North => SquareDirection::South,
                        SquareDirection::South => SquareDirection::North,
                        SquareDirection::East => SquareDirection::West,
                        SquareDirection::West => SquareDirection::East,
                        _ => unreachable!(),
                    }
                } else {
                    *d
                }
            })
            .collect()
    }

    pub fn with_flip(mut self, flip: SquareDirection) -> Self {
        use PipeKind::*;
        use SquareDirection::*;

        self.flip = match flip {
            North | South if self.kind == Straight => TileFlip {
                d: true,
                ..default()
            },
            South | West if self.kind == Elbow => TileFlip {
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
        }
    }
}
