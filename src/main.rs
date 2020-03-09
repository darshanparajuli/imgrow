use clap;
use image;

use clap::{App, Arg};
use image::imageops::FilterType;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::process;

const DEFAULT_SPACING: u32 = 10;
const DEFAULT_MAX_HEIGHT: i32 = -1;

fn main() {
    let config = get_config();

    let spacing = config.spacing;
    let mut total_width = 0;
    let mut max_height = 0;
    let mut images = Vec::new();

    for (i, name) in config.input.iter().enumerate() {
        match image::open(name) {
            Ok(img) => {
                let (w, h) = img.dimensions();
                max_height = std::cmp::max(h, max_height);
                total_width += w;

                if i < config.input.len() - 1 {
                    total_width += spacing;
                }

                images.push(img);
            }
            Err(e) => {
                exit_error(format!("Failed to open '{}': {}", name, e));
            }
        }
    }

    println!("Width: {}", total_width);
    println!("Height: {}", max_height);

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

    if config.max_height != DEFAULT_MAX_HEIGHT {
        let aspect_ratio = (config.max_height as f32) / (max_height as f32);
        let new_width = (total_width as f32 * aspect_ratio as f32).round() as u32;
        buffer = image::imageops::resize(
            &buffer,
            new_width,
            config.max_height as u32,
            FilterType::Lanczos3,
        );
    }

    match buffer.save(&config.output) {
        Ok(_) => {
            println!("Saved to '{}'.", config.output);
        }
        Err(e) => {
            exit_error(format!("Error saving to '{}': {}", config.output, e));
        }
    }
}

struct Config {
    input: Vec<String>,
    output: String,
    spacing: u32,
    max_height: i32,
}

fn get_config() -> Config {
    let matches = App::new("imgrow")
        .version("0.1.0")
        .author("Darshan Parajuli <parajulidarshan@gmail.com>")
        .about("Put a bunch of images in a row.")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets output file.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("spacing")
                .short("s")
                .long("spacing")
                .help("Number of pixels in between images.")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("max_height")
                .long("height")
                .help("Maximum height.")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("images")
                .long("images")
                .short("i")
                .value_name("IMAGE")
                .multiple(true)
                .required(true)
                .help("Images."),
        )
        .get_matches();

    let spacing = matches
        .value_of("spacing")
        .unwrap_or(&DEFAULT_SPACING.to_string())
        .parse::<u32>();
    if spacing.is_err() {
        exit_error("Invalid spacing value!".into());
    }
    let spacing = spacing.unwrap();

    let mut max_height = DEFAULT_MAX_HEIGHT;
    if let Some(h) = matches.value_of("max_height") {
        match h.parse::<i32>() {
            Ok(h) => {
                if h <= 0 {
                    exit_error("Height must be greater than 0!".into());
                }
                max_height = h;
            }
            Err(_) => {
                exit_error("Invalid height value!".into());
            }
        }
    }

    Config {
        input: matches
            .values_of("images")
            .unwrap()
            .map(|s| s.to_string())
            .collect(),
        output: matches.value_of("output").unwrap().to_string(),
        spacing,
        max_height,
    }
}

fn exit_error(msg: String) {
    eprintln!("{}", msg);
    process::exit(1);
}
