use crate::vec3::Vec3;
use std::io;

pub type Colour = Vec3;

pub fn io_writeln_colour(writer: &mut dyn io::Write, colour: Colour) -> io::Result<()> {
    writeln!(
        writer,
        "{0} {1} {2}",
        (255.999 * colour.x()) as i32,
        (255.999 * colour.y()) as i32,
        (255.999 * colour.z()) as i32
    )
}