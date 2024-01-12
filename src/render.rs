use image::codecs::png::PngEncoder;
use image::ColorType;
use image::ImageEncoder;
use num::Complex;
use std::fs::File;

use super::mandelbrot;

pub fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    limit: usize,
) {
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match mandelbrot::in_mandelbrot(point, limit) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

pub fn color_map(pixels: &[u8]) -> Vec<u8> {
    let mut map = vec![0; 3 * pixels.len()];
    let color_map = colorgrad::plasma();

    for (i, pixel) in pixels.into_iter().enumerate() {
        if *pixel != 0 {
            let colors = color_map.at(*pixel as f64 / 255.0).to_rgba8();
            map[3 * i + 0] = colors[0];
            map[3 * i + 1] = colors[1];
            map[3 * i + 2] = colors[2];
        }
    }

    map
}

pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder.write_image(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Rgb8).unwrap();

    Ok(())
}
