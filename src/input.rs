// src/input.rs
use crate::player::Player;
use raylib::prelude::*;
use std::f32::consts::PI;

pub fn process_events(
    rl: &RaylibHandle,
    player: &mut Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    const MOVE_SPEED: f32 = 3.0;
    const ROTATION_SPEED: f32 = PI / 60.0;

    if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
        player.a -= ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
        player.a += ROTATION_SPEED;
    }

    if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
        let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.a.sin();

        if is_valid_position(new_x, new_y, maze, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        } else {
        }
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
        let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.a.sin();

        if is_valid_position(new_x, new_y, maze, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        } else {
        }
    }
}

fn is_valid_position(x: f32, y: f32, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
    if x < 0.0 || y < 0.0 {
        return false;
    }

    let maze_x = (x as usize) / block_size;
    let maze_y = (y as usize) / block_size;

    if maze_y >= maze.len() || maze_x >= maze[0].len() {
        return false;
    }

    let cell = maze[maze_y][maze_x];
    let is_valid = cell == ' ' || cell == 'p' || cell == 'g';

    is_valid
}
