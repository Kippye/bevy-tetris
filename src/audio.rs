use std::collections::HashMap;
use bevy::prelude::*;
use bevy::ecs::event::Events;
use rand::prelude::*;

#[derive(Clone, Copy)]
pub struct SoundRange {
    pub range: bool,
    pub min: f32,
    pub max: f32
}
impl SoundRange {
    /// Get a non-range sound range with both min and max set to the given value
    pub const fn constant(val: f32) -> Self {
        Self {
            range: false,
            min: val,
            max: val
        }
    }

    /// Construct a new SoundRange from a minimum and maximum value
    pub fn from_limits(min: f32, max: f32) -> Self {
        Self {
            range: true,
            min: min.min(max),
            max: max
        }
    }

    /// Gets a random value within in the range
    pub fn get_random_value(&self) -> f32 {
        if self.range {
            let mut rng = rand::thread_rng();
            rng.gen_range(self.min..=self.max)
        }
        else {
            self.max
        }
    }
}

/// My own custom settings for sounds that enable easily using random ranges for volume and speed
#[derive(Clone, Copy)]
pub struct SoundSettings {
    pub repeat: bool,
    pub volume_range: SoundRange,
    pub speed_range: SoundRange
}
impl Default for SoundSettings {
    fn default() -> Self {
        Self::ONCE
    }
}
impl SoundSettings {
    /// Non-randomized sound that plays once
    pub const ONCE: SoundSettings = SoundSettings {
        repeat: false,
        volume_range: SoundRange::constant(1.0),
        speed_range: SoundRange::constant(1.0)
    };

    /// Non-randomized sound that loops
    pub const LOOP: SoundSettings = SoundSettings {
        repeat: true,
        volume_range: SoundRange::constant(1.0),
        speed_range: SoundRange::constant(1.0)
    };
}

#[derive(Hash, std::cmp::Eq, Clone, Copy)]
pub enum Sound {
    GameBegin,
    PiecePlace,
    LineClear,
    TetrisClear,
    GameOver
}
impl PartialEq for Sound {
    fn eq(&self, other: &Self) -> bool {
        *self as i32 == *other as i32
    }
}
impl Sound {
    fn get_filename(&self) -> &str {
        match self {
            Sound::GameBegin => "game_begin.ogg",
            Sound::PiecePlace => "piece_place.ogg",
            Sound::LineClear => "line_clear_2.ogg",
            Sound::TetrisClear => "tetris_clear_2.ogg",
            Sound::GameOver => "game_over_3.ogg"
        }
    }

    fn get_settings(&self) -> SoundSettings {
        match self {
            Sound::PiecePlace => SoundSettings {
                repeat: false,
                volume_range: SoundRange::constant(1.0),
                speed_range: SoundRange::from_limits(0.8, 1.2)
            },
            Sound::GameOver => SoundSettings {
                volume_range: SoundRange::constant(0.6),
                ..Default::default()
            },
            Sound::GameBegin => SoundSettings {
                volume_range: SoundRange::constant(0.5),
                ..Default::default()
            },
            _ => SoundSettings::ONCE
        }
    }
}

// RESOURCES
pub struct SoundAudios(HashMap<Sound, Handle<AudioSource>>);

// EVENTS
pub struct PlaySoundEvent {
    pub sound: Sound
}

fn load_sounds(asset_server: Res<AssetServer>, mut sound_audios: ResMut<SoundAudios>) {
    sound_audios.0.insert(Sound::GameBegin, asset_server.load(&*format!("sounds/{}", Sound::GameBegin.get_filename())));    
    sound_audios.0.insert(Sound::PiecePlace, asset_server.load(&*format!("sounds/{}", Sound::PiecePlace.get_filename())));
    sound_audios.0.insert(Sound::LineClear, asset_server.load(&*format!("sounds/{}", Sound::LineClear.get_filename())));
    sound_audios.0.insert(Sound::TetrisClear, asset_server.load(&*format!("sounds/{}", Sound::TetrisClear.get_filename())));
    sound_audios.0.insert(Sound::GameOver, asset_server.load(&*format!("sounds/{}", Sound::GameOver.get_filename())));
}

fn play_sound(audio: Res<Audio>, mut play_sound_listener: EventReader<PlaySoundEvent>, sound_audios: Res<SoundAudios>) {
    for event in play_sound_listener.iter() {
        match sound_audios.0.get(&event.sound) {
            Some(sound) => { 
                let sound_settings = event.sound.get_settings();
                audio.play_with_settings(sound.clone(), PlaybackSettings { repeat: sound_settings.repeat, volume: sound_settings.volume_range.get_random_value(), speed: sound_settings.speed_range.get_random_value() }); 
            },
            None => { return; }
        }
    }
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Events<PlaySoundEvent>>()
            .insert_resource(SoundAudios(HashMap::new()))
            .add_startup_system(load_sounds)
            .add_system(play_sound);
    }
}