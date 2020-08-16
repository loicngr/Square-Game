use crate::entity::{Entity, EntityLocation};
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

#[derive(Debug)]
pub struct GameWindow {
    pub x: i32,
    pub y: i32
}

// Game World structure
#[derive(Debug)]
pub struct GameWorld {
    // Vector of entities
    pub entities: Vec<Entity>,
    pub window: GameWindow
}

impl GameWorld {

    // Get an entity mutable ref by her string id
    pub fn get_entity(&mut self, entity_id: &str) -> &mut Entity {
        let entity = self.entities.iter_mut().find(|entity| entity.id == entity_id).expect("Entity not found");
        entity
    }

    // Get an entity ref by her id
    pub fn get_entity_ref(&self, entity_id: &str) -> &Entity {
        let entity = self.entities.iter().find(|entity| entity.id == entity_id).expect("Entity not found");
        entity
    }

    // Draw and fill entities
    pub fn draw_entities(&self, entities: Vec<&str>, canvas: &mut WindowCanvas) {
        for entity_id in &entities { // Loop over entities
            let entity = self.get_entity_ref(entity_id); // Get entity

            let entity_rect = Rect::new(entity.location.x, entity.location.y, entity.size.width, entity.size.height);
            let entity_color = Color::RGBA(entity.color.r, entity.color.g, entity.color.b, entity.color.a);

            canvas.set_draw_color(entity_color); // Draw entity with its color
            canvas.draw_rect(entity_rect).unwrap(); // Draw rect with entity size
            canvas.fill_rect(entity_rect).unwrap(); // Fill rect with entity color
            canvas.present();
        }
    }
/*
    pub fn draw_entity_random_location(&mut self, entity_id: &str, entity_player_location: &EntityLocation, canvas: &mut WindowCanvas) {
        let entity = self.get_entity_ref(entity_id); // Get entity

        let entity_rect = Rect::new(entity.location.x + 5, entity.location.y + 100, entity.size.width, entity.size.height);
        let entity_color = Color::RGBA(entity.color.r, entity.color.g, entity.color.b, entity.color.a);

        canvas.set_draw_color(entity_color); // Draw entity with its color
        canvas.draw_rect(entity_rect).unwrap(); // Draw rect with entity size
        canvas.fill_rect(entity_rect).unwrap(); // Fill rect with entity color
        canvas.present();
    }*/
}

impl GameWindow {
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn set(&mut self, canvas: &WindowCanvas) {
        let window_border_size = canvas.window().drawable_size();

        self.set_x(window_border_size.0 as i32);
        self.set_y(window_border_size.1 as i32);
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}