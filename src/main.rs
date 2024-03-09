mod camera;
mod hit_record;
mod hittable;
mod hittable_list;
mod interval;
mod lambertial;
mod material;
mod metal;
mod ray;
mod sphere;
mod vec3;

use std::rc::Rc;

use camera::Camera;
use hittable_list::HittableList;
use lambertial::Lambertian;
use metal::Metal;
use vec3::Color;

use crate::sphere::Sphere;
use crate::vec3::Point3;

fn main() -> std::io::Result<()> {
    // World
    let mut world: HittableList = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));

    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

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
