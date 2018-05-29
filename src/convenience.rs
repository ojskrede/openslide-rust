//! This module contains functions that often are more convenient to use than the raw openslide
//! wrappers
//!

use failure::Error;
use image::{Rgba, RgbaImage};

pub enum WordRepresentation {
    BigEndian,
    LittleEndian,
}

/// This function takes a buffer, as the one obtained from openslide::read_region, and decodes into
/// an Rgba image buffer.
pub fn decode_buffer(buffer: &Vec<u32>,
                     height: u32,
                     width: u32,
                     word_representation: WordRepresentation) -> Result<RgbaImage, Error> {
    let (a_pos, r_pos, g_pos, b_pos) = match word_representation {
        WordRepresentation::BigEndian => (0, 1, 2, 3),
        WordRepresentation::LittleEndian => (3, 2, 1, 0),
    };

    let mut rgba_image = RgbaImage::new(width as u32, height as u32);

    for (col, row, pixel) in rgba_image.enumerate_pixels_mut() {
        let curr_pos = row * width + col;
        let values = buffer[curr_pos as usize];
        // TODO: Iterate over chars() instead (?)
        let bit_repr = format!("{:b}", values);
        let alpha_bit_repr = String::from(&bit_repr[(8 * a_pos)..(8 * a_pos + 8)]);
        let red_bit_repr = String::from(&bit_repr[(8 * r_pos)..(8 * r_pos + 8)]);
        let green_bit_repr = String::from(&bit_repr[(8 * g_pos)..(8 * g_pos + 8)]);
        let blue_bit_repr = String::from(&bit_repr[(8 * b_pos)..(8 * b_pos + 8)]);

        let alpha = u8::from_str_radix(&alpha_bit_repr, 2)?;
        let mut red = u8::from_str_radix(&red_bit_repr, 2)?;
        let mut green = u8::from_str_radix(&green_bit_repr, 2)?;
        let mut blue = u8::from_str_radix(&blue_bit_repr, 2)?;


        if alpha != 0 && alpha != 255 {
            red = (red as f32 * (255.0 / alpha as f32)).round().max(0.0).min(255.0) as u8;
            green = (green as f32 * (255.0 / alpha as f32)).round().max(0.0).min(255.0) as u8;
            blue = (blue as f32 * (255.0 / alpha as f32)).round().max(0.0).min(255.0) as u8;
        }

        *pixel = Rgba([red, green, blue, alpha]);
    }

    Ok(rgba_image)
}
