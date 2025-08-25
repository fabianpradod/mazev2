// src/caster.rs
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use raylib::prelude::*;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;
    let max_distance = 1000.0;

    framebuffer.set_current_color(Color::WHITESMOKE);

    while d < max_distance {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = player.pos.x + cos;
        let y = player.pos.y + sin;

        if x < 0.0 || y < 0.0 || x >= framebuffer.width as f32 || y >= framebuffer.height as f32 {
            return Intersect { distance: d, impact: ' ' };
        }

        let i = (x as usize) / block_size;
        let j = (y as usize) / block_size;

        if j >= maze.len() || i >= maze[0].len() || (maze[j][i] != ' ' && maze[j][i] != 'p') {
            return Intersect {
                distance: d,
                impact: if j >= maze.len() || i >= maze[0].len() { ' ' } else { maze[j][i] },
            };
        }

        if draw_line {
            framebuffer.set_pixel(x as u32, y as u32);
        }

        d += 1.0;
    }

    Intersect { distance: max_distance, impact: ' ' }
}
