use num::{Complex, Zero};

pub fn in_mandelbrot(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z: Complex<f64> = Complex::zero();

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}
