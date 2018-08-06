//! Example of how to use the convenience functions
//!

extern crate failure;
extern crate openslide;

use std::path::Path;
use failure::Error;

fn basic_usage(
    filename: &Path
) -> Result<(), Error> {
    let os = openslide::OpenSlide::new(filename)?;

    println!("Num levels: {}", os.get_level_count()?);
    println!("Dimensions at level 0: {:?}", os.get_level0_dimensions()?);
    println!("Dimensions at level 0: {:?}", os.get_level_dimensions(0u8)?);
    println!("Downsample factor at level 0: {}", os.get_level_downsample(0u8)?);
    println!("Best level for downsampling factor 4.5: {}", os.get_best_level_for_downsample(4.5)?);

    let im = os.read_region(1500u32, 1000u32, 0u32, 512u32, 512u32)?;
    im.save("/tmp/wsi_region_2.png")?;

    for (key, val) in os.get_properties()? {
        println!("{0:<40} {1}", key, val);
    }

    println!("{:?}", os.properties.mpp_y());
    println!("{:?}", os.properties.resolution_unit());
    println!("{:?}", os.properties.model());

    Ok(())
}

fn main() {
    let filename = Path::new("assets/CMU-1-Small-Region.svs");
    println!("Analyzing {}", filename.display());

    match basic_usage(&filename) {
        Ok(_) => println!("Basic usage functions are working okay"),
        Err(msg) => {
            println!("Basic usage functions not working");
            println!("{}", msg);
        },
    }
}
