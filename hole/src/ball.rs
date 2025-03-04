use macroquad::prelude::Vec2;
use crate::wall::*;

const CONTACT_DAMPENER: f32 = 0.08;
const FRICTION: f32 = 0.95;
const FALLING_SPEED: f32 = 3.0;

pub struct Ball {
    pub pos: Vec2,
    vel: Vec2,
    pub radius: f32,
    starting_pos: Vec2
}

impl Ball {
    pub fn new(x_start: f32, y_start: f32, r: f32) -> Self {
        Self {
            pos: Vec2::new(x_start, y_start),
            vel: Vec2::new(0.0, 0.0),
            radius: r,
            starting_pos: Vec2::new(x_start, y_start)
        }
    }

    pub fn reset(&mut self) {
        self.pos = self.starting_pos;
        self.stop();
    }
    
    pub fn stop(&mut self) {
        self.vel.x = 0.0;
        self.vel.y = 0.0;
    }
    pub fn is_moving(&mut self) -> bool {
        return self.vel.x != 0.0 || self.vel.y != 0.0;
    }

    pub fn is_able_to_fall_in_hole(&mut self) -> bool {
        return self.vel.length() < FALLING_SPEED;
    }

    pub fn hit(&mut self, x: f32, y: f32) {
        self.vel.x = (x - self.pos.x) * CONTACT_DAMPENER;
        self.vel.y = (y - self.pos.y) * CONTACT_DAMPENER;
    }

    pub fn move_ball_and_collide(&mut self, walls: &Vec<Wall>) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        for w in walls.iter() {
            // Check at current pos and at half step for collision
            if w.distance_to_wall(self.pos) < self.radius || w.distance_to_wall(self.pos + (self.vel / 2.0)) < self.radius {
                match w.wall_type {
                    WallType::HORIZONTAL => {
                        //println!("Collided with horizontal wall");
                        //println!("vx {} vy {}", self.vel.x, self.vel.y);
                        self.vel.y = -self.vel.y;
                        //println!("vx {} vy {}", self.vel.x, self.vel.y);
                        if self.vel.y > 0.0 { self.pos.y = w.top_left.y + WALL_THICKNESS + self.radius; }
                        if self.vel.y < 0.0 { self.pos.y = w.top_left.y - (WALL_THICKNESS + self.radius); }
                    },
                    WallType::VERTICAL => {
                        //println!("Collided with vertical wall");
                        //println!("vx {} vy {}", self.vel.x, self.vel.y);
                        self.vel.x = -self.vel.x;
                        //println!("vx {} vy {}", self.vel.x, self.vel.y);
                        if self.vel.x > 0.0 { self.pos.x = w.top_left.x + WALL_THICKNESS + self.radius; }
                        if self.vel.x < 0.0 { self.pos.x = w.top_left.x - (WALL_THICKNESS + self.radius); }
                    },
                    WallType::SLANTED => {
                    }
                };
            }
        }

        // Slow down
        self.vel.x *= FRICTION;
        self.vel.y *= FRICTION;

        // Don't want to roll forever
        if self.vel.x != 0.0 && self.vel.x.abs() < 0.25 { self.vel.x = 0.0; }
        if self.vel.y != 0.0 && self.vel.y.abs() < 0.25 { self.vel.y = 0.0; }

        //println!("vx {} vy {}", self.vel.x, self.vel.y);
    }

    pub fn to_string(&self) -> String {
        return self.starting_pos.x.to_string() + " " + &self.starting_pos.y.to_string();
    }
}