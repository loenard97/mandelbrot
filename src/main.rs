use clap::Parser;
use num::Complex;
use std::thread;

mod mandelbrot;
mod render;

#[derive(Parser, Debug)]
struct Args {
    position: String,
    #[arg(short, long, default_value_t = 1_000)]
    pixel: usize,
    #[arg(short, long, default_value_t = 100)]
    threads: usize,
    #[arg(short, long, default_value_t = 255)]
    iterations: usize,
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

    fn to_complex(self) -> (Complex<f64>, Complex<f64>) {
        match self {
            PoI::Home => (Complex::new(-3.0, 2.0), Complex::new(1.0, -2.0)),
            PoI::Flower => (
                Complex::new(-1.999985882, 0.000000001),
                Complex::new(-1.999985880, -0.000000001),
            ),
            PoI::Island => (
                Complex::new(-1.768780, -0.001740),
                Complex::new(-1.768778, -0.001738),
            ),
            PoI::Seahorse => (
                Complex::new(-1.343333334, 0.0),
                Complex::new(-1.343333333, -0.000000001),
            ),
            PoI::Starfish => (
                Complex::new(-1.343333334, 0.0),
                Complex::new(-1.343333333, -0.000000001),
            ),
            PoI::Sun => (
                Complex::new(-1.343333334, 0.0),
                Complex::new(-1.343333333, -0.000000001),
            ),
            PoI::Tendrils => (
                Complex::new(-1.343333334, 0.0),
                Complex::new(-1.343333333, -0.000000001),
            ),
            PoI::Tree => (
                Complex::new(-1.343333334, 0.0),
                Complex::new(-1.343333333, -0.000000001),
            ),
        }
    }
}

fn plot_mandelbrot(args: &Args) {
    let position = PoI::from_str(&args.position);
    let (upper_left, lower_right) = position.to_complex();

    let bounds = (args.pixel, args.pixel);
    let mut pixels = vec![0; bounds.0 * bounds.1];
    let threads = args.threads;
    let rows_per_band = bounds.1 / threads + 1;
    let iterations = args.iterations;

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
    thread::scope(|s| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = render::pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                render::pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            s.spawn(move || {
                render::render(band, band_bounds, band_upper_left, band_lower_right, iterations);
            });
        }
    });

    let map = render::color_map(&pixels);
    let _ = render::write_image("test.png", &map, bounds);

}

fn main() {
    let args = Args::parse();
    plot_mandelbrot(&args);
}
