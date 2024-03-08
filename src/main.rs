mod vec3;

use std::io::Write;

static IMAGE_WIDTH: u32 = 256;
static IMAGE_HEIGHT: u32 = 256;

fn main() -> std::io::Result<()> {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for i in 0..IMAGE_HEIGHT {
        std::io::stderr().write_all(format!("Row {}/{}\n", i, IMAGE_HEIGHT).as_bytes())?;
        for j in 0..IMAGE_WIDTH {
            let r = j as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = i as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0 as f32;

            let ir = (r * 255.999) as u32;
            let ig = (g * 255.999) as u32;
            let ib = (b * 255.999) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    Ok(())
}
