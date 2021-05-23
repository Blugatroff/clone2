use std::fmt::Debug;
use ton::Player;
use ton::Sample;

use crate::state::DAY_LENGTH;

pub struct Time {
    pub start: std::time::Instant,
    pub day_time: f32,
}
impl Default for Time {
    fn default() -> Self {
        Self {
            start: std::time::Instant::now(),
            day_time: 0.5,
        }
    }
}
impl Time {
    pub fn update(&mut self) {
        self.day_time = self.start.elapsed().as_secs_f32() / DAY_LENGTH + 0.5;
    }
}

pub struct DeltaTime(pub f32);
impl Default for DeltaTime {
    fn default() -> Self {
        Self(0.0)
    }
}

pub struct Sink(Option<ton::Sink>);
#[allow(dead_code)]
impl Sink {
    pub fn sleep_until_end(&self) {
        if let Some(sink) = &self.0 {
            sink.sleep_until_end()
        }
    }
    pub fn detach(self) {
        if let Some(sink) = self.0 {
            sink.detach()
        }
    }
    pub fn stop(&self) {
        if let Some(sink) = &self.0 {
            sink.stop()
        }
    }
    pub fn play(&self) {
        if let Some(sink) = &self.0 {
            sink.play()
        }
    }
    pub fn pause(&self) {
        if let Some(sink) = &self.0 {
            sink.pause()
        }
    }
    pub fn set_emmitter_position(&self, pos: [f32; 3]) {
        if let Some(sink) = &self.0 {
            sink.set_emitter_position(pos)
        }
    }
    pub fn set_left_ear_position(&self, pos: [f32; 3]) {
        if let Some(sink) = &self.0 {
            sink.set_left_ear_position(pos)
        }
    }
    pub fn set_right_ear_position(&self, pos: [f32; 3]) {
        if let Some(sink) = &self.0 {
            sink.set_right_ear_position(pos)
        }
    }
    pub fn set_volume(&self, volume: f32) {
        if let Some(sink) = &self.0 {
            sink.set_volume(volume)
        }
    }
}
pub struct SoundPlayer(Option<Player>);
impl SoundPlayer {
    pub fn new() -> Self {
        SoundPlayer(Player::new().ok())
    }
    pub fn play<S>(&self, source: S) -> Sink
    where
        S: ton::Source + Send + 'static,
        S::Item: Sample + Send + Debug,
    {
        Sink(self.0.as_ref().map(|p| p.play(source).ok()).flatten())
    }
}
impl Default for SoundPlayer {
    fn default() -> Self {
        Self::new()
    }
}
