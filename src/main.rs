extern crate sdl2;

mod core;
mod entity;

use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::entity::EntityDirection;
use crate::core::{GameWorld, GameWindow};
use std::borrow::{Borrow, BorrowMut};
use rand::Rng;
use sdl2::render::WindowCanvas;

const COLOR_BACKGROUND: Color = Color::RGBA(0, 0, 0, 1);
const PLAYER_ENTITY_ID: &str = "entity_1";
const WINDOW_TITLE: &str = "Square game";

fn update_window_title(canvas: &mut WindowCanvas, player_point: i32) {
    canvas.window_mut().set_title(format!("{} - {}", WINDOW_TITLE, player_point).as_str()).unwrap();
}

// Main function
fn main() {
    println!("---- Start ----");
    println!("\n");

        let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(WINDOW_TITLE, 800, 600)
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
    game_world.draw_entities(vec![PLAYER_ENTITY_ID, "entity_2"], &mut canvas);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player_point = 0;
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


        let entity_player_ref = game_world.get_entity_ref(PLAYER_ENTITY_ID);
        let entity_enemy_ref = game_world.get_entity_ref("entity_2");
        let entity_collision = entity_player_ref.check_collision_entity(&entity_enemy_ref);

        if entity_collision {
            let entity_player_location = entity_player_ref.location.clone();
            let entity_enemy = game_world.get_entity("entity_2");
            let entity_old_location = entity_enemy.location.generate_location(entity_player_location,
                                                                              game_window_size_x,
                                                                              game_window_size_y,
                                                                                entity_enemy.movement_scale
                                                                                        );
            entity_enemy.update_last_location(entity_old_location[0], entity_old_location[1]);
            entity_enemy.draw_entity(&mut canvas);

            let entity_player = game_world.get_entity(PLAYER_ENTITY_ID);
            entity_player.draw_entity(&mut canvas);

            player_point = (player_point + 1);
            update_window_title(&mut canvas, player_point);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0,  1_000_000_000 / 60 ));
    }

    println!("\n");
    println!("---- End ----");
}