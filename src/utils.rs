//! Misc utility definitions

use byteorder::ByteOrder;
use image::{Rgba, RgbaImage};
use num::{Integer, ToPrimitive, Unsigned};
use std::fmt::{Debug, Display};
use std::str;
use std::path::Path;
use std::io;

use crate::error::{Error, ErrorKind};

/// Check if the input filepath exists, and return an error if it does not
pub fn check_existence<T: Into<Vec<u8>>>(filepath: T) -> Result<(), Error> {
    let p_bytes = filepath.into();
    let p_str = str::from_utf8(&p_bytes)?;
    let p = Path::new(&p_str);
    if !p.exists() {
        return Err(Error::from(io::Error::new(io::ErrorKind::NotFound, format!("{}", p.display()))))
    }
    Ok(())
}

/// Convenience conversion function that return result
pub fn to_u32<T: ToPrimitive>(number: T) -> Result<u32, Error> {
    number
        .to_u32()
        .ok_or(Error::new(ErrorKind::NumPrimitiveCast { message: "to u32".to_string() } ))
}

/// Convenience conversion function that return result
pub fn to_u64<T: ToPrimitive>(number: T) -> Result<u64, Error> {
    number
        .to_u64()
        .ok_or(Error::new(ErrorKind::NumPrimitiveCast { message: "to u64".to_string() } ))
}

/// Convenience conversion function that return result
pub fn to_i32<T: ToPrimitive>(number: T) -> Result<i32, Error> {
    number
        .to_i32()
        .ok_or(Error::new(ErrorKind::NumPrimitiveCast { message: "to i32".to_string() } ))
}

/// Convenience conversion function that return result
pub fn to_i64<T: ToPrimitive>(number: T) -> Result<i64, Error> {
    number
        .to_i64()
        .ok_or(Error::new(ErrorKind::NumPrimitiveCast { message: "to i64".to_string() } ))
}

/// Convenience conversion function that return result
pub fn to_f64<T: ToPrimitive>(number: T) -> Result<f64, Error> {
    number
        .to_f64()
        .ok_or(Error::new(ErrorKind::NumPrimitiveCast { message: "to f64".to_string() } ))
}

/// A list of supported formats
///
/// Information gathered from [https://openslide.org/formats/](https://openslide.org/formats/)
///
#[derive(Clone, Debug)]
pub enum Format {
    /// Single-file pyramidal tiled TIFF, with non-standard metadata and compression.
    ///
    /// File extensions:
    /// 	.svs, .tif
    Aperio,
    /// Multi-file JPEG/NGR with proprietary metadata and index file formats, and single-file
    /// TIFF-like format with proprietary metadata.
    ///
    /// File extensions:
    /// 	.vms, .vmu, .ndpi
    Hamamatsu,
    /// Single-file pyramidal tiled BigTIFF with non-standard metadata.
    ///
    /// File extensions
    /// 	.scn
    Leica,
    /// Multi-file with very complicated proprietary metadata and indexes.
    ///
    /// File extensions
    /// 	.mrxs
    Mirax,
    /// Single-file pyramidal tiled TIFF or BigTIFF with non-standard metadata.
    ///
    /// File extensions
    ///     .tiff
    Phillips,
    /// SQLite database containing pyramid tiles and metadata.
    ///
    /// File extensions
    ///     .svslide
    Sakura,
    /// Single-file pyramidal tiled TIFF, with non-standard metadata and overlaps. Additional files
    /// contain more metadata and detailed overlap info.
    ///
    /// File extensions
    ///     .tif
    Trestle,
    /// Single-file pyramidal tiled BigTIFF, with non-standard metadata and overlaps.
    ///
    /// File extensions
    ///     .bif, .tif
    Ventana,
    /// Single-file pyramidal tiled TIFF.
    ///
    /// File extensions
    ///     .tif
    GenericTiledTiff,
}

/// The different ways the u8 color values are encoded into a u32 value.
///
/// A successfull reading from OpenSlide's `read_region()` will result in a buffer of `u32` with
/// `height * width` elements, where `height` and `width` is the shape (in pixels) of the read
/// region. This `u32` value consist of four `u8` values which are the red, green, blue, and alpha
/// value of a certain pixel. This enum determines in which order to arange these channels within
/// one element.
#[derive(Clone, Debug)]
pub enum WordRepresentation {
    /// From most significant bit to least significant bit: `[alpha, red, green, blue]`
    BigEndian,
    /// From most significant bit to least significant bit: `[blue, green, red, alpha]`
    LittleEndian,
}

/// This function takes a buffer, as the one obtained from openslide::read_region, and decodes into
/// an Rgba image buffer.
pub fn decode_buffer<T: Unsigned + Integer + ToPrimitive + Debug + Display + Clone + Copy>(
    buffer: &Vec<u32>,
    height: T,
    width: T,
    word_representation: WordRepresentation,
) -> Result<RgbaImage, Error> {
    let height = height
        .to_u32()
        .ok_or(Error::new(ErrorKind::NumPrimitiveCast { message: "to u32".to_string() } ))?;
    let width = width
        .to_u32()
        .ok_or(Error::new(ErrorKind::NumPrimitiveCast { message: "to u32".to_string() } ))?;
    let mut rgba_image = RgbaImage::new(width, height);

    for (col, row, pixel) in rgba_image.enumerate_pixels_mut() {
        let curr_pos = row * width + col;
        let value = buffer[curr_pos as usize];

        let mut buf = [0; 4];
        match word_representation {
            WordRepresentation::BigEndian => byteorder::BigEndian::write_u32(&mut buf, value),
            WordRepresentation::LittleEndian => byteorder::BigEndian::write_u32(&mut buf, value),
        };
        let [alpha, mut red, mut green, mut blue] = buf;

        if alpha != 0 && alpha != 255 {
            red = (red as f32 * (255.0 / alpha as f32))
                .round()
                .max(0.0)
                .min(255.0) as u8;
            green = (green as f32 * (255.0 / alpha as f32))
                .round()
                .max(0.0)
                .min(255.0) as u8;
            blue = (blue as f32 * (255.0 / alpha as f32))
                .round()
                .max(0.0)
                .min(255.0) as u8;
        }

        *pixel = Rgba([red, green, blue, alpha]);
    }

    Ok(rgba_image)
}
