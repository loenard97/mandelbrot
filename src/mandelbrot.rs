#![allow(dead_code)]

use num::{Complex, Zero};

pub fn in_mandelbrot(c: Complex<f64>, limit: u32) -> u32 {
    let mut z: Complex<f64> = Complex::zero();

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }

    0
}

pub fn in_mandelbrot2(c: Complex<f64>, limit: u32) -> u32 {
    let x0 = c.re;
    let y0 = c.im;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut i = 0;

    while x*x + y*y <= 4.0 {
        let xtemp = x*x - y*y + x0;
        y = 2.0 * x * y + y0;
        x = xtemp;
        i += 1;
        if i >= limit {
            return 0;
        }
    }

    i
}

pub fn in_mandelbrot3(c: Complex<f64>, limit: u32) -> u32 {
    let x0 = c.re;
    let y0 = c.im;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 0.0;
    let mut i = 0;

    while x2 + y2 <= 4.0 {
        y = (x + x) * y + y0;
        x = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;
        i += 1;
        if i >= limit {
            return 0;
        }
    }

    i
}
