use macroquad::prelude::*;
use hole::constants;
use std::env;


#[macroquad::main("Play")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid usage! Must supply hole file to create hole from");
        std::process::exit(1)
    }
    let mut hole = hole::hole::Hole::from_file(args[1].clone());

    request_new_screen_size(constants::SCREEN_SIZE, constants::SCREEN_SIZE);

    loop {
        clear_background(DARKGREEN);

        if is_key_pressed(KeyCode::Space) {
            let click_loc: (f32, f32) = mouse_position();
            println!("mouse_pos: {:?}", click_loc);
            if !hole.ball.is_moving() {
                hole.ball.hit(click_loc.0, click_loc.1);
            }
        }

        if is_key_pressed(KeyCode::R) {
            hole.ball.reset();
        }

        if hole.ball.is_moving() {
            hole.ball.move_ball_and_collide(&hole.walls);
        }

        if hole.ball.pos.distance(hole.cup.pos) < hole.cup.radius && hole.ball.is_able_to_fall_in_hole() {
            draw_text("It's in the hole!", 20.0, 20.0, 30.0, DARKGRAY);
            hole.ball.stop();
        }

        draw_circle(hole.ball.pos.x, hole.ball.pos.y, hole.ball.radius, WHITE);
        draw_circle(hole.cup.pos.x, hole.cup.pos.y, hole.cup.radius, BLACK);
        for w in hole.walls.iter() {
            draw_rectangle(w.top_left.x, w.top_left.y, w.dimensions.x, w.dimensions.y, DARKBROWN);
        }

        next_frame().await
    }
}
