// src/minimap.rs
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Minimap {
    pub size: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub scale: f32,
}

impl Minimap {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let size = 210;
        let padding = 20;
        let offset_x = padding; // Move to left side
        let offset_y = padding;

        Self {
            size,
            offset_x,
            offset_y,
            scale: 3.2, // Reduced scale to prevent overflow
        }
    }

    pub fn render(
        &self,
        framebuffer: &mut Framebuffer,
        player: &Player,
        maze: &Vec<Vec<char>>,
        block_size: usize,
    ) {
        // Draw minimap background (semi-transparent black)
        self.draw_background(framebuffer);

        // Draw maze walls
        self.draw_maze(framebuffer, maze, block_size);

        // Draw player FOV cone
        self.draw_fov_cone(framebuffer, player, block_size);

        // Draw player position and direction
        self.draw_player(framebuffer, player, block_size);

        // Draw border around minimap
        self.draw_border(framebuffer);
    }

    fn draw_background(&self, framebuffer: &mut Framebuffer) {
        framebuffer.set_current_color(Color::new(0, 0, 0, 180));
        framebuffer.draw_rectangle(
            self.offset_x as i32,
            self.offset_y as i32,
            self.size as i32,
            self.size as i32,
        );
    }

    fn draw_maze(&self, framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, block_size: usize) {
        let maze_width = maze[0].len() * block_size;
        let maze_height = maze.len() * block_size;

        // Calculate center offset to center the maze in the minimap
        let center_offset_x = (self.size as f32 - maze_width as f32 / self.scale) / 2.0;
        let center_offset_y = (self.size as f32 - maze_height as f32 / self.scale) / 2.0;

        for (row_idx, row) in maze.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell != ' ' && cell != 'p' {
                    let world_x = (col_idx * block_size) as f32;
                    let world_y = (row_idx * block_size) as f32;

                    let minimap_x = self.offset_x as f32 + center_offset_x + world_x / self.scale;
                    let minimap_y = self.offset_y as f32 + center_offset_y + world_y / self.scale;
                    let cell_size = ((block_size as f32 / self.scale) as i32).max(2);

                    if self.is_in_minimap_bounds(minimap_x as i32, minimap_y as i32) {
                        let color = match cell {
                            '#' => Color::DARKGRAY,
                            '*' => Color::GRAY,
                            'g' => Color::GOLD,
                            _ => Color::BLUE, // Default wall color
                        };

                        framebuffer.set_current_color(color);
                        framebuffer.draw_rectangle(
                            minimap_x as i32,
                            minimap_y as i32,
                            cell_size,
                            cell_size,
                        );
                    }
                }
            }
        }
    }

    fn draw_fov_cone(&self, framebuffer: &mut Framebuffer, player: &Player, block_size: usize) {
        let maze_width = 9 * block_size; // Assuming your maze is roughly 9 units wide
        let maze_height = 9 * block_size; // Assuming your maze is roughly 9 units tall

        let center_offset_x = (self.size as f32 - maze_width as f32 / self.scale) / 2.0;
        let center_offset_y = (self.size as f32 - maze_height as f32 / self.scale) / 2.0;

        let player_minimap_x = self.offset_x as f32 + center_offset_x + player.pos.x / self.scale;
        let player_minimap_y = self.offset_y as f32 + center_offset_y + player.pos.y / self.scale;

        let cone_length = 50.0;
        let half_fov = player.fov / 2.0;

        // Draw FOV cone lines
        framebuffer.set_current_color(Color::new(255, 255, 0, 100)); // Semi-transparent yellow

        // Left edge of FOV
        let left_angle = player.a - half_fov;
        let left_end_x = player_minimap_x + cone_length * left_angle.cos();
        let left_end_y = player_minimap_y + cone_length * left_angle.sin();

        framebuffer.draw_line(
            player_minimap_x as i32,
            player_minimap_y as i32,
            left_end_x as i32,
            left_end_y as i32,
        );

        // Right edge of FOV
        let right_angle = player.a + half_fov;
        let right_end_x = player_minimap_x + cone_length * right_angle.cos();
        let right_end_y = player_minimap_y + cone_length * right_angle.sin();

        framebuffer.draw_line(
            player_minimap_x as i32,
            player_minimap_y as i32,
            right_end_x as i32,
            right_end_y as i32,
        );
    }

    fn draw_player(&self, framebuffer: &mut Framebuffer, player: &Player, block_size: usize) {
        let maze_width = 9 * block_size; // Assuming your maze is roughly 9 units wide
        let maze_height = 9 * block_size; // Assuming your maze is roughly 9 units tall

        let center_offset_x = (self.size as f32 - maze_width as f32 / self.scale) / 2.0;
        let center_offset_y = (self.size as f32 - maze_height as f32 / self.scale) / 2.0;

        let player_minimap_x =
            (self.offset_x as f32 + center_offset_x + player.pos.x / self.scale) as i32;
        let player_minimap_y =
            (self.offset_y as f32 + center_offset_y + player.pos.y / self.scale) as i32;

        if self.is_in_minimap_bounds(player_minimap_x, player_minimap_y) {
            // Draw player as a red circle
            framebuffer.set_current_color(Color::RED);
            framebuffer.draw_circle(player_minimap_x, player_minimap_y, 4);

            // Draw direction line
            let line_length = 15.0;
            let end_x = player_minimap_x as f32 + line_length * player.a.cos();
            let end_y = player_minimap_y as f32 + line_length * player.a.sin();

            framebuffer.set_current_color(Color::WHITE);
            framebuffer.draw_line(player_minimap_x, player_minimap_y, end_x as i32, end_y as i32);
        }
    }

    fn draw_border(&self, framebuffer: &mut Framebuffer) {
        framebuffer.set_current_color(Color::WHITE);

        // Top border
        framebuffer.draw_rectangle(
            self.offset_x as i32 - 1,
            self.offset_y as i32 - 1,
            self.size as i32 + 2,
            2,
        );

        // Bottom border
        framebuffer.draw_rectangle(
            self.offset_x as i32 - 1,
            (self.offset_y + self.size) as i32 - 1,
            self.size as i32 + 2,
            2,
        );

        // Left border
        framebuffer.draw_rectangle(
            self.offset_x as i32 - 1,
            self.offset_y as i32 - 1,
            2,
            self.size as i32 + 2,
        );

        // Right border
        framebuffer.draw_rectangle(
            (self.offset_x + self.size) as i32 - 1,
            self.offset_y as i32 - 1,
            2,
            self.size as i32 + 2,
        );
    }

    fn world_to_minimap_x(&self, world_x: f32) -> i32 {
        (self.offset_x as f32 + world_x / self.scale) as i32
    }

    fn world_to_minimap_y(&self, world_y: f32) -> i32 {
        (self.offset_y as f32 + world_y / self.scale) as i32
    }

    fn is_in_minimap_bounds(&self, x: i32, y: i32) -> bool {
        x >= self.offset_x as i32
            && x < (self.offset_x + self.size) as i32
            && y >= self.offset_y as i32
            && y < (self.offset_y + self.size) as i32
    }
}
