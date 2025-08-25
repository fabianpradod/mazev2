// src/maze.rs
use crate::framebuffer::Framebuffer;
use raylib::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap().chars().collect()).collect()
}

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '|' | '-' => Color::BLUE, // paredes azules
        ' ' => Color::WHITE,            // espacios blancos
        'p' | 'g' => Color::RED,        // inicio y meta rojos
        _ => Color::GRAY,               // cualquier otro caracter
    };

    framebuffer.set_current_color(color);

    for dy in 0..block_size {
        for dx in 0..block_size {
            let x = x0 + dx;
            let y = y0 + dy;
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, block_size: usize) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let x0 = col_index * block_size;
            let y0 = row_index * block_size;
            draw_cell(framebuffer, x0, y0, block_size, cell);
        }
    }
}
