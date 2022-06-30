use crate::consts::*;
use crate::helper::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Clone, Copy)]
pub enum Shape {
    L,
    J,
    S,
    Z,
    O,
    T,
    I
}
impl Shape {
    pub fn from_int(n: i32) -> Self {
        match n {
            0 => Shape::L,
            1 => Shape::J,
            2 => Shape::S,
            3 => Shape::Z,
            4 => Shape::O,
            5 => Shape::T,
            6 => Shape::I,
            _ => { println!("Tried to convert invalid integer to Shape!"); Shape::I }
        }
    }

    pub fn get_random() -> Self {
        let mut rng = rand::thread_rng();
        Shape::from_int(rng.gen_range(0..=6))
    }

    pub fn get_rotation_offset(&self, segment_index: usize, rotation_index: usize) -> Vec2 {
        match self {
            Shape::L => {
                match rotation_index {
                    0 => {
                        match segment_index {
                            0 => Vec2::new(0.0, 1.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(0.0, -1.0),
                            3 => Vec2::new(1.0, -1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    1 => {
                        match segment_index {
                            0 => Vec2::new(1.0, 0.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(-1.0, 0.0),
                            3 => Vec2::new(-1.0, -1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    2 => {
                        match segment_index {
                            0 => Vec2::new(0.0, -1.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(0.0, 1.0),
                            3 => Vec2::new(-1.0, 1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    3 => {
                        match segment_index {
                            0 => Vec2::new(-1.0, 0.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(1.0, 0.0),
                            3 => Vec2::new(1.0, 1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    _ => Vec2::ZERO
                }
            },
            Shape::J => {
                match rotation_index {
                    0 => {
                        match segment_index {
                            0 => Vec2::new(0.0, 1.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(0.0, -1.0),
                            3 => Vec2::new(-1.0, -1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    1 => {
                        match segment_index {
                            0 => Vec2::new(1.0, 0.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(-1.0, 0.0),
                            3 => Vec2::new(-1.0, 1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    2 => {
                        match segment_index {
                            0 => Vec2::new(0.0, -1.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(0.0, 1.0),
                            3 => Vec2::new(1.0, 1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    3 => {
                        match segment_index {
                            0 => Vec2::new(-1.0, 0.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(1.0, 0.0),
                            3 => Vec2::new(1.0, -1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    _ => Vec2::ZERO
                }
            },
            Shape::S => {
                match rotation_index {
                    0 | 2 => {
                        match segment_index {
                            0 => Vec2::new(-1.0, 0.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(0.0, 1.0),
                            3 => Vec2::new(1.0, 1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    1 | 3 => {
                        match segment_index {
                            0 => Vec2::new(-1.0, 2.0),
                            1 => Vec2::new(-1.0, 1.0),
                            2 => Vec2::new(0.0, 1.0),
                            3 => Vec2::new(0.0, 0.0),
                            _ => Vec2::ZERO
                        }
                    },
                    _ => Vec2::ZERO
                }
            },
            Shape::Z => {
                match rotation_index {
                    0 | 2 => {
                        match segment_index {
                            0 => Vec2::new(-1.0, 1.0),
                            1 => Vec2::new(0.0, 1.0),
                            2 => Vec2::new(0.0, 0.0),
                            3 => Vec2::new(1.0, 0.0),
                            _ => Vec2::ZERO
                        }
                    },
                    1 | 3 => {
                        match segment_index {
                            0 => Vec2::new(0.0, 2.0),
                            1 => Vec2::new(0.0, 1.0),
                            2 => Vec2::new(-1.0, 1.0),
                            3 => Vec2::new(-1.0, 0.0),
                            _ => Vec2::ZERO
                        }
                    },
                    _ => Vec2::ZERO
                }
            },
            Shape::T => {
                match rotation_index {
                    0 => {
                        match segment_index {
                            0 => Vec2::new(-1.0, 0.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(1.0, 0.0),
                            3 => Vec2::new(0.0, -1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    1 => {
                        match segment_index {
                            0 => Vec2::new(0.0, 1.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(0.0, -1.0),
                            3 => Vec2::new(-1.0, 0.0),
                            _ => Vec2::ZERO
                        }
                    },
                    2 => {
                        match segment_index {
                            0 => Vec2::new(1.0, 0.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(-1.0, 0.0),
                            3 => Vec2::new(0.0, 1.0),
                            _ => Vec2::ZERO
                        }
                    },
                    3 => {
                        match segment_index {
                            0 => Vec2::new(0.0, -1.0),
                            1 => Vec2::new(0.0, 0.0),
                            2 => Vec2::new(0.0, 1.0),
                            3 => Vec2::new(1.0, 0.0),
                            _ => Vec2::ZERO
                        }
                    },
                    _ => Vec2::ZERO
                }
            },
            Shape::O => {
                match segment_index {
                    0 => Vec2::new(0.0, 0.0),
                    1 => Vec2::new(0.0, 1.0),
                    2 => Vec2::new(1.0, 0.0),
                    3 => Vec2::new(1.0, 1.0),
                    _ => Vec2::ZERO
                }
            },
            Shape::I => {
                match rotation_index {
                    0 | 2 => {
                        match segment_index {
                            0 => Vec2::new(0.0, 0.0),
                            1 => Vec2::new(0.0, 1.0),
                            2 => Vec2::new(0.0, 2.0),
                            3 => Vec2::new(0.0, 3.0),
                            _ => Vec2::ZERO
                        }
                    },
                    1 | 3 => {
                        match segment_index {
                            0 => Vec2::new(-2.0, 0.0),
                            1 => Vec2::new(-1.0, 0.0),
                            2 => Vec2::new(0.0, 0.0),
                            3 => Vec2::new(1.0, 0.0),
                            _ => Vec2::ZERO
                        }
                    },
                    _ => Vec2::ZERO
                }
            }
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            // NES tetris-like colors
            // Shape::L => Color::rgb(1.0, 0.0, 0.0),
            // Shape::J => Color::rgb(0.0, 0.0, 1.0),
            // Shape::S => Color::rgb(0.0, 0.0, 1.0),
            // Shape::Z => Color::rgb(1.0, 0.0, 0.0),
            // Shape::O => Color::rgb(1.0, 1.0, 1.0),
            // Shape::T => Color::rgb(1.0, 1.0, 1.0),
            // Shape::I => Color::rgb(1.0, 1.0, 1.0)
            // Different colors for every piece (still inspired by NES)
            Shape::L => Color::rgb(1.0, 0.0, 0.0),
            Shape::J => Color::rgb(0.0, 0.0, 1.0),
            Shape::S => Color::rgb(1.0, 1.0, 1.0),
            Shape::Z => Color::rgb(1.0, 1.0, 0.0),
            Shape::O => Color::rgb(0.0, 1.0, 1.0),
            Shape::T => Color::rgb(1.0, 0.0, 1.0),
            Shape::I => Color::rgb(0.0, 1.0, 0.0)
        }
    } 
}

// COMPONENTS
/// Each piece contains a vector of the positions of each segment
#[derive(Clone, Component)]
pub struct Piece {
    pub shape: Shape,
    pub position: Vec3,
    pub rotation: usize,
    pub segment_entities: Vec<Entity>,
    pub dropped_pixels: usize
}

#[derive(Copy, Clone, Component)]
pub struct Segment {
    pub segment_index: usize
}

#[derive(Component)]
pub struct TowerSegment;

// RESOURCES
/// Stores the current game board state (full / empty squares)
#[derive(Debug)]
pub struct Board;

impl Board {
    /// Checks whether or not a certain position is within the board's boundaries
    pub fn is_on_board(x: i32, y: i32, ignore_upper_boundary: bool) -> bool {
        x >= 0 && x < BOARD_WIDTH as i32 && y >= 0 && (ignore_upper_boundary || y < BOARD_HEIGHT as i32)
    }

    /// Clamps a Vec2 between the board's edges
    pub fn clamp_to_board(pos: &mut Vec3) {
        let (xp, yp) = coords_to_pixel(pos.x, pos.y);

        let (cxw, cyw) = coords_to_world(xp.clamp(0, BOARD_WIDTH as i32 - 1) as f32, yp.max(0) as f32);
        pos.x = cxw;
        pos.y = cyw;
    } 
}