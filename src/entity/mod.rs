use sdl2::render::WindowCanvas;
use crate::{COLOR_BACKGROUND};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use rand::Rng;

// Entity direction enumeration
#[derive(Debug)]
pub enum EntityDirection {
    Idle,
    Up,
    Down,
    Left,
    Right
}

// Entity location structure
#[derive(Debug, Clone)]
pub struct EntityLocation {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct EntitySize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct EntityColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// Entity structure
#[derive(Debug)]
pub struct Entity {
    pub id: &'static str,
    pub location: EntityLocation,
    pub last_location: EntityLocation,
    pub direction: EntityDirection,
    pub size: EntitySize,
    pub color: EntityColor,
    pub movement_scale: i32
}

impl EntityLocation {

    // Check if entity is location equal to other entity
    pub fn equal_to(&self, entity: &Entity) -> bool {
        let is_equal =
            if entity.location.x == self.x && entity.location.y == self.y {
                true
            } else {
                false
            };
        is_equal
    }

    pub fn generate_location(&mut self, player_position: EntityLocation, screen_width: i32, screen_height: i32, entity_scale: i32) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let old_location = vec![self.x, self.y]; // Save location before update it

        loop {
            let generated_x = rng.gen_range(0, screen_width - entity_scale); // Generate number in x
            let generated_y = rng.gen_range(0, screen_height - entity_scale); // Generate number in y

            // If x and y number is in the grid
            if generated_x % entity_scale == 0 && generated_y % entity_scale == 0 {
                // If is not a player_entity location
                if generated_x != player_position.x || generated_y != player_position.y {
                    self.x = generated_x;
                    self.y = generated_y;
                    break;
                }
            }
        }

        old_location
    }
}

impl Entity {

    // Update entity location 2d vector (x, y)
    pub fn update_location(&mut self, x: i32, y: i32) {
        self.location.x = x;
        self.location.y = y;
    }

    pub fn update_last_location(&mut self, x: i32, y: i32) {
        self.last_location.x = x;
        self.last_location.y = y;
    }

    // Draw entity in Canvas
    pub(crate) fn draw_entity(&self, canvas: &mut WindowCanvas) {
        let entity_rect = Rect::new(self.location.x, self.location.y, self.size.width, self.size.height);
        let entity_color = Color::RGBA(self.color.r, self.color.g, self.color.b, self.color.a);

        canvas.set_draw_color(entity_color); // Draw entity with its color
        canvas.draw_rect(entity_rect).unwrap(); // Draw rect with entity size
        canvas.fill_rect(entity_rect).unwrap(); // Fill rect with entity color
        canvas.present();

        // Remove old entity draw square
        let old_entity_rect = Rect::new(self.last_location.x, self.last_location.y, self.size.width, self.size.height);
        canvas.set_draw_color(COLOR_BACKGROUND);
        canvas.draw_rect(old_entity_rect).unwrap();
        canvas.fill_rect(old_entity_rect).unwrap();
        canvas.present();
    }

    // Update entity direction
    pub fn update_direction(&mut self, canvas: &mut WindowCanvas) {
        self.last_location.x = self.location.x;
        self.last_location.y = self.location.y;

        match self.direction {
            EntityDirection::Right => {
                self.update_location(self.location.x + self.movement_scale, self.location.y);
                self.draw_entity(canvas);
            },
            EntityDirection::Left => {
                self.update_location(self.location.x - self.movement_scale, self.location.y);
                self.draw_entity(canvas);
            },
            EntityDirection::Down => {
                self.update_location(self.location.x, self.location.y + self.movement_scale);
                self.draw_entity(canvas);
            },
            EntityDirection::Up => {
                self.update_location(self.location.x, self.location.y - self.movement_scale);
                self.draw_entity(canvas);
            }
            _ => {}
        }
    }

    // TODO : regroup/optimize collision functions

    pub fn check_entity_collision_right(&self, window_x: i32) -> i32 {
        let entity_moved_location = self.location.x + self.movement_scale;

        let in_window =
            if entity_moved_location >= window_x {
                0
            } else {
                1
            };
        in_window
    }

    pub fn check_entity_collision_left(&self) -> i32 {
        let entity_moved_location = self.location.x - self.movement_scale;

        let in_window =
            if entity_moved_location < 0 {
                0
            } else {
                1
            };
        in_window
    }

    pub fn check_entity_collision_up(&self) -> i32 {
        let entity_moved_location = self.location.y - self.movement_scale;

        let in_window =
            if entity_moved_location < 0 {
                0
            } else {
                1
            };
        in_window
    }

    pub  fn check_entity_collision_down(&self, window_y: i32) -> i32 {
        let entity_moved_location = self.location.y + self.movement_scale;

        let in_window =
            if entity_moved_location >= window_y {
                0
            } else {
                1
            };
        in_window
    }

    pub fn check_collision_entity(&self, entity: &Entity) -> bool {
        self.location.equal_to(entity)
    }
}

// Function to create my entities
// Return a vector with entities
pub fn create_entities() -> Vec<Entity> {
    // My entity 1
    let entity_one = Entity {
        id: "entity_1",
        location: EntityLocation {
            x: 0,
            y: 0
        },
        last_location: EntityLocation {
            x: 0,
            y: 0
        },
        direction: EntityDirection::Idle,
        size: EntitySize {
            width: 50,
            height: 50
        },
        color: EntityColor {
            r: 0,
            g: 100,
            b: 255,
            a: 1
        },
        movement_scale: 50,
    };

    // My entity 2
    let entity_two = Entity {
        id: "entity_2",
        location: EntityLocation {
            x: 100,
            y: 200
        },
        last_location: EntityLocation {
            x: 0,
            y: 0
        },
        direction: EntityDirection::Idle,
        size: EntitySize {
            width: 50,
            height: 50
        },
        color: EntityColor {
            r: 0,
            g: 200,
            b: 255,
            a: 1
        },
        movement_scale: 50,
    };

    vec![entity_one, entity_two]
}