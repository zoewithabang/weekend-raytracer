use std::fs::File;
use std::error::Error;
use std::io::Write;
use crate::colour::{self, Colour};

pub fn run() -> Result<(), Box<dyn Error>> {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let mut buffer = File::create("ppmtest.ppm")?;

    writeln!(buffer, "P3")?;
    writeln!(buffer, "{0} {1}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(buffer, "{}", 255)?;

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("{0} {1}", "\rScanlines remaining: ", j);
        for i in 0..IMAGE_WIDTH {
            let colour = Colour::from(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_WIDTH - 1) as f64),
                0.25
            );

            colour::io_writeln_colour(&mut buffer, colour)?;
        }
    }

    println!();

    Ok(())
}