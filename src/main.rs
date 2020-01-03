extern crate imageproc;
use image::{open, GrayImage, Luma};
use std::{env, fs, path::Path};
use imageproc::contrast::{otsu_level, threshold};

fn otsu(input: &GrayImage) -> u8 {
    100
}
// Based on the imageproc otsu implementation.
fn otsu_multi_level(image: &GrayImage, levels: usize) -> Vec<u8> {
    let hist = histogram(image);
    let (width, height) = image.dimensions();
    let total_weight = width * height;

    // Sum of all pixel intensities, to use when calculating means.
    let total_pixel_sum = hist.channels[0]
        .iter()
        .enumerate()
        .fold(0f64, |sum, (t, h)| sum + (t as u32 * h) as f64);

    // Sum of all pixel intensities in the background class.
    let mut background_pixel_sum = 0f64;

    // The weight of a class (background or foreground) is
    // the number of pixels which belong to that class at
    // the current threshold.
    let mut background_weight = 0u32;
    let mut foreground_weight;

    let mut largest_variance = 0f64;
    let mut best_thresholds = Vec::with_capacity(levels);

    for (threshold, hist_count) in hist.channels[0].iter().enumerate() {
        background_weight += hist_count;
        if background_weight == 0 {
            continue;
        };

        foreground_weight = total_weight - background_weight;
        if foreground_weight == 0 {
            break;
        };

        background_pixel_sum += (threshold as u32 * hist_count) as f64;
        let foreground_pixel_sum = total_pixel_sum - background_pixel_sum;

        let background_mean = background_pixel_sum / (background_weight as f64);
        let foreground_mean = foreground_pixel_sum / (foreground_weight as f64);

        let mean_diff_squared = (background_mean - foreground_mean).powi(2);
        let intra_class_variance =
            (background_weight as f64) * (foreground_weight as f64) * mean_diff_squared;

        if intra_class_variance > largest_variance {
            largest_variance = intra_class_variance;
            best_threshold = threshold as u8;
        }
    }

    best_threshold
}

fn main() {
    if env::args().len() != 3 {
        panic!("Please enter an input file and a target directory")
    }

    let input_path = env::args().nth(1).unwrap();
    let output_dir = env::args().nth(2).unwrap();

    let input_path = Path::new(&input_path);
    let output_dir = Path::new(&output_dir);

    if !output_dir.is_dir() {
        fs::create_dir(output_dir).expect("Failed to create output directory")
    }

    if !input_path.is_file() {
        panic!("Input file does not exist");
    }

    // Load image and convert to grayscale
    let input_image = open(input_path)
        .expect(&format!("Could not load image at {:?}", input_path))
        .to_luma();
}

