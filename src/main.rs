use image;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::env;
use std::process;

fn main() {
    let file_names: Vec<String> = env::args().skip(1).collect();
    if file_names.len() < 2 {
        exit_error("Usage: imgrow <file1> <file2> ..".into());
    }

    let spacing = 10;
    let mut total_width = 0;
    let mut max_height = 0;
    let mut images = Vec::new();

    for (i, name) in file_names.iter().enumerate() {
        match image::open(name) {
            Ok(img) => {
                let (w, h) = img.dimensions();
                max_height = std::cmp::max(h, max_height);
                total_width += w;

                if i < file_names.len() - 1 {
                    total_width += spacing;
                }

                images.push(img);
            }
            Err(e) => {
                exit_error(format!("Failed to open '{}': {}", name, e));
            }
        }
    }

    println!("total width: {}", total_width);
    println!("max height: {}", max_height);

    let mut buffer: RgbaImage = ImageBuffer::new(total_width, max_height);
    for x in 0..total_width {
        for y in 0..max_height {
            buffer.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }

    let mut x_off = 0;

    for (i, img) in images.iter().enumerate() {
        let (w, h) = img.dimensions();

        for x in 0..w {
            for y in 0..h {
                let pixel = img.get_pixel(x, y);
                buffer.put_pixel(x + x_off, y, pixel);
            }
        }

        x_off += w;
        if i < images.len() - 1 {
            x_off += spacing;
        }
    }

    match buffer.save("output.png") {
        Ok(_) => {
            println!("Saved to output.png");
        }
        Err(e) => {
            exit_error(format!("Error saving to output.png: {}", e));
        }
    }
}

fn exit_error(msg: String) {
    eprintln!("{}", msg);
    process::exit(1);
}
