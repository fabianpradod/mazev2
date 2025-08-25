// src/main.rs
mod caster;
mod framebuffer;
mod game_state;
mod input;
mod maze;
mod minimap;
mod player;
mod render3d;
mod resources;

use framebuffer::Framebuffer;
use game_state::{GameManager, GameState};
use input::process_events;
use maze::load_maze;
use minimap::Minimap;
use player::Player;
use raylib::prelude::*;
use render3d::render3d;
use resources::Textures;

fn main() {
    let maze = load_maze("./maze.txt");

    let block_size = 64;
    let screen_width = 1024;
    let screen_height = 512;

    let (mut rl, thread) =
        raylib::init().size(screen_width as i32, screen_height as i32).title("Maze 3D").build();

    // Print Raylib version info
    println!("Raylib version: 5.6-dev");

    let mut framebuffer = Framebuffer::new(screen_width as u32, screen_height as u32, Color::BLACK);

    let mut player =
        Player::from_maze(&maze, block_size).expect("No se encontró 'p' en el laberinto");

    let initial_player_pos = player.pos;
    let initial_player_angle = player.a;

    let mut game_manager = GameManager::new();

    // Create minimap
    let minimap = Minimap::new(screen_width as u32, screen_height as u32);

    // Load textures with raylib handle
    let textures = Textures::new(&mut rl, &thread);

    // Set target FPS
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        let fps = rl.get_fps();

        match game_manager.state {
            GameState::Menu => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    game_manager.reset();
                    // Reset player to initial position
                    player.pos = initial_player_pos;
                    player.a = initial_player_angle;
                }

                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);

                // Title
                d.draw_text(
                    "MAZE 3D",
                    screen_width / 2 - 100,
                    screen_height / 2 - 100,
                    50,
                    Color::WHITE,
                );

                // Instructions
                d.draw_text(
                    "Press SPACE to Start",
                    screen_width / 2 - 120,
                    screen_height / 2,
                    24,
                    Color::GRAY,
                );
                d.draw_text(
                    "Controls:",
                    screen_width / 2 - 50,
                    screen_height / 2 + 50,
                    20,
                    Color::DARKGRAY,
                );
                d.draw_text(
                    "W/A/S/D or Arrow Keys - Move",
                    screen_width / 2 - 140,
                    screen_height / 2 + 80,
                    18,
                    Color::DARKGRAY,
                );
                d.draw_text(
                    "Find the Goal (g) to Win!",
                    screen_width / 2 - 120,
                    screen_height / 2 + 110,
                    18,
                    Color::GREEN,
                );
                d.draw_text(
                    "Minimap shows your position",
                    screen_width / 2 - 125,
                    screen_height / 2 + 140,
                    18,
                    Color::YELLOW,
                );
            }

            GameState::Playing => {
                process_events(&rl, &mut player, &maze, block_size);

                // Check victory condition
                if player.check_victory(&maze, block_size) {
                    game_manager.state = GameState::Victory;
                }

                game_manager.level_time += delta_time;

                // Render 3D view
                framebuffer.clear();
                render3d(&mut framebuffer, &player, &maze, &textures);

                // Render minimap on top of 3D view
                minimap.render(&mut framebuffer, &player, &maze, block_size);

                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                framebuffer.draw_to_screen(&mut d);

                // Draw FPS counter
                let fps_text = format!("FPS: {}", fps);
                let text_width = measure_text(&fps_text, 20);
                d.draw_text(&fps_text, screen_width - text_width - 10, 10, 20, Color::LIME);

                // Low FPS warning
                if fps < 15 {
                    d.draw_text("LOW FPS!", screen_width - 80, 35, 16, Color::RED);
                }

                // Minimap label
                d.draw_text("MINIMAP", 30, screen_height - 30, 16, Color::WHITE);
            }

            GameState::Victory => {
                // Handle input for restart
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    // Restart the game
                    game_manager.reset();
                    player.pos = initial_player_pos;
                    player.a = initial_player_angle;
                } else if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    // Return to menu (but don't use KEY_M here since it's used for mute)
                    game_manager.state = GameState::Menu;
                    player.pos = initial_player_pos;
                    player.a = initial_player_angle;
                }

                let mut d = rl.begin_drawing(&thread);

                // Animated background color (subtle pulse effect)
                let pulse = ((game_manager.level_time * 2.0).sin() * 0.5 + 0.5) * 30.0;
                d.clear_background(Color::new(0, pulse as u8, 0, 255));

                // Draw victory message with bigger text
                d.draw_text(
                    "CONGRATULATIONS!",
                    screen_width / 2 - 200,
                    screen_height / 2 - 150,
                    45,
                    Color::GOLD,
                );

                d.draw_text(
                    "LEVEL COMPLETE!",
                    screen_width / 2 - 150,
                    screen_height / 2 - 80,
                    35,
                    Color::GREEN,
                );

                // Show completion time
                d.draw_text(
                    &format!("Completion Time: {:.2} seconds", game_manager.level_time),
                    screen_width / 2 - 150,
                    screen_height / 2 - 10,
                    24,
                    Color::WHITE,
                );

                // Show best time (optional - would need to track this)
                d.draw_rectangle(
                    screen_width / 2 - 200,
                    screen_height / 2 + 40,
                    400,
                    2,
                    Color::WHITE,
                );

                // Restart instructions
                d.draw_text(
                    "Press SPACE to Play Again",
                    screen_width / 2 - 140,
                    screen_height / 2 + 60,
                    22,
                    Color::LIGHTGRAY,
                );

                d.draw_text(
                    "Press ESC to Return to Menu",
                    screen_width / 2 - 140,
                    screen_height / 2 + 90,
                    22,
                    Color::LIGHTGRAY,
                );

                d.draw_text(
                    "Press ESC to Quit",
                    screen_width / 2 - 100,
                    screen_height / 2 + 120,
                    22,
                    Color::DARKGRAY,
                );

                // Still show FPS counter
                let fps_text = format!("FPS: {}", fps);
                let text_width = measure_text(&fps_text, 20);
                d.draw_text(&fps_text, screen_width - text_width - 10, 10, 20, Color::LIME);

                // Add some victory graphics (stars or checkmark)
                draw_victory_decoration(&mut d, screen_width, screen_height);
            }
        }
    }
}

