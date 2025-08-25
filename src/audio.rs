// src/audio.rs
use raylib::prelude::*;

pub struct AudioManager {
    pub background_music: Option<Music>,
    pub is_music_playing: bool,
    pub music_volume: f32,
}

impl AudioManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // Try to load background music - if it fails, just continue without audio
        let background_music = rl.load_music_stream(thread, "music/background.mp3").ok();

        if background_music.is_none() {
            println!("Warning: Could not load music/background.mp3");
        }

        Self { background_music, is_music_playing: false, music_volume: 0.5 }
    }

    pub fn start_background_music(&mut self, rl: &mut RaylibHandle) {
        if let Some(ref mut music) = self.background_music {
            if !self.is_music_playing {
                rl.play_music_stream(music);
                rl.set_music_volume(music, self.music_volume);
                self.is_music_playing = true;
            }
        }
    }

    pub fn stop_background_music(&mut self, rl: &mut RaylibHandle) {
        if let Some(ref mut music) = self.background_music {
            if self.is_music_playing {
                rl.stop_music_stream(music);
                self.is_music_playing = false;
            }
        }
    }

    pub fn pause_background_music(&mut self, rl: &mut RaylibHandle) {
        if let Some(ref mut music) = self.background_music {
            if self.is_music_playing {
                rl.pause_music_stream(music);
            }
        }
    }

    pub fn resume_background_music(&mut self, rl: &mut RaylibHandle) {
        if let Some(ref mut music) = self.background_music {
            if self.is_music_playing {
                rl.resume_music_stream(music);
            }
        }
    }

    pub fn set_music_volume(&mut self, rl: &mut RaylibHandle, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        if let Some(ref mut music) = self.background_music {
            rl.set_music_volume(music, self.music_volume);
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if let Some(ref mut music) = self.background_music {
            if self.is_music_playing {
                rl.update_music_stream(music);

                // Loop the music if it's not playing
                if !rl.is_music_stream_playing(music) {
                    rl.play_music_stream(music);
                }
            }
        }
    }

    pub fn adjust_volume(&mut self, rl: &mut RaylibHandle, delta: f32) {
        let new_volume = (self.music_volume + delta).clamp(0.0, 1.0);
        self.set_music_volume(rl, new_volume);
    }

    pub fn toggle_music(&mut self, rl: &mut RaylibHandle) {
        if let Some(ref mut music) = self.background_music {
            if rl.is_music_stream_playing(music) {
                self.pause_background_music(rl);
            } else {
                self.resume_background_music(rl);
            }
        }
    }

    pub fn has_music(&self) -> bool {
        self.background_music.is_some()
    }
}
