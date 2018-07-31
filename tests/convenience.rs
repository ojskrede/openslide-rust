//! Integration tests of the convenience module
//!

extern crate openslide;
extern crate failure;
extern crate float_cmp;
extern crate image;

use std::path::Path;
use float_cmp::ApproxEq;
use image::{RgbaImage};

fn get_slide() -> openslide::OpenSlide {
    let filename = Path::new("assets/CMU-1-Small-Region.svs");
    let os = match openslide::OpenSlide::new(&filename) {
        Ok(val) => val,
        Err(msg) => {
            assert!(false, format!("Unable to load slide:\n{}", msg));
            openslide::OpenSlide::new(&Path::new("")).unwrap() // For compilation
        },
    };

    os
}

#[test]
fn test_level_count() {
    let slide = get_slide();
    let value = match slide.get_level_count() {
        Ok(val) => val,
        Err(msg) => {
            assert!(false, format!("Level count error:\n{}", msg));
            0 // For compilation
        },
    };
    assert_eq!(value, 1)
}

#[test]
fn test_level0_dimensions() {
    let slide = get_slide();
    let value = match slide.get_level0_dimensions() {
        Ok(val) => val,
        Err(msg) => {
            assert!(false, format!("Level 0 dimension error:\n{}", msg));
            (0, 0) // For compilation
        },
    };
    assert_eq!(value, (2220, 2967))
}

#[test]
fn test_level_dimensions() {
    let slide = get_slide();
    let value = match slide.get_level_dimensions(0u8) {
        Ok(val) => val,
        Err(msg) => {
            assert!(false, format!("Level dimension error:\n{}", msg));
            (0, 0) // For compilation
        },
    };
    assert_eq!(value, (2220, 2967))
}

#[test]
fn test_level_downsample() {
    let slide = get_slide();
    let value = match slide.get_level_downsample(0u8) {
        Ok(val) => val,
        Err(msg) => {
            assert!(false, format!("Level dimension error:\n{}", msg));
            0.0 // For compilation
        },
    };
    assert!(value.approx_eq(&1.0, ::std::f64::EPSILON, 2))
}

#[test]
fn test_best_level_for_downsample() {
    let slide = get_slide();
    let value = match slide.get_best_level_for_downsample(2.5) {
        Ok(val) => val,
        Err(msg) => {
            assert!(false, format!("Level dimension error:\n{}", msg));
            1 // For compilation
        },
    };
    assert_eq!(0, value)
}

#[test]
fn test_read_region() {
    let slide = get_slide();
    let value = match slide.read_region(1510u32, 1510u32, 0u32, 4u32, 4u32) {
        Ok(val) => val,
        Err(msg) => {
            assert!(false, format!("Level dimension error:\n{}", msg));
            RgbaImage::new(0, 0) // For compilation
        },
    };
    let true_value = vec![152, 123, 172, 255, 148, 122, 171, 255,
                          137, 123, 167, 255, 167, 158, 175, 255,
                          179, 185, 205, 255, 183, 197, 213, 255,
                          198, 210, 221, 255, 224, 226, 224, 255,
                          247, 249, 246, 255, 248, 255, 247, 255,
                          255, 249, 243, 255, 236, 255, 255, 255,
                          249, 239, 250, 255, 250, 246, 252, 255,
                          254, 245, 241, 255, 246, 246, 246, 255];
    assert_eq!(true_value, value.into_vec())
}
