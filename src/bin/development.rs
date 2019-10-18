//! Developement tests
//!

extern crate image;
extern crate openslide;
#[macro_use]
extern crate clap;

use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::error::Error;

use clap::{App, Arg, ArgMatches};
use openslide::OpenSlide;

fn get_cli<'a>() -> ArgMatches<'a> {
    let matches = App::new("Analyse wsi")
        .version("0.2.0")
        .author("Ole-Johan Skrede")
        .about("Toy program used for development of openslide.")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input_file")
                .value_name("FILE")
                .help("Filename of input wsi image")
                .required(true),
        )
        .arg(
            Arg::with_name("out_root_dir")
                .short("o")
                .long("out_root_dir")
                .value_name("FOLDER")
                .default_value("/tmp/analyse_wsi")
                .help("Root directory of output images"),
        )
        .arg(
            Arg::with_name("source_row")
                .short("r")
                .long("source_row")
                .value_name("UINT32")
                .default_value("0")
                .help("Crop start row in target pixels"),
        )
        .arg(
            Arg::with_name("source_column")
                .short("c")
                .long("source_column")
                .value_name("UINT32")
                .default_value("0")
                .help("Crop end row in target pixels"),
        )
        .arg(
            Arg::with_name("target_height")
                .short("h")
                .long("target_height")
                .allow_hyphen_values(true)
                .value_name("INT32")
                .default_value("512")
                .help("Size of crop in target pixels. -1 indicates full height"),
        )
        .arg(
            Arg::with_name("target_width")
                .short("w")
                .long("target_width")
                .allow_hyphen_values(true)
                .value_name("INT32")
                .default_value("512")
                .help("Size of crop in target pixels. -1 indicates full width"),
        )
        .arg(
            Arg::with_name("zoom_factor")
                .short("z")
                .long("zoom_factor")
                .value_name("FLOAT32")
                .default_value("1.0")
                .help("Zoom factor"),
        )
        .arg(
            Arg::with_name("print_properties")
                .short("p")
                .long("print_properties")
                .takes_value(false)
                .help("Whether or not to print the slide properties"),
        )
        .get_matches();

    matches
}

fn write_region(
    fname: Option<&OsStr>,
    os: &OpenSlide,
    out_dir: &Path,
    source_row: u32,
    source_col: u32,
    target_height: i32,
    target_width: i32,
    zoom_factor: f32,
) -> Result<(), Box<dyn Error>> {
    let zoom_lvl = os.get_best_level_for_downsample(zoom_factor as f64)?;
    println!("Max number of levels: {}", os.get_level_count()?);
    println!(
        "Best zoom level for zoom factor {} is: {}",
        zoom_factor, zoom_lvl
    );
    println!(
        "Level {} downsample: {}",
        zoom_lvl,
        os.get_level_downsample(zoom_lvl as u8)?
    );
    println!(
        "Level {} dimensions: {:?}",
        zoom_lvl,
        os.get_level_dimensions(zoom_lvl)?
    );

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
    let im = os.read_region(
        source_row,
        source_col,
        zoom_lvl,
        target_height,
        target_width,
    )?;
    /*
    let out_fname = out_dir.join(format!("wsi_region_x{}_y{}_h{}_w{}_z{}.png",
                                 source_row,
                                 source_col,
                                 target_height,
                                 target_width,
                                 zoom_lvl))
    */
    let out_fname = match fname {
        Some(name) => out_dir.join(format!("{}.png", name.to_string_lossy())),
        None => out_dir.join("wsi_crop.png"),
    };

    im.save(&out_fname)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_cli();

    let input_file = match matches.value_of("input_file") {
        Some(val) => {
            let filepath = Path::new(val);
            if filepath.exists() {
                filepath
            } else {
                println!("Input file does not exist");
                ::std::process::exit(1);
            }
        }
        None => unreachable!(),
    };

    let out_dir = match matches.value_of("out_root_dir") {
        Some(val) => {
            let dirpath = Path::new(val);
            if !dirpath.exists() {
                fs::create_dir_all(&dirpath)?;
            }
            dirpath
        }
        None => unreachable!(),
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

    println!("Num levels: {}", os.get_level_count()?);
    println!("Dimensions at level 0: {:?}", os.get_level0_dimensions()?);
    println!("Dimensions at level 0: {:?}", os.get_level_dimensions(0)?);
    println!(
        "Downsample factor at level 0: {}",
        os.get_level_downsample(0)?
    );
    println!(
        "Best level for downsampling factor 4.5: {}",
        os.get_best_level_for_downsample(4.5)?
    );

    if matches.is_present("print_properties") {
        println!("Properties from dict");
        for (key, val) in os.get_properties()? {
            println!("{0:<40} {1}", key, val);
        }
        println!("Properties from struct");
        os.predefined_properties.print_available()
    }

    write_region(
        input_file.file_stem(),
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
