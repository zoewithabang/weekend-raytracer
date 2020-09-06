mod ppmtest;
mod vec3;
mod colour;
mod ray;
mod point3;
mod raytest;

use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hey, hit some buttons!");
    println!("======================");
    println!("1 - PPM Test");
    println!("2 - Ray Test");
    println!("Q - Quit");

    'everything: loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim().to_lowercase().as_str() {
            "1" => ppmtest::run()?,
            "2" => raytest::run()?,
            "q" => (),
            _ => println!("???")
        }

        if choice.trim() == "q" {
            break 'everything;
        }

        println!("Done, next!");
    }

    println!("BOOM");

    Ok(())
}
