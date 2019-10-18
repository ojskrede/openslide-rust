//! Example of how to use the raw bindings
//!
//! NOTE how we explicitly have to call
//!
//! ```
//! bindings::close(osr);
//! ```
//!
//! when an error occurs, or when the scope ends. We detect the error inside all the openslide::*
//! functions, and should ideally destruct the object there. But in the safe interface
//! (convenience), the OpenSlide object implements Drop by closing the slide. If an error occurs
//! there, and it is already closed, it will be a double free if the OpenSlide struct goes out of
//! scope, which it will, even if the error occured.
//!
//! As I see it, there are three options
//!
//! 1: Do it as it is implemented now, leaving it ut to the user of the raw interface to remember
//!    to close the osr object at appropriate times.
//! 2: Do it as we do it now and hide / remove the raw interface from public
//! 3: Close the slide straight after an error is occured and edit the Drop implementation. This
//!    is probably the best option if I could figure out some way to detect if the slide has
//!    already been freed (and then not free it again when calling Drop), or only drop it if an
//!    error did not occur.

extern crate openslide;

use openslide::{bindings, utils};
use std::path::Path;
use std::error::Error;

fn basic_usage(filename: &str) -> Result<(), Box<dyn Error>> {
    let vendor = bindings::detect_vendor(filename)?;
    println!("Vendor: {}", vendor);

    let osr = bindings::open(filename)?;

    let levels = bindings::get_level_count(osr)?;
    println!("Slide has {} levels", levels);

    let (height, width) = bindings::get_level0_dimensions(osr)?;
    println!("Slide has dimension {} x {} at level 0", height, width);

    let level = 0;
    let (height, width) = bindings::get_level_dimensions(osr, level)?;
    println!(
        "Slide has dimension {} x {} at level {}",
        height, width, level
    );

    let factor = bindings::get_level_downsample(osr, level)?;
    println!(
        "Slide at level {} is downsampled with factor {}",
        level, factor
    );

    let downsample_factor = 5.6;
    let level = bindings::get_best_level_for_downsample(osr, downsample_factor)?;
    println!(
        "Best level for downsample factor {} is {}",
        downsample_factor, level
    );

    let x = 0;
    let y = 960;
    let level = 1;
    let h = 100;
    let w = 100;
    let word_repr = utils::WordRepresentation::BigEndian;
    let buffer = bindings::read_region(osr, x, y, level, w, h)?;
    let im = utils::decode_buffer(&buffer, h as u32, w as u32, word_repr)?;
    im.save(Path::new("/tmp/wsi_region_1.png"))?;
    println!("Region is written");

    // Test error
    //let factor = bindings::get_level_downsample(osr, 2)?;
    //println!("{:?}", bindings::get_error(osr));

    bindings::close(osr);

    Ok(())
}

fn properties(filename: &str) -> Result<(), Box<dyn Error>> {
    let osr = bindings::open(filename)?;

    println!("Slide in {} has the following properties:", filename);
    println!("{0:<40} {1}", "Property key", "Property value");
    for name in bindings::get_property_names(osr)? {
        println!(
            "{0:<40} {1}",
            name,
            bindings::get_property_value(osr, &name)?
        );
    }

    bindings::close(osr);
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
        }
    }

    match properties(filename) {
        Ok(_) => println!("Property functions are working okay"),
        Err(msg) => {
            println!("Property functions not working");
            println!("{}", msg);
        }
    }

    println!("Example program is terminated");
}
