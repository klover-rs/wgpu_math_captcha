use std::sync::Mutex;

use rand::Rng;

use crate::vertex::Vertex;

#[derive(Debug)]
pub enum Shape {
    Line, // 1 + n^2
    Triangle, // (3^3 -n) /2
    Circle // n^2
}

lazy_static::lazy_static! {
    static ref USED_OFFSETS: Mutex<Vec<[f32; 2]>> = Mutex::new(Vec::new());
}
pub fn clear_offsets() {
    USED_OFFSETS.lock().unwrap().clear();
}


impl Shape {
    pub fn compute(&self, n: i32) -> i32 {
        match self {
            Shape::Line => 1 + n.pow(2),
            Shape::Triangle => (27-n) / 2,
            Shape::Circle => n.pow(2),
        }
    }

    fn pick_offset(&self) -> [f32; 2] {
        let mut rng = rand::thread_rng();
        let radius = match self {
            Shape::Line => 0.2,
            Shape::Triangle => 0.25,
            Shape::Circle => 0.2,
        };

        loop {
            let x = rng.gen_range(-0.8..0.8);
            let y = rng.gen_range(-0.8..0.8);
            let candidate = [x, y];

            let mut overlaps = false;
            for used in USED_OFFSETS.lock().unwrap().iter() {
                let dx = used[0] - candidate[0];
                let dy = used[1] - candidate[1];
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < radius * 2.0 {
                    overlaps = true;
                    break;
                }
            }

            if !overlaps {
                USED_OFFSETS.lock().unwrap().push(candidate);
                return candidate;
            }
        }
    }

    pub fn to_vertices(&self, count: i32) -> Vec<Vertex> {
        let mut verts = Vec::new();
        for _ in 0..count {
            let offset = self.pick_offset();

            verts.extend(match self {
                Shape::Line => vec![
                    Vertex { position: [offset[0] - 0.2, offset[1]], color: [0.0, 0.0, 0.0] },
                    Vertex { position: [offset[0] + 0.2, offset[1]], color: [0.0, 0.0, 0.0] },
                ],
                Shape::Triangle => vec![
                    Vertex { position: [offset[0], offset[1] + 0.2], color: [1.0, 0.0, 0.0] },
                    Vertex { position: [offset[0] - 0.2, offset[1] - 0.2], color: [1.0, 0.0, 0.0] },
                    Vertex { position: [offset[0] + 0.2, offset[1] - 0.2], color: [1.0, 0.0, 0.0] },
                ],
                Shape::Circle => {
                    let mut circle = Vec::new();
                    let segments = 48;
                    let radius = 0.2;

                    for i in 0..segments {
                        let theta1 = (i as f32) / (segments as f32) * std::f32::consts::TAU;
                        let theta2 = ((i + 1) as f32) / (segments as f32) * std::f32::consts::TAU;

                        circle.push(Vertex { position: offset, color: [0.0, 1.0, 0.0] });
                        circle.push(Vertex {
                            position: [offset[0] + radius * theta1.cos(), offset[1] + radius * theta1.sin()],
                            color: [0.0, 1.0, 0.0],
                        });
                        circle.push(Vertex {
                            position: [offset[0] + radius * theta2.cos(), offset[1] + radius * theta2.sin()],
                            color: [0.0, 1.0, 0.0],
                        });

                    }
                    circle

                }
            });
            
        }
        verts
        


    }
}

pub fn random_shape() -> Shape {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..3) {
        0 => Shape::Line,
        1 => Shape::Triangle,
        _ => Shape::Circle,
    }
}

