mod camera;
mod hit_record;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable_list::HittableList;
use material::Material;
use vec3::Color;

use crate::sphere::Sphere;
use crate::vec3::Point3;

static PI: f64 = 3.1415926535897932385;

fn main() -> std::io::Result<()> {
    // World
    let mut world: HittableList = HittableList::new();

    let ground_material = Material::new_lambertian(Color::new(0.5, 0.5, 0.5));
    world.add_sphere(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in (-11)..11 {
        for b in (-11)..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Material::new_lambertian(albedo)
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>() * 0.5;
                    Material::new_metal(albedo, fuzz)
                } else {
                    // glass
                    Material::new_dielectric(1.5)
                };

                world.add_sphere(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Material::new_dielectric(1.5);
    world.add_sphere(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    world.add_sphere(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add_sphere(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 1200;
    let samples_per_pixel: u32 = 500;
    let max_depth: u32 = 50;
    let vfov: f64 = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        vup,
        defocus_angle,
        focus_dist,
    );

    // Render
    camera.render(&world)?;

    Ok(())
}
