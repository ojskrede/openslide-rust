//! Developement tests
//!

extern crate openslide;
extern crate failure;
extern crate image;
#[macro_use] extern crate clap;

use std::path::Path;
use std::path;
use std::fs;

use image::{RgbaImage};
use failure::{Error, err_msg};
use clap::{Arg, App, ArgMatches};
use openslide::OpenSlide;


fn get_cli<'a>() -> ArgMatches<'a> {
    let matches = App::new("Analyse wsi")
                          .version("0.1.0")
                          .author("Ole-Johan Skrede")
                          .about("Scale, crop, and analyse whole slide images")
                          .arg(Arg::with_name("input_file")
                               .short("i")
                               .long("input_file")
                               .value_name("FILE")
                               .help("Filename of input wsi image")
                               .required(true)
                               )
                          .arg(Arg::with_name("out_root_dir")
                               .short("o")
                               .long("out_root_dir")
                               .value_name("FOLDER")
                               .default_value("/tmp/analyse_wsi")
                               .help("Root directory of output images")
                               )
                          .arg(Arg::with_name("source_row")
                               .short("r")
                               .long("source_row")
                               .value_name("UINT32")
                               .default_value("0")
                               .help("Crop start row in target pixels")
                               )
                          .arg(Arg::with_name("source_column")
                               .short("c")
                               .long("source_column")
                               .value_name("UINT32")
                               .default_value("0")
                               .help("Crop end row in target pixels")
                               )
                          .arg(Arg::with_name("target_height")
                               .short("h")
                               .long("target_height")
                               .allow_hyphen_values(true)
                               .value_name("INT32")
                               .default_value("512")
                               .help("Size of crop in target pixels. -1 indicates full height")
                               )
                          .arg(Arg::with_name("target_width")
                               .short("w")
                               .long("target_width")
                               .allow_hyphen_values(true)
                               .value_name("INT32")
                               .default_value("512")
                               .help("Size of crop in target pixels. -1 indicates full width")
                               )
                          .arg(Arg::with_name("zoom_factor")
                               .short("z")
                               .long("zoom_factor")
                               .value_name("FLOAT32")
                               .default_value("1.0")
                               .help("Zoom factor")
                               )
                          .arg(Arg::with_name("print_properties")
                               .short("p")
                               .long("print_properties")
                               .takes_value(false)
                               .help("Whether or not to print the slide properties")
                               )
                          .get_matches();

    matches
}


fn write_region(
    os: &OpenSlide,
    out_dir: &path::Path,
    source_row: u32,
    source_col: u32,
    target_height: i32,
    target_width: i32,
    zoom_factor: f32,
) -> Result<(), Error> {

    let zoom_lvl = os.get_best_level_for_downsample(zoom_factor as f64)?;
    println!("Best zoom level for zoom factor {} is: {}", zoom_factor, zoom_lvl);

    let target_height = if target_height == -1 {
        let (_, full_height) = os.get_level_dimensions(zoom_lvl)?;
        full_height as u32
    } else {
        target_height as u32
    };

    let target_width = if target_width == -1 {
        let (full_width, _) = os.get_level_dimensions(zoom_lvl)?;
        full_width as u32
    } else {
        target_width as u32
    };

    println!("Read region with height: {}", target_height);
    println!("Read region with width: {}", target_width);
    let im = os.read_region(source_row, source_col, zoom_lvl, target_height, target_width)?;
    im.save(out_dir.join(format!("wsi_region_x{}_y{}_h{}_w{}_z{}.png",
                                 source_row,
                                 source_col,
                                 target_height,
                                 target_width,
                                 zoom_lvl)))?;

    Ok(())
}

fn main() -> Result<(), Error> {

    let matches = get_cli();

    let input_file = match matches.value_of("input_file") {
        Some(val) => {
            let filepath = path::Path::new(val);
            if filepath.exists() {
                filepath
            } else {
                return Err(err_msg("Input file does not exist"))
            }
        }
        None => unreachable!()
    };

    let out_dir = match matches.value_of("out_root_dir") {
        Some(val) => {
            let dirpath = path::Path::new(val);
            if !dirpath.exists() {
                fs::create_dir_all(&dirpath)?;
            }
            dirpath
        }
        None => unreachable!()
    };

    let source_row = value_t!(matches.value_of("source_row"), u32)?;
    let source_column = value_t!(matches.value_of("source_column"), u32)?;
    let target_height = value_t!(matches.value_of("target_height"), i32)?;
    let target_width = value_t!(matches.value_of("target_width"), i32)?;
    let zoom_factor = {
        let val = value_t!(matches.value_of("zoom_factor"), f32)?;
        if val < 1.0 {
            println!("Too small zoom factor: {}", val);
            println!("Zoom factor below 1.0 does not make sense in this application.");
            println!("Zoom factor is set to 1.0");
            1.0
        } else {
            val
        }
    };

    let os = OpenSlide::new(input_file)?;

    if matches.is_present("print_properties") {
        for (key, val) in os.get_properties()? {
            println!("{0:<40} {1}", key, val);
        }
    }

    write_region(
        &os,
        &out_dir,
        source_row,
        source_column,
        target_height,
        target_width,
        zoom_factor,
        )?;



    Ok(())
}
