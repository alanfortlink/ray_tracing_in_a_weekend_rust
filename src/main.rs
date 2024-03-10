mod camera;
mod dielectric;
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
use dielectric::Dielectric;
use hittable_list::HittableList;
use lambertial::Lambertian;
use metal::Metal;
use vec3::Color;

use crate::sphere::Sphere;
use crate::vec3::Point3;

static PI: f64 = 3.1415926535897932385;

fn main() -> std::io::Result<()> {
    // World
    let mut world: HittableList = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

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
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
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
    let vfov: f64 = 20.0;
    let look_from = Point3::new(-2.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Point3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        vup,
    );

    // Render
    camera.render(&world)?;

    Ok(())
}