// Helper function to measure text width
fn measure_text(text: &str, font_size: i32) -> i32 {
    (text.len() as i32 * font_size) / 2
}

// Helper function to draw victory decorations
fn draw_victory_decoration(d: &mut RaylibDrawHandle, screen_width: i32, screen_height: i32) {
    let star_positions = [
        (screen_width / 2 - 250, screen_height / 2 - 150),
        (screen_width / 2 + 250, screen_height / 2 - 150),
        (screen_width / 2 - 250, screen_height / 2 + 150),
        (screen_width / 2 + 250, screen_height / 2 + 150),
    ];

    for (x, y) in star_positions.iter() {
        draw_star(d, *x, *y, 20, Color::YELLOW);
    }
}

// Helper function to draw a simple star
fn draw_star(d: &mut RaylibDrawHandle, x: i32, y: i32, size: i32, color: Color) {
    d.draw_triangle(
        Vector2::new((x) as f32, (y - size) as f32),
        Vector2::new((x - size / 2) as f32, (y + size / 3) as f32),
        Vector2::new((x + size / 2) as f32, (y + size / 3) as f32),
        color,
    );
    d.draw_triangle(
        Vector2::new((x) as f32, (y + size) as f32),
        Vector2::new((x - size / 2) as f32, (y - size / 3) as f32),
        Vector2::new((x + size / 2) as f32, (y - size / 3) as f32),
        color,
    );
}
