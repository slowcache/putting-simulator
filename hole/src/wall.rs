use macroquad::prelude::Vec2;

pub const HALF_WALL_THICKNESS: f32 = 2.0;
pub const WALL_THICKNESS: f32 = 4.0;

#[derive(Debug, Copy, Clone)]
pub struct Wall {
    pub top_left: Vec2, // x, y
    pub dimensions: Vec2, // width, height
    pub length: f32,
    pub wall_type: WallType,
    a: Vec2,
    b: Vec2
}

#[derive(Debug, Copy, Clone)]
pub enum WallType {
    HORIZONTAL,
    VERTICAL,
    SLANTED
}

impl Wall {
    pub fn new(top_left: Vec2, bottom_right: Vec2) -> Self {
        let wt: WallType = get_wall_type(top_left, bottom_right);
        match wt {
            WallType::HORIZONTAL => {
                Self {
                    top_left: Vec2::new(top_left.x, top_left.y - HALF_WALL_THICKNESS),
                    dimensions: Vec2::new(bottom_right.x - top_left.x, HALF_WALL_THICKNESS + HALF_WALL_THICKNESS),
                    length: bottom_right.x - top_left.x,
                    wall_type: WallType::HORIZONTAL,
                    a: top_left,
                    b: bottom_right
                }
            },
            WallType::VERTICAL => {
                Self {
                    top_left: Vec2::new(top_left.x - HALF_WALL_THICKNESS, top_left.y),
                    dimensions: Vec2::new(HALF_WALL_THICKNESS + HALF_WALL_THICKNESS, bottom_right.y - top_left.y),
                    length: bottom_right.y - top_left.y,
                    wall_type: WallType::VERTICAL,
                    a: top_left,
                    b: bottom_right
                }
            }
            WallType::SLANTED => {
                Self {
                    top_left: Vec2::new(0.0, 0.0),
                    dimensions: Vec2::new(0.0, 0.0),
                    length: 0.0,
                    wall_type: WallType::SLANTED,
                    a: top_left,
                    b: bottom_right
                }
            }
        }
    }

    // https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
    pub fn distance_to_wall(&self, p: Vec2) -> f32 {
        let in_bounds = match self.wall_type {
            WallType::HORIZONTAL => {
                p.x > self.a.x - WALL_THICKNESS && p.x < self.b.x + WALL_THICKNESS
            }
            WallType::VERTICAL => {
                p.y > self.a.y - WALL_THICKNESS && p.y < self.b.y + WALL_THICKNESS
            }
            WallType::SLANTED => {
                false
            }
        };

        if !in_bounds {
            return f32::MAX;
        }

        let a = (self.b.y - self.a.y) * p.x;
        let b = (self.b.x - self.a.x) * p.y;
        let c = self.b.x * self.a.y;
        let d = self.b.y * self.a.x;

        let numerator = (a - b + c - d).abs();

        let distance = (numerator / self.length) - HALF_WALL_THICKNESS;
        
        distance
    }

    pub fn to_string(&self) -> String {
        match self.wall_type {
            WallType::HORIZONTAL => {
                return "wall ".to_owned() + &self.top_left.x.to_string() + " " + &self.top_left.y.to_string() + " " + &(&self.top_left.x + &self.length).to_string() + " " + &self.top_left.y.to_string();
            },
            WallType::VERTICAL => {
                return "wall ".to_owned() + &self.top_left.x.to_string() + " " + &self.top_left.y.to_string() + " " + &self.top_left.x.to_string() + " " + &(&self.top_left.y + &self.length).to_string();
            }
            WallType::SLANTED => {
                return "".to_string();
            }
        }
    }
}

pub fn get_wall_type(top_left: Vec2, bottom_right: Vec2) -> WallType {
    if top_left.x == bottom_right.x { return WallType::VERTICAL; }
    if top_left.y == bottom_right.y { return WallType::HORIZONTAL; }
    return WallType::SLANTED;
}


