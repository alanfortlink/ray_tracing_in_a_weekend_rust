mod camera;
mod hit_record;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable_list::HittableList;

use crate::sphere::Sphere;
use crate::vec3::Point3;

fn main() -> std::io::Result<()> {
    // World
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    // Render
    camera.render(&world)?;

    Ok(())
}
