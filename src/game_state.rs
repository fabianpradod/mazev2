// src/game_state.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Victory,
}

pub struct GameManager {
    pub state: GameState,
    pub level_time: f32,
}

impl GameManager {
    pub fn new() -> Self {
        Self { state: GameState::Menu, level_time: 0.0 }
    }

    pub fn reset(&mut self) {
        self.state = GameState::Playing;
        self.level_time = 0.0;
    }
}
