use macroquad::prelude::*;
use hole::hole::Hole;
use hole::ball::Ball;
use hole::wall::Wall;
use hole::constants;
use std::env;

const INDICATOR_OFFSET: f32 = 5.0;

#[macroquad::main("Edit")]
async fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let mut hole = Hole::new();
    if args.len() == 2 {
        hole = Hole::from_file(args[1].clone());
    }

    request_new_screen_size(constants::SCREEN_SIZE, constants::SCREEN_SIZE);

    let mut wall_start: Vec2 = Vec2::new(0.0, 0.0);

    loop {
        clear_background(DARKGREEN);

        if is_key_pressed(KeyCode::B) {
            let click_loc: (f32, f32) = mouse_position();
            hole.ball = Ball::new(click_loc.0, click_loc.1, constants::BALL_RADIUS);
        }

        if is_key_pressed(KeyCode::C) {
            let click_loc: (f32, f32) = mouse_position();
            hole.cup = Ball::new(click_loc.0, click_loc.1, constants::CUP_RADIUS);
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
                if wall_start.y <= wall_end.y { // Anchor is above the current pos
                    hole.walls.push(Wall::new(wall_start, wall_end));
                } else {
                    hole.walls.push(Wall::new(wall_end, wall_start));
                }
            } else {
                // Make Vertical
                let wall_end = Vec2::new(click_loc.0, wall_start.y);
                if wall_start.x <= wall_end.x { // Anchor is left of the current pos
                    hole.walls.push(Wall::new(wall_start, wall_end));
                } else {
                    hole.walls.push(Wall::new(wall_end, wall_start));
                }
            }
        }

        if is_key_pressed(KeyCode::S) {
            let _ = hole.save_to_file("out.hole".to_string());
        }

        // Draw Anchor
        draw_rectangle(wall_start.x - INDICATOR_OFFSET, wall_start.y, INDICATOR_OFFSET * 2.0, 2.0, GRAY);
        draw_rectangle(wall_start.x, wall_start.y - INDICATOR_OFFSET, 2.0, INDICATOR_OFFSET * 2.0, GRAY);

        draw_circle(hole.ball.pos.x, hole.ball.pos.y, hole.ball.radius, WHITE);
        draw_circle(hole.cup.pos.x, hole.cup.pos.y, hole.cup.radius, BLACK);
        for w in hole.walls.iter() {
            draw_rectangle(w.top_left.x, w.top_left.y, w.dimensions.x, w.dimensions.y, DARKBROWN);
        }

        next_frame().await
    }
}
