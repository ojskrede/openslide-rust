//! Misc utility definitions

use std::fmt::{Display, Debug};

use num::{ToPrimitive, Unsigned, Integer};
use image::{Rgba, RgbaImage};
use failure::{err_msg, Error, format_err};


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

fn bit_to_u8(bit_representation: &str, name: &str) -> Result<u8, Error> {
    match u8::from_str_radix(bit_representation, 2) {
        Ok(val) => Ok(val),
        Err(msg) => {
            eprintln!("Error in parsing bits as u8 for {} channel", name);
            Err(format_err!("{:?}", msg))
        },
    }
}

/// This function takes a buffer, as the one obtained from openslide::read_region, and decodes into
/// an Rgba image buffer.
pub fn decode_buffer<T: Unsigned + Integer + ToPrimitive + Debug + Display + Clone + Copy>(
    buffer: &Vec<u32>,
    height: T,
    width: T,
    word_representation: WordRepresentation
) -> Result<RgbaImage, Error> {
    let (a_pos, r_pos, g_pos, b_pos) = match word_representation {
        WordRepresentation::BigEndian => (0, 1, 2, 3),
        WordRepresentation::LittleEndian => (3, 2, 1, 0),
    };

    let mut rgba_image = RgbaImage::new(
        width.to_u32().ok_or(err_msg("Conversion to primitive error"))?,
        height.to_u32().ok_or(err_msg("Conversion to primitive error"))?);

    for (col, row, pixel) in rgba_image.enumerate_pixels_mut() {
        let curr_pos = row * width.to_u32().ok_or(err_msg("Conversion to primitive error"))? + col;
        let values = buffer[curr_pos as usize];
        // TODO: Iterate over chars() instead (?)
        let bit_repr = format!("{:b}", values);
        let alpha_bit_repr = String::from(&bit_repr[(8 * a_pos)..(8 * a_pos + 8)]);
        let red_bit_repr = String::from(&bit_repr[(8 * r_pos)..(8 * r_pos + 8)]);
        let green_bit_repr = String::from(&bit_repr[(8 * g_pos)..(8 * g_pos + 8)]);
        let blue_bit_repr = String::from(&bit_repr[(8 * b_pos)..(8 * b_pos + 8)]);

        let alpha = bit_to_u8(&alpha_bit_repr, "alpha")?;
        let mut red = bit_to_u8(&red_bit_repr, "red")?;
        let mut green = bit_to_u8(&green_bit_repr, "green")?;
        let mut blue = bit_to_u8(&blue_bit_repr, "blue")?;


        if alpha != 0 && alpha != 255 {
            red = (red as f32 * (255.0 / alpha as f32)).round().max(0.0).min(255.0) as u8;
            green = (green as f32 * (255.0 / alpha as f32)).round().max(0.0).min(255.0) as u8;
            blue = (blue as f32 * (255.0 / alpha as f32)).round().max(0.0).min(255.0) as u8;
        }

        *pixel = Rgba([red, green, blue, alpha]);
    }

    Ok(rgba_image)
}
