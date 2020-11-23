#![allow(clippy::many_single_char_names)]

mod color;

use rand::{prelude::ThreadRng, thread_rng, Rng};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const NB_SAMPLES: u32 = 50;
const SIZE: f64 = 0.000000001;
const MAX_ITER: u32 = 1000;

fn main() {
    let px: f64 = -0.5557506;
    let py: f64 = -0.55560;

    let finished = AtomicU32::new(0);

    let buf: Vec<_> = (0..HEIGHT)
        .into_par_iter()
        .map_init(thread_rng, |rng, y| render_line(rng, y, px, py))
        .inspect(|_| {
            let done = finished.fetch_add(1, Ordering::Relaxed);
            let percentage_finished = (done + 1) as f64 / (HEIGHT as f64) * 100.;
            print!("Progress: {}%\r", percentage_finished as u32);
        })
        .flatten()
        .collect();

    image::save_buffer("fractal.png", &buf, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
}

fn render_line(rng: &mut ThreadRng, line_number: u32, px: f64, py: f64) -> Vec<u8> {
    let line_size = WIDTH * 3;
    let mut line: Vec<u8> = vec![0; line_size as usize];

    for x in 0..WIDTH {
        let (r, g, b) = (0..NB_SAMPLES)
            .map(|_| {
                let nx = SIZE * (((x as f64) + rng.gen_range(0., 1.0)) / (WIDTH as f64)) + px;
                let ny =
                    SIZE * (((line_number as f64) + rng.gen_range(0., 1.0)) / (HEIGHT as f64)) + py;
                let (m_res, m_iter) = mandelbrot_iter(nx, ny);
                paint(m_res, m_iter)
            })
            .fold((0, 0, 0), |acc, x| {
                (acc.0 + x.0 as i32, acc.1 + x.1 as i32, acc.2 + x.2 as i32)
            });

        line[(x * 3) as usize] = ((r as f64) / (NB_SAMPLES as f64)) as u8;
        line[((x * 3) + 1) as usize] = ((g as f64) / (NB_SAMPLES as f64)) as u8;
        line[((x * 3) + 2) as usize] = ((b as f64) / (NB_SAMPLES as f64)) as u8;
    }

    line
}

fn paint(r: f64, n: u32) -> (u8, u8, u8) {
    if r > 4. {
        color::hsl_to_rgb(n as f64 / 800. * r, 1., 0.5)
    } else {
        (255, 255, 255)
    }
}

fn mandelbrot_iter(px: f64, py: f64) -> (f64, u32) {
    let (mut x, mut y, mut xx, mut yy) = (0., 0., 0., 0.);
    let mut xy;

    for i in 0..MAX_ITER {
        xx = x * x;
        yy = y * y;
        xy = x * y;
        if xx + yy > 4. {
            return (xx + yy, i);
        }
        x = xx - yy + px;
        y = 2. * xy + py;
    }

    (xx + yy, MAX_ITER)
}
