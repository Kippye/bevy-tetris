use bevy::prelude::*;
use bevy::ecs::event::*;

mod consts;
mod helper;
mod window;
use window::WindowPlugin;
mod board;
mod game;
use game::{GamePlugin, GameState};
mod score;
mod ui;
use score::ScoreResource;
use ui::UIPlugin;
mod audio;
use audio::AudioPlugin;
use audio::*;

struct ProgramData {
    window_width: f32,
    window_height: f32
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1007.0,
            ..Default::default()
        })
        .insert_resource(ProgramData {
            window_width: 1920.0,
            window_height: 1008.0
        })
        .insert_resource(GameState { 
            started: true, // TEMP
            starting_level: 1,
            level: 1,
            has_piece: false
        })
        .init_resource::<ScoreResource>()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(WindowPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .run();
}

fn setup(mut commands: Commands, mut play_sound_events: ResMut<Events<PlaySoundEvent>>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .commands()
        .spawn_bundle(UiCameraBundle::default())
        .commands();

    play_sound_events.send(PlaySoundEvent { sound: Sound::GameBegin });
}