use crate::ray::Ray;
use crate::colour::Colour;
use crate::point3::Point3;
use crate::vec3::Vec3;
use std::fs::File;
use crate::colour;
use std::error::Error;
use std::io::Write;

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let half_b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - (radius * radius);
    let discriminant = (half_b * half_b) - (a * c);

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_colour(ray: Ray) -> Colour {
    let t = hit_sphere(Point3::from(0.0, 0.0, -1.0), 0.5, &ray);

    if t > 0.0 {
        let  n = (ray.at(t) - Vec3::from(0.0, 0.0, -1.0)).unit_vector();

        return Colour::from(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let white = Colour::from(1.0, 1.0, 1.0);
    let blue = Colour::from(0.2, 0.45, 1.0);

    &(&white * (1.0 - t)) + &(&blue * t)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

    let mut buffer = File::create("raytest.ppm")?;

    writeln!(buffer, "P3")?;
    writeln!(buffer, "{0} {1}", image_width, image_height)?;
    writeln!(buffer, "{}", 255)?;

    for j in (0..image_height).rev() {
        print!("{0} {1}", "\rScanlines remaining: ", j);
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let ray = Ray::from(origin, lower_left_corner + (horizontal * u) + (vertical * v) - origin);
            let colour = ray_colour(ray);

            colour::io_writeln_colour(&mut buffer, colour)?;
        }
    }

    println!();

    Ok(())
}