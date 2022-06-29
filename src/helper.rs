use crate::consts::*;

/// Turns a "board position" (ranging from 0-9; 0-19) into a transform translation position
pub fn coords_to_world(x: f32, y: f32) -> (f32, f32) {
    ((x - (BOARD_WIDTH as f32 / 2.0 - 0.5)) * SEGMENT_SIZE, (y - (BOARD_HEIGHT as f32 / 2.0 - 0.5)) * SEGMENT_SIZE)
}

// Turns a world position into a "board position" (clamped)
pub fn coords_to_pixel(x: f32, y: f32) -> (i32, i32) {
    ((x / SEGMENT_SIZE + (BOARD_WIDTH as f32 / 2.0 - 0.5)).round() as i32, (y / SEGMENT_SIZE + (BOARD_HEIGHT as f32 / 2.0 - 0.5)).round() as i32)
}