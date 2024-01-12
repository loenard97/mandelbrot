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
    buffer: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    colormap: &str,
    iterations: u32,
) {
    let color_map = match colormap {
        "blues" => colorgrad::blues(),
        "br_bg" => colorgrad::br_bg(),
        "bu_gn" => colorgrad::bu_gn(),
        "bu_pu" => colorgrad::bu_pu(),
        "cividis" => colorgrad::cividis(),
        "cool" => colorgrad::cool(),
        "cubehelix_default" => colorgrad::cubehelix_default(),
        "gn_bu" => colorgrad::gn_bu(),
        "greens" => colorgrad::greens(),
        "greys" => colorgrad::greys(),
        "inferno" => colorgrad::inferno(),
        "magma" => colorgrad::magma(),
        "or_rd" => colorgrad::or_rd(),
        "oranges" => colorgrad::oranges(),
        "pi_yg" => colorgrad::pi_yg(),
        "plasma" => colorgrad::plasma(),
        "pr_gn" => colorgrad::pr_gn(),
        "pu_bu" => colorgrad::pu_bu(),
        "pu_bu_gn" => colorgrad::pu_bu_gn(),
        "pu_or" => colorgrad::pu_or(),
        "pu_rd" => colorgrad::pu_rd(),
        "purples" => colorgrad::purples(),
        "rainbow" => colorgrad::rainbow(),
        "rd_bu" => colorgrad::rd_bu(),
        "rd_gy" => colorgrad::rd_gy(),
        "rd_pu" => colorgrad::rd_pu(),
        "rd_yl_bu" => colorgrad::rd_yl_bu(),
        "rd_yl_gn" => colorgrad::rd_yl_gn(),
        "reds" => colorgrad::reds(),
        "sinebow" => colorgrad::sinebow(),
        "spectral" => colorgrad::spectral(),
        "turbo" => colorgrad::turbo(),
        "viridis" => colorgrad::viridis(),
        "warm" => colorgrad::warm(),
        "yl_gn" => colorgrad::yl_gn(),
        "yl_gn_bu" => colorgrad::yl_gn_bu(),
        "yl_or_br" => colorgrad::yl_or_br(),
        "yl_or_rd" => colorgrad::yl_or_rd(),
        _ => colorgrad::greys(),
    };

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            let point_val = mandelbrot::in_mandelbrot(point, iterations);
            let offset = 3 * (row * bounds.0 + col);

            if point_val != 0 {
                let val = (iterations as f64 - point_val as f64) / iterations as f64;
                let colors = color_map.at(val).to_rgba8();
                buffer[offset] = colors[0];
                buffer[offset + 1] = colors[1];
                buffer[offset + 2] = colors[2];
            } else {
                buffer[offset] = 0;
                buffer[offset + 1] = 0;
                buffer[offset + 2] = 0;
            }
        }
    }
}

pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder
        .write_image(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Rgb8)
        .unwrap();

    Ok(())
}
