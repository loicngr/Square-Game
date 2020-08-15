extern crate sdl2;

mod core;
mod entity;

use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::entity::EntityDirection;
use crate::core::{GameWorld, GameWindow};

const COLOR_BACKGROUND: Color = Color::RGBA(0, 0, 0, 1);
const PLAYER_ENTITY_ID: &str = "entity_1";


// Main function
fn main() {
    println!("---- Start ----");
    println!("\n");

        let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(COLOR_BACKGROUND);
    canvas.clear();
    canvas.present();

    let window_border_size = canvas.window().drawable_size();

    // My mutable GameWorld instance
    let mut game_world = GameWorld {
        entities: entity::create_entities(), // Contains entities created in create_entities() function
        window: GameWindow {
            x: window_border_size.0 as i32,
            y: window_border_size.1 as i32
        }

    };
    game_world.draw_entities(vec![PLAYER_ENTITY_ID], &mut canvas);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'game_loop: loop {
        game_world.window.set(&canvas);

        let game_window_size_x = game_world.window.get_x();
        let game_window_size_y = game_world.window.get_y();

        let entity_player = game_world.get_entity(PLAYER_ENTITY_ID);
        entity_player.direction = EntityDirection::Idle;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let in_window = entity_player.check_entity_collision_right(game_window_size_x);
                    if in_window == 1 {
                        entity_player.direction = EntityDirection::Right;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let in_window = entity_player.check_entity_collision_left();
                    if in_window == 1 {
                        entity_player.direction = EntityDirection::Left;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    let in_window = entity_player.check_entity_collision_down(game_window_size_y);
                    if in_window == 1 {
                        entity_player.direction = EntityDirection::Down;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    let in_window = entity_player.check_entity_collision_up();
                    if in_window == 1 {
                        entity_player.direction = EntityDirection::Up;
                    }
                }
                _ => {}
            }
        }

        // Update entity direction and draw it in canvas
        entity_player.update_direction(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0,  1_000_000_000 / 60 ));
    }

    println!("\n");
    println!("---- End ----");
}