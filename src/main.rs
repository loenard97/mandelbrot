use clap::Parser;
use num::Complex;
use std::thread;

mod mandelbrot;
mod render;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "home")]
    position: String,
    #[arg(long, default_value_t = 1920)]
    width: u32,
    #[arg(long, default_value_t = 1080)]
    height: u32,
    #[arg(long, default_value_t = 255)]
    iterations: u32,
    #[arg(long, default_value = "rainbow")]
    colormap: String,
    fill: Option<bool>,
}

#[derive(Debug)]
enum PoI {
    Home,
    Flower,
    Island,
    Seahorse,
    Starfish,
    Sun,
    Tendrils,
    Tree,
}

impl PoI {
    fn from_str(s: &str) -> Self {
        match s {
            "home" => Self::Home,
            "flower" => Self::Flower,
            "island" => Self::Island,
            "seahorse" => Self::Seahorse,
            "starfish" => Self::Starfish,
            "sun" => Self::Sun,
            "tendrils" => Self::Tendrils,
            "tree" => Self::Tree,
            _ => Self::Home,
        }
    }

    fn to_complex(self) -> (Complex<f64>, f64) {
        match self {
            PoI::Home => (Complex::new(-0.7, 0.0), 1.5),
            PoI::Flower => (Complex::new(-1.99998588117, 0.0), 0.00000000018),
            PoI::Island => (Complex::new(-1.76877883, -0.00173892), 0.0000003),
            PoI::Seahorse => (Complex::new(-0.7440, -0.1260), 0.005),
            PoI::Starfish => (Complex::new(-1.343333334, 0.0), 0.000001),
            PoI::Sun => (Complex::new(-1.343333334, 0.0), 0.000001),
            PoI::Tendrils => (Complex::new(-0.2262667, 1.1161744), 0.000001),
            PoI::Tree => (Complex::new(-1.343333334, 0.0), 0.000001),
        }
    }
}

fn plot_mandelbrot(args: &Args) {
    let position = PoI::from_str(&args.position);
    let (center_pos, zoom) = position.to_complex();
    let div = args.width as f64 / args.height as f64;
    let upper_left = center_pos - zoom * Complex::new(div, 1.0);
    let lower_right = center_pos + zoom * Complex::new(div, 1.0);
    let bounds = (args.width as usize, args.height as usize);
    let mut pixels = vec![0; 3 * bounds.0 * bounds.1];
    let iterations = args.iterations;

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(3 * bounds.0).collect();
    thread::scope(|s| {
        for (cur_row, band) in bands.into_iter().enumerate() {
            let band_bounds = (bounds.0, 1);
            let band_upper_left =
                render::pixel_to_point(bounds, (0, cur_row), upper_left, lower_right);
            let band_lower_right =
                render::pixel_to_point(bounds, (bounds.0, cur_row), upper_left, lower_right);

            s.spawn(move || {
                render::render(
                    band,
                    band_bounds,
                    band_upper_left,
                    band_lower_right,
                    &args.colormap,
                    iterations,
                );
            });
        }
    });

    let _ = render::write_image("test.png", &pixels, bounds);
}

fn main() {
    let args = Args::parse();
    plot_mandelbrot(&args);
}
