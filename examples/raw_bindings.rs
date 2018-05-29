//! Example of how to use the raw bindings
//!

extern crate failure;
extern crate openslide;

use std::path::Path;
use failure::Error;

fn basic_usage(filename: &str) -> Result<(), Error> {

    let vendor = openslide::detect_vendor(filename)?;
    println!("Vendor: {}", vendor);

    let osr = openslide::open(filename)?;

    let levels = openslide::get_level_count(osr)?;
    println!("Slide has {} levels", levels);

    let (height, width) = openslide::get_level0_dimensions(osr)?;
    println!("Slide has dimension {} x {} at level 0", height, width);

    let level = 0;
    let (height, width) = openslide::get_level_dimensions(osr, level)?;
    println!("Slide has dimension {} x {} at level {}", height, width, level);

    let factor = openslide::get_level_downsample(osr, level)?;
    println!("Slide at level {} is downsampled with factor {}", level, factor);

    let downsample_factor = 5.6;
    let level = openslide::get_best_level_for_downsample(osr, downsample_factor)?;
    println!("Best level for downsample factor {} is {}", downsample_factor, level);

    let x = 1000;
    let y = 1500;
    let level = 0;
    let h = 512;
    let w = 512;
    let word_repr = openslide::WordRepresentation::BigEndian;
    let buffer = openslide::read_region(osr, x, y, level, w, h)?;
    let im = openslide::decode_buffer(&buffer, h as u32, w as u32, word_repr)?;
    im.save(Path::new("/tmp/wsi_region.png"))?;

    openslide::close(osr)?;

    Ok(())
}

fn properties(filename: &str) -> Result<(), Error> {
    let osr = openslide::open(filename)?;

    println!("Slide in {} has the following properties:", filename);
    println!("{0:<40} {1}", "Property key", "Property value");
    for name in openslide::get_property_names(osr)? {
        println!("{0:<40} {1}", name, openslide::get_property_value(osr, &name)?);
    }

    openslide::close(osr)?;
    Ok(())
}

fn main() {
    let filename = "assets/CMU-1-Small-Region.svs";
    println!("Analyzing {}", filename);

    match basic_usage(filename) {
        Ok(_) => println!("Basic usage functions are working okay"),
        Err(msg) => {
            println!("Basic usage functions not working");
            println!("{}", msg);
        },
    }

    match properties(filename) {
        Ok(_) => println!("Property functions are working okay"),
        Err(msg) => {
            println!("Property functions not working");
            println!("{}", msg);
        },
    }

    println!("Example program is terminated");
}
