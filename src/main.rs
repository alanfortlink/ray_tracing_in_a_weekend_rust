mod ray;
mod vec3;

use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::Vec3;

use crate::ray::Ray;

use std::{cmp::max, io::Write};

fn ray_color(ray: &Ray) -> Color {
    let a = 0.5 * ray.dir.unit_vector().y + 1.0;
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = max(1, (image_width as f64 / aspect_ratio) as u32);

    // Camera
    let focal_length = 1.0;
    let real_aspect_ratio = ((image_width as f64) / (image_height as f64)) as u32;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (real_aspect_ratio) as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Horizontal edge
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);

    // Vertical edge
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Deltas
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Upper-left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        std::io::stderr().write_all(format!("Row {}/{}\n", j, image_height).as_bytes())?;
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(&ray);
            println!("{}", color.to_color_string());
        }
    }

    Ok(())
}
