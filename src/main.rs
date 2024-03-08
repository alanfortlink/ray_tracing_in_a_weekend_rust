mod vec3;

use std::io::Write;

use crate::vec3::Color;

static IMAGE_WIDTH: u32 = 256;
static IMAGE_HEIGHT: u32 = 256;

fn main() -> std::io::Result<()> {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for i in 0..IMAGE_HEIGHT {
        std::io::stderr().write_all(format!("Row {}/{}\n", i, IMAGE_HEIGHT).as_bytes())?;
        for j in 0..IMAGE_WIDTH {
            let r = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = i as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0 as f64;

            let color = Color::new(r, g, b);
            println!("{}", color.to_color_string());
        }
    }

    Ok(())
}
