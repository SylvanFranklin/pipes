use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    tiles::{TileFlip, TileTextureIndex},
};

#[allow(dead_code)]
#[derive(Component, Clone, Copy, Eq, PartialEq, Debug)]
pub enum PipeKind {
    Any,
    Empty,
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
            _ => TileTextureIndex(0),
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

            _ => vec![],
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
}

#[derive(Clone)]
pub struct GenerationRule {
    pub pattern: Vec<PipeKind>,
    pub replace: Vec<PipeKind>,
    pub interrupt: bool,
}

impl GenerationRule {
    pub fn new(raw_pattern: &str, raw_replace: &str) -> Self {
        let mut pattern: Vec<PipeKind> = Vec::new();
        let mut replace: Vec<PipeKind> = Vec::new();

        use PipeKind::*;
        String::from(raw_replace).chars().for_each(|c| {
            match c {
                ' ' => replace.push(Empty),
                '+' => replace.push(Cross),
                '-' => replace.push(Straight),
                'l' => replace.push(Elbow),
                'T' => replace.push(T),
                '*' => replace.push(Any),
                _ => {}
            };
        });
        String::from(raw_pattern).chars().for_each(|c| {
            match c {
                ' ' => pattern.push(Empty),
                '+' => pattern.push(Cross),
                '-' => pattern.push(Straight),
                'l' => pattern.push(Elbow),
                'T' => pattern.push(T),
                '*' => pattern.push(Any),
                _ => {}
            };
        });
        let interrupt = false;

        GenerationRule {
            pattern,
            replace,
            interrupt,
        }
    }
}

pub struct PipeClusterConstructor {
    pub rules: Vec<GenerationRule>,
}

impl PipeClusterConstructor {
    pub fn new() -> Self {
        Self {
            rules: vec![GenerationRule::new(" ", "-")],
        }
    }
}
