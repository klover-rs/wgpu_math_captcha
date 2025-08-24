use core::num;
use std::{io::{self, BufRead}, thread};

use rand::Rng;

use crate::shape::Shape;
mod window;
mod shape;
mod wgpu;
mod vertex;
fn main() {
    
    let mut rng = rand::thread_rng();

    let puzzle = vec![
        (Shape::Line, rng.gen_range(1..8)),
        (Shape::Triangle, rng.gen_range(1..8)),
        (Shape::Circle, rng.gen_range(1..8)),
    ];

    // show puzzle
    println!("CAPTCHA DEMO");
    let mut code = String::new();
    for (shape, number) in &puzzle {
        let value = shape.compute(*number);
        println!("{:?} with {}", shape, number);
        code.push_str(&value.to_string());

    }

    println!("Hidden Code: {}", code);

    let hidden_code_clone = code.clone();

    let (tx, rx) = std::sync::mpsc::channel::<String>();
    thread::spawn(move || {
        println!("enter the code: ");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(input) = line {
                if input.trim() == hidden_code_clone {
                    println!("✅ Captcha solved!");
                    let _ = tx.send("SOLVED".into()); // notify window
                } else {
                    println!("❌ Wrong code: {}", input);
                }
            }
        }
    });

    // right here we need to prompt the input in the terminal

    window::run(puzzle, rx);
}
