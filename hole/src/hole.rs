use crate::ball::*;
use crate::wall::*;
use crate::constants::*;

use std::fs::File;
use std::io::Write;
use std::io::LineWriter;
use std::io::{self, BufRead};
use std::path::Path;
use macroquad::prelude::Vec2;

const UNSET_RADIUS: f32 = 0.0;

pub struct Hole {
    pub ball: Ball,
    pub cup: Ball,
    pub walls: Vec<Wall>
}

impl Hole {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(300.0, 500.0, BALL_RADIUS),
            cup: Ball::new(300.0, 100.0, CUP_RADIUS),
            walls: vec!()
        }
    }

    pub fn from_file(filename: String) -> Self {
        let mut ball = Ball::new(0.0, 0.0, UNSET_RADIUS);
        let mut cup = Ball::new(0.0, 0.0, UNSET_RADIUS);
        let mut walls: Vec<Wall> = vec!();

        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.map_while(Result::ok) {
                let trimmed = line.trim();
                if trimmed.len() == 0 {
                    continue;
                }
                if trimmed.starts_with("ball") {
                    let ball_info = trimmed.split(" ").collect::<Vec<&str>>();
                    assert!(ball_info.len() == 3, "Invalid usage! Expected 'ball x y'");
                    let x = ball_info[1].parse::<f32>().expect("Unable to parse x of ball");
                    let y = ball_info[2].parse::<f32>().expect("Unable to parse y of ball");
                    ball = Ball::new(x, y, BALL_RADIUS);
                }
                if trimmed.starts_with("cup") {
                    let cup_info = trimmed.split(" ").collect::<Vec<&str>>();
                    assert!(cup_info.len() == 3, "Invalid usage! Expected 'cup x y'");
                    let x = cup_info[1].parse::<f32>().expect("Unable to parse x of cup");
                    let y = cup_info[2].parse::<f32>().expect("Unable to parse y of cup");
                    cup = Ball::new(x, y, CUP_RADIUS);
                }
                if trimmed.starts_with("wall") {
                    let wall_info = trimmed.split(" ").collect::<Vec<&str>>();
                    assert!(wall_info.len() == 5, "Invalid usage! Expected 'wall x1 y1 x2 y2'");
                    let x1 = wall_info[1].parse::<f32>().expect("Unable to parse x1 of wall");
                    let y1 = wall_info[2].parse::<f32>().expect("Unable to parse y1 of wall");
                    let x2 = wall_info[3].parse::<f32>().expect("Unable to parse x2 of wall");
                    let y2 = wall_info[4].parse::<f32>().expect("Unable to parse y2 of wall");
                    walls.push(Wall::new(Vec2::new(x1, y1), Vec2::new(x2, y2)));
                }
            }
        }

        assert!(ball.radius != UNSET_RADIUS, "ball not defined in file!");
        assert!(cup.radius != UNSET_RADIUS, "cup not defined in file!");

        Self {
            ball: ball,
            cup: cup,
            walls: walls
        }
    }

    pub fn save_to_file(&self, filename: String) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(filename)?;
        let mut file = LineWriter::new(file);
        file.write_all(("ball ".to_owned() + &self.ball.to_string() + "\n").as_bytes())?;
        file.write_all(("cup ".to_owned() + &self.cup.to_string() + "\n").as_bytes())?;
        for w in self.walls.iter() {
            file.write_all((w.to_string() + "\n").as_bytes())?;
        }
        Ok(())
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_collision_on_100_180() {
        let mut h = Hole::from_file("res/test.hole".to_string());

        h.ball.hit(100.0, 180.0);

        while h.ball.is_moving() {
            h.ball.move_ball_and_collide(&h.walls);
        }

        assert!(h.ball.pos.distance(h.cup.pos) < 180.0, "Ball did not collide with wall properly! Distance={}", h.ball.pos.distance(h.cup.pos));
    }

    #[test]
    fn test_wall_collision_on_corner() {
        let mut h = Hole::from_file("res/test.hole".to_string());

        h.ball.hit(160.0, 180.0);

        let mut counter: usize = 0;
        while h.ball.is_moving() {
            counter += 1;
            println!("Frame {}", counter);
            h.ball.move_ball_and_collide(&h.walls);
        }

        assert!(h.ball.pos.distance(h.cup.pos) < 200.0, "Ball did not collide with wall properly! Distance={}", h.ball.pos.distance(h.cup.pos));
    }

    #[test]
    fn test_wall_phasing() {
        let mut h = Hole::from_file("res/test.hole".to_string());

        h.ball.hit(85.0, 10.0);

        let mut counter: usize = 0;
        while h.ball.is_moving() {
            counter += 1;
            println!("Frame {}", counter);
            h.ball.move_ball_and_collide(&h.walls);
        }

        assert!(h.ball.pos.distance(h.cup.pos) < 400.0, "Ball did not collide with wall properly! Distance={}", h.ball.pos.distance(h.cup.pos));
    }
}