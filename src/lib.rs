#![deny(missing_docs)]

//! This crate allows a `dhash` signature to be constructed from an image
//! 
//! Requires the `image` crate
//!
//! A `dhash` is a differential gradient hash that compares the difference in gradient between adjacent pixels, and provides a 64 bit signature of an image.
//!
//!
//! A `dhash` can be used to compare against other images for similarity and is resilient to differences in:
//!
//! * Aspect Ratio
//! * Image Size
//! * Brightness and Contrast
//!
//!
//!
//! Implementation details taken from the [Kind of Like That](http://www.hackerfactor.com/blog/?/archives/529-Kind-of-Like-That.html) blog
//! ## Usage (CLI)
//! 
//! Install this crate:
//! ```bash
//! cargo install dhash
//! ```
//! 
//! Run `dhash <img1>` to print out a `dhash` of the image at path `img1`
//! 
//! ```bash
//! $ dhash test.jpg
//! dhash for test.jpg is `13547707017824698364`
//! ```
//! 
//! Run `dhash <img1> <img2>` to print out a `dhash` of both images and the distance between them (a lower number is closer):
//! 
//! ```bash
//! $ dhash test.jpg other.jpg
//! dhash for test.jpg is `4485936524854165493`
//! dhash for other.jpg is `3337201687795727957`
//! distance is: 11
//! ```
//! 

use image::imageops::{grayscale, resize, FilterType};

use image::{GenericImageView, ImageBuffer};
use image::{Luma, Pixel};

const IMG_SCALE: u32 = 8;

/// Computes the `dhash` value of a given image
///
/// A `dhash` is a signature of an image that can be compared to other images
/// 
/// Requires the `image` crate for loading in the image
/// 
/// # Example
/// 
/// ```no_run
/// # use dhash::get_dhash;
/// # fn main() {
/// let img = image::open("test.jpg").expect("Could not open image");
/// let dhash = get_dhash(&img);
/// # }
/// ```
/// 
pub fn get_dhash<I: GenericImageView + 'static>(img: &I) -> u64 {
    let buffered_image = to_grey_signature_image(img);

    let mut bits: [bool; (IMG_SCALE * IMG_SCALE) as usize] =
        [false; (IMG_SCALE * IMG_SCALE) as usize];

    let mut cur_value = 0;

    for i in 0..IMG_SCALE {
        for j in 0..IMG_SCALE {
            let left_pixel = buffered_image.get_pixel(i, j);
            let right_pixel = buffered_image.get_pixel(i + 1, j);

            bits[cur_value] = left_pixel[0] > right_pixel[0];

            cur_value += 1;
        }
    }

    let mut value = 0;

    for i in 0..bits.len() {
        if bits[i] {
            value += 1 << i;
        }
    }

    return value;
}

/// Converts the image to a `dhash` image
///
/// Returns an image that is a 9x8 grayscale image so the pixels can be used in comparison
/// 
/// Used internally by the `get_dhash` method and is not normally needed to be called directly
/// 
///
/// # Example
/// 
/// ```no_run
/// # use dhash::to_grey_signature_image;
/// # fn main() {
/// let img = image::open("test.jpg").expect("Could not open image");
/// let grey_signature_image = to_grey_signature_image(&img);
/// # }
/// ```
pub fn to_grey_signature_image<I: GenericImageView + 'static>(
    img: &I,
) -> ImageBuffer<
    Luma<<<I as GenericImageView>::Pixel as Pixel>::Subpixel>,
    std::vec::Vec<<<I as GenericImageView>::Pixel as Pixel>::Subpixel>,
> {
    let grey_image = grayscale(img);

    let signature_image = resize(&grey_image, IMG_SCALE + 1, IMG_SCALE, FilterType::Nearest);

    return signature_image;
}

/// Returns the Hamming Distance between two `dhashes`
///
/// The closer this number is to 0, the more similar the images are.  With 0 being an exact match
/// 
/// # Example
/// ```
/// # use dhash::hamming_distance;
/// # fn main() {
/// let distance = hamming_distance(4485936524854165493, 3337201687795727957);
/// assert_eq!(distance, 11);
/// # }
/// ```
pub fn hamming_distance(left: u64, right: u64) -> u32 {
    (left ^ right).count_ones()
}
