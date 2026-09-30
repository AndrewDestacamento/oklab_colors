mod oklab;
mod rgb;
use itertools::Itertools;
use oklab::*;
use parking_lot::Mutex;
use rayon::prelude::*;
use rgb::*;
use std::f64::consts;

pub fn main() {
    let start_time = std::time::Instant::now();

    let hues = (0..=7)
        .map(|div| consts::FRAC_PI_4 * div as f64)
        .collect::<Vec<f64>>();
    for hue in hues {
        let color = Oklch {
            l: 2.0 / 3.0,
            c: 1.0 / 6.0,
            h: hue,
            d65_reference_l: false,
        };
        let rgb_color = color.to_srgb_closest();
        println!("Color for {}: {:?}", hue * 180.0 / consts::TAU, rgb_color);
    }

    println!("Endtime: {:#?}", start_time.elapsed());
}
