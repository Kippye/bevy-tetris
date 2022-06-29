use bevy::{prelude::*, window::WindowResized, ecs::event::Events};

use crate::ProgramData;

fn on_window_resize(mut resize_events: EventReader<WindowResized>, mut program_data: ResMut<ProgramData>) {
    for event in resize_events.iter() {
        program_data.window_width = event.width;
        program_data.window_height = event.height;
        println!("h {}", event.height);
    }
}

pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Events<WindowResized>>()
            .add_system(on_window_resize);
    }
}