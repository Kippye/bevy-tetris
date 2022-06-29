/// Width of the game board (in segments)
pub const BOARD_WIDTH: usize = 10;

/// Height of the game board (in segments)
pub const BOARD_HEIGHT: usize = 20;

/// Size of a piece segment
pub const SEGMENT_SIZE: f32 = 40.0;

// MOVEMENT
/// The delay between movements when holding a side movement key down
pub const MOVE_DELAY: f32 = 0.125;
/// The delay between movements when holding the down key
pub const DROP_DELAY: f32 = 0.05;

// GAMEPLAY
/// The delay between each time that pieces fall on level 0 (lower on further levels)
pub const FALL_DELAY: f32 = 1.0;
/// The piece spawn X coordinate, in board coordinates
pub const PIECE_SPAWN_XP: i32 = BOARD_WIDTH as i32 / 2 - 1;
/// The piece spawn Y coordinate, in board coordinates
pub const PIECE_SPAWN_YP: i32 = BOARD_HEIGHT as i32 - 2; 
/// The amount of score rewarded for moving a piece down yourself on level 0
pub const PLACE_SCORE_BASE: usize = 1;
/// The amount of score rewarded for a single line at level 0
pub const LINE_SCORE_BASE: usize = 50;
pub const TETRIS_MULTIPLIER: usize = 26;
/// The amount of lines that must be cleared in order to progress to the next level
pub const LINES_PER_LEVEL: usize = 10;
/// The percentage that speed increases by per level (might have to make this a flat value i dunno)
pub const LEVEL_SPEED_INCREASE_PERCENTAGE: f32 = 1.26;