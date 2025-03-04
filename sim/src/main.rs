use macroquad::prelude::*;
use colors_transform::{Hsl, Color};
use std::env;
use std::thread;
use std::thread::JoinHandle;
use hole::ball::Ball;
use hole::wall::Wall;

const SCREEN_SIZE: f32 = 600.0;
const FARTHEST_DISTANCE: f32 = 600.0;
const MAX_HUE: f32 = 300.0;
const SATURATION: f32 = 100.0;
const LIGHTNESS: f32 = 50.0;
const MAX_RGB_VALUE: f32 = 255.0;
const NUM_THREADS: usize = SCREEN_SIZE as usize / 100;

#[macroquad::main("Simulate")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Invalid usage! Expected `./exe <number of putts to simulate> <hole file>");
        std::process::exit(1)
    }

    let step: usize = args[1].parse::<usize>().expect("Could not parse step");
    let mut distances: Vec<f32> = vec!();
    let hole = hole::hole::Hole::from_file(args[2].clone());

    let mut threads: Vec<JoinHandle<Vec<f32>>> = vec!();

    let y_step: f32 = SCREEN_SIZE / NUM_THREADS as f32;

    for i in 0..NUM_THREADS {
        let start_y = i as f32 * y_step;
        let stop_y = (i + 1) as f32 * y_step;

        let mut walls: Vec<Wall> = vec!();
        for i in 0..hole.walls.len() {
            walls.push(hole.walls[i]); // copy walls so it is thread safe. This happening NUM_THREADS times is ok
        }
            
        threads.push(thread::spawn(move || {
            let mut x: f32 = 0.0; // start shooting at x=0, y=start_y
            let mut y: f32 = start_y;
            let mut i: usize = 0;

            let rows: f32 = (stop_y - start_y) / step as f32;
            let putts: usize = ((SCREEN_SIZE * rows) / step as f32) as usize;
            let mut distances: Vec<f32> = vec![f32::MAX;  putts];
            
            while y < stop_y {
                let mut ball: Ball = Ball::new(hole.ball.pos.x, hole.ball.pos.y, hole.ball.radius);
                ball.hit(x, y);

                loop {
                    if !ball.is_moving() { break; }
                    ball.move_ball_and_collide(&walls);
                    if ball.pos.distance(hole.cup.pos) < hole.cup.radius && ball.is_able_to_fall_in_hole() {
                        distances[i] = 0.0;
                        break;
                    }
                }

                if distances[i] != 0.0 { //kind of bad code here, there should be a better way to handle this
                    distances[i] = ball.pos.distance(hole.cup.pos);
                }

                x = x + step as f32;
                if x >= SCREEN_SIZE {
                    x = x - SCREEN_SIZE;
                    y = y + step as f32;
                }
                i += 1;
             }

            distances
        }));
    }

    println!("All threads started");

    for _ in 0..NUM_THREADS {
        let cur_thread = threads.remove(0);
        distances.append(&mut cur_thread.join().expect("Thread has panicked!"));
    }

    println!("All threads finished");
    println!("Data Points: {}", distances.len());

    request_new_screen_size(SCREEN_SIZE, SCREEN_SIZE);
    let mut print = true;
    loop {
        clear_background(BLACK);
        let mut x = 0.0;
        let mut y = 0.0;
        for i in 0..distances.len() {
            if distances[i] == 0.0 {
                draw_rectangle(x, y, step as f32, step as f32, WHITE);
            }
            else { // could preCompute colors to save cycles on viewing
                let mut hue: f32 = (distances[i] / FARTHEST_DISTANCE) * MAX_HUE;
                if hue > MAX_HUE {hue = MAX_HUE;}
                let rgb = Hsl::from(hue, SATURATION, LIGHTNESS);
                let color = macroquad::color::Color::new(rgb.get_red() / MAX_RGB_VALUE, rgb.get_green() / MAX_RGB_VALUE, rgb.get_blue() / MAX_RGB_VALUE, 1.0);
                draw_rectangle(x, y, step as f32, step as f32, color);
            }

            if print { println!("{} {} {} ", y, x, distances[i]); }
            x = x + step as f32;
            if x >= SCREEN_SIZE {
                x = 0.0;
                y = y + step as f32;
            }
        }
        print = false;
        next_frame().await
    }
}
