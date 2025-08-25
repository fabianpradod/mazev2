// src/player.rs
use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
    pub a: f32, // angle
    pub fov: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self { pos: Vector2::new(x, y), a: PI / 3.0, fov: PI / 3.0 }
    }

    pub fn from_maze(maze: &Vec<Vec<char>>, block_size: usize) -> Option<Self> {
        for (row_index, row) in maze.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                if cell == 'p' {
                    let x = (col_index * block_size + block_size / 2) as f32;
                    let y = (row_index * block_size + block_size / 2) as f32;
                    return Some(Self::new(x, y));
                }
            }
        }
        None
    }

    pub fn check_victory(&self, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
        let maze_x = (self.pos.x as usize) / block_size;
        let maze_y = (self.pos.y as usize) / block_size;

        if maze_y < maze.len() && maze_x < maze[0].len() {
            return maze[maze_y][maze_x] == 'g';
        }
        false
    }
}
