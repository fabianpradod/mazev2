// src/render3d.rs
use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::resources::Textures;
use raylib::prelude::*;

pub fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    textures: &Textures,
) {
    let block_size = 64;
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    for i in (0..num_rays).step_by(2) {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        if intersect.distance < 1.0 {
            continue;
        }

        // Correct fisheye effect
        let angle_diff = a - player.a;
        let corrected_distance = intersect.distance * angle_diff.cos();

        // Calculate stake height
        let stake_height = (hh * 100.0) / corrected_distance;
        let stake_height = stake_height.min(framebuffer.height as f32);

        // Calculate vertical positions
        let stake_top = ((hh - (stake_height / 2.0)).max(0.0)) as usize;
        let stake_bottom = ((hh + (stake_height / 2.0)).min(framebuffer.height as f32)) as usize;

        // Special rendering for goal tile - make it glow with pulsing effect
        if intersect.impact == 'g' {
            // Make goal walls golden/yellow
            let goal_color = Color::new(255, 215, 0, 255); // Gold color

            for y in stake_top..stake_bottom {
                if y < framebuffer.height as usize {
                    // Apply pulsing effect
                    let pulse = ((y as f32 * 0.1).sin() * 0.5 + 0.5);
                    let shaded_color = Color::new(
                        (goal_color.r as f32 * pulse) as u8,
                        (goal_color.g as f32 * pulse) as u8,
                        goal_color.b,
                        255,
                    );

                    framebuffer.set_current_color(shaded_color);
                    framebuffer.set_pixel(i as u32, y as u32);
                    if (i + 1) as u32 <= framebuffer.width - 1 {
                        framebuffer.set_pixel((i + 1) as u32, y as u32);
                    }
                }
            }
            continue;
        }

        // Get the appropriate texture
        let texture = textures.get_wall_texture(intersect.impact);

        // Calculate texture coordinate
        let hit_x = player.pos.x + intersect.distance * a.cos();
        let hit_y = player.pos.y + intersect.distance * a.sin();

        let tex_coord_x = match intersect.impact {
            '|' => (hit_y % block_size as f32) / block_size as f32,
            '-' | '+' | '#' | '*' => (hit_x % block_size as f32) / block_size as f32,
            _ => (hit_x % block_size as f32) / block_size as f32,
        };

        // Draw the column
        for y in stake_top..stake_bottom {
            if y < framebuffer.height as usize {
                // Calculate texture Y coordinate
                let tex_coord_y = (y - stake_top) as f32 / (stake_bottom - stake_top) as f32;

                // Sample the texture
                let tex_x = (tex_coord_x * textures.texture_size as f32) as usize;
                let tex_y = (tex_coord_y * textures.texture_size as f32) as usize;

                // Get pixel color from texture
                let color = textures.get_texture_pixel(
                    texture,
                    tex_x.min(textures.texture_size - 1),
                    tex_y.min(textures.texture_size - 1),
                );

                // Apply distance shading
                let intensity = 1.0 - (corrected_distance / 500.0).min(0.8);
                let shaded_color = Color::new(
                    (color.r as f32 * intensity) as u8,
                    (color.g as f32 * intensity) as u8,
                    (color.b as f32 * intensity) as u8,
                    255,
                );

                framebuffer.set_current_color(shaded_color);
                framebuffer.set_pixel(i as u32, y as u32);
                if (i + 1) as u32 <= framebuffer.width - 1 {
                    framebuffer.set_pixel((i + 1) as u32, y as u32);
                }
            }
        }
    }
}
