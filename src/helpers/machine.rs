use bevy::prelude::Component;
use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::SquareDirection;
use bevy_ecs_tilemap::prelude::*;

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub enum PipeKind {
    Straight,
    Elbow,
    Cross,
    T,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct Pipe {
    pub kind: PipeKind,
    pub connections: Connections,
    flip: TileFlip,
    pub texture_index: TileTextureIndex,
}

#[allow(dead_code)]
pub struct Connections {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}

impl Connections {
    pub fn to_vec(&self) -> Vec<SquareDirection> {
        let mut set: Vec<SquareDirection> = Vec::new();

        if self.north {
            set.push(SquareDirection::North)
        }

        if self.south {
            set.push(SquareDirection::South)
        }

        if self.east {
            set.push(SquareDirection::East)
        }

        if self.west {
            set.push(SquareDirection::West)
        }
        set
    }
}

impl Connections {
    pub fn new(con: Vec<SquareDirection>) -> Self {
        let mut this = Self::default();
        con.iter().for_each(|d| match d {
            SquareDirection::North => this.north = true,
            SquareDirection::South => this.south = true,
            SquareDirection::East => this.east = true,
            SquareDirection::West => this.west = true,
            _ => {}
        });

        return this;
    }

    pub fn default() -> Self {
        return Connections {
            north: false,
            south: false,
            east: false,
            west: false,
        };
    }
}

#[allow(dead_code)]
impl Pipe {
    pub fn new(kind: PipeKind) -> Self {
        Pipe {
            kind,
            connections: Pipe::get_connections(&kind),
            flip: TileFlip::default(),
            texture_index: Pipe::get_texture_index(kind),
        }
    }

    // pub fn add_flip(&mut self, flip: Tilp) {
    //     self.flip =
    // }

    pub fn spread_to_tile(self) -> (TileTextureIndex, TileFlip, Pipe) {
        (self.texture_index, self.flip, self)
    }

    pub fn get_connections(kind: &PipeKind) -> Connections {
        return match kind {
            PipeKind::Straight => {
                Connections::new(vec![SquareDirection::North, SquareDirection::South])
            }
            PipeKind::Elbow => {
                Connections::new(vec![SquareDirection::East, SquareDirection::South])
            }
            PipeKind::Cross => Connections::new(vec![
                SquareDirection::North,
                SquareDirection::East,
                SquareDirection::South,
                SquareDirection::West,
            ]),
            PipeKind::T => Connections::new(vec![
                SquareDirection::South,
                SquareDirection::East,
                SquareDirection::West,
            ]),
        };
    }

    // pub fn get_random_connection(&self) -> Option<&SquareDirection> {
    //     use rand::seq::SliceRandom;
    //     self.connections.choose(&mut rand::thread_rng())
    // }

    fn get_texture_index(kind: PipeKind) -> TileTextureIndex {
        use PipeKind::*;
        match kind {
            Straight => TileTextureIndex(1),
            Cross => TileTextureIndex(2),
            Elbow => TileTextureIndex(3),
            T => TileTextureIndex(3),
        }
    }

    // pub fn insert_into_tilemap

    pub fn random_from_dir(dir: &SquareDirection) -> Self {
        match dir {
            SquareDirection::North | SquareDirection::South => Pipe {
                flip: TileFlip {
                    d: true,
                    ..default()
                },
                ..Pipe::new(PipeKind::Straight)
            },
            SquareDirection::East | SquareDirection::West => Pipe::new(PipeKind::Straight),
            _ => Pipe::new(PipeKind::Cross),
        }
    }
}

#[allow(dead_code)]
pub const ALL_PIPES: [PipeKind; 4] = [
    PipeKind::Straight,
    PipeKind::Elbow,
    PipeKind::Cross,
    PipeKind::T,
];
