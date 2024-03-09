use crate::{
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

use std::{cmp::max, io::Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
}

static PI: f64 = 3.1415926535897932385;

static EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
static UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let hit_record_option = world.hit(r, &Interval::new(0.0, f64::INFINITY));
    if let Some(hit_record) = hit_record_option {
        return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0));
    }

    let a = 0.5 * r.dir.unit_vector().y + 1.0;
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self {
        let image_height: u32 = max(1, (image_width as f64 / aspect_ratio) as u32);

        // Camera
        let focal_length = 1.0;
        let real_aspect_ratio = (image_width as f64) / (image_height as f64);
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
        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        }
    }

    pub fn render(&self, world: &HittableList) -> std::io::Result<()> {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            std::io::stderr().write_all(format!("Row {}/{}\n", j, self.image_height).as_bytes())?;
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, world);
                }
                println!("{}", pixel_color.to_color_string(self.samples_per_pixel));
            }
        }

        Ok(())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand::random::<f64>();
        let py = -0.5 + rand::random::<f64>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
