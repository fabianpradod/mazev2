// src/resources.rs
use raylib::prelude::*;

pub struct Textures {
    pub wall_brick: Vec<Color>,
    pub wall_stone: Vec<Color>,
    pub wall_metal: Vec<Color>,
    pub texture_size: usize,
}

impl Textures {
    pub fn new(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Self {
        // Load images
        let mut brick_img =
            Image::load_image("textures/brick.png").expect("Failed to load brick.png");
        let mut stone_img =
            Image::load_image("textures/stone.png").expect("Failed to load stone.png");
        let mut metal_img =
            Image::load_image("textures/metal.png").expect("Failed to load metal.png");

        let texture_size = brick_img.width as usize;

        let wall_brick = Self::image_to_color_array(&mut brick_img);
        let wall_stone = Self::image_to_color_array(&mut stone_img);
        let wall_metal = Self::image_to_color_array(&mut metal_img);

        Self { wall_brick, wall_stone, wall_metal, texture_size }
    }

    fn image_to_color_array(img: &mut Image) -> Vec<Color> {
        let mut colors = Vec::new();
        for y in 0..img.height {
            for x in 0..img.width {
                colors.push(img.get_color(x, y));
            }
        }
        colors
    }

    pub fn get_wall_texture(&self, wall_type: char) -> &Vec<Color> {
        match wall_type {
            '#' => &self.wall_stone,
            '*' => &self.wall_metal,
            '+' | '|' | '-' => &self.wall_brick,
            _ => &self.wall_brick,
        }
    }

    pub fn get_texture_pixel(&self, texture: &Vec<Color>, x: usize, y: usize) -> Color {
        let index = (y * self.texture_size + x).min(texture.len() - 1);
        texture[index]
    }
}
