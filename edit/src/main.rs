use macroquad::prelude::*;
use hole::hole::Hole;
use hole::ball::Ball;
use hole::wall::Wall;
use hole::hole::BALL_RADIUS;
use hole::hole::CUP_RADIUS;
use std::env;

use std::fs::{File};
use std::io::prelude::*;
use std::io::LineWriter;

#[macroquad::main("Edit")]
async fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let mut hole = Hole::new();
    if args.len() == 2 {
        hole = Hole::from_file(args[1].clone());
    }

    request_new_screen_size(600.0, 600.0);

    let mut wall_start: Vec2 = Vec2::new(0.0, 0.0);

    loop {
        clear_background(DARKGREEN);

        if is_key_pressed(KeyCode::B) {
            let click_loc: (f32, f32) = mouse_position();
            hole.ball = Ball::new(click_loc.0, click_loc.1, BALL_RADIUS);
        }

        if is_key_pressed(KeyCode::C) {
            let click_loc: (f32, f32) = mouse_position();
            hole.cup = Ball::new(click_loc.0, click_loc.1, CUP_RADIUS);
        }

        if is_key_pressed(KeyCode::W) {
            let click_loc: (f32, f32) = mouse_position();
            wall_start.x = click_loc.0;
            wall_start.y = click_loc.1;
        }
        if is_key_pressed(KeyCode::E) {
            let click_loc: (f32, f32) = mouse_position();
            let x_length = (click_loc.0 - wall_start.x).abs();
            let y_length = (click_loc.1 - wall_start.y).abs();
            if x_length <= y_length {
                // Make Horizontal
                let wall_end = Vec2::new(wall_start.x, click_loc.1);
                if wall_start.y <= wall_end.y {
                    hole.walls.push(Wall::new(wall_start, wall_end));
                } else {
                    hole.walls.push(Wall::new(wall_end, wall_start));
                }
            } else {
                // Make Vertical
                let wall_end = Vec2::new(click_loc.0, wall_start.y);
                if wall_start.x <= wall_end.x {
                    hole.walls.push(Wall::new(wall_start, wall_end));
                } else {
                    hole.walls.push(Wall::new(wall_end, wall_start));
                }
            }
        }

        if is_key_pressed(KeyCode::S) {
            let file = File::create("out.hole")?;
            let mut file = LineWriter::new(file);
            file.write_all(("ball ".to_owned() + &hole.ball.to_string() + "\n").as_bytes())?;
            file.write_all(("cup ".to_owned() + &hole.cup.to_string() + "\n").as_bytes())?;
            for w in hole.walls.iter() {
                file.write_all((w.to_string() + "\n").as_bytes())?;
            }
        }

        draw_circle(hole.ball.pos.x, hole.ball.pos.y, hole.ball.radius, WHITE);
        draw_circle(hole.cup.pos.x, hole.cup.pos.y, hole.cup.radius, BLACK);
        for w in hole.walls.iter() {
            draw_rectangle(w.top_left.x, w.top_left.y, w.dimensions.x, w.dimensions.y, DARKBROWN);
        }

        next_frame().await
    }
}
