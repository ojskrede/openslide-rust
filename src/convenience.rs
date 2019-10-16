//! This module contains functions that often are more convenient to use than the raw OpenSlide
//! wrappers
//!

use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::path::Path;

use failure::{format_err, Error};
use image::RgbaImage;
use num::zero;
use num::{Integer, Num, ToPrimitive, Unsigned};

use {bindings, predefined_properties::PredefinedProperties, utils};

/// A convenient OpenSlide object with the ordinary OpenSlide functions as methods
///
/// This wraps the bindings found in the bindings module, but has a more (in my opinion) convenient
/// API for rust. It also contains some other convenience methods.
#[derive(Clone)]
pub struct OpenSlide {
    osr: *const bindings::OpenSlideType,
    pub predefined_properties: PredefinedProperties,
}

impl Drop for OpenSlide {
    /// This method is called when the object in dropped, and tries to close the slide.
    fn drop(&mut self) {
        // As recommended in the openslide library, we close the slide immediately after it is in
        // an non-null error state. If this is the case, this would result in a double free if
        // tried to close it here also. For this reason, it is only closed if the slide is not in
        // an error state.
        if bindings::get_error(self.osr).is_none() {
            bindings::close(self.osr);
        }
    }
}

impl OpenSlide {
    /// This method tries to open the slide at the given filename location.
    ///
    /// This function can be expensive; avoid calling it unnecessarily. For example, a tile server
    /// should not create a new object on every tile request. Instead, it should maintain a cache
    /// of OpenSlide objects and reuse them when possible.
    pub fn new(filename: &Path) -> Result<OpenSlide, Error> {
        if !filename.exists() {
            return Err(format_err!(
                "Error: Nonexisting path: {}",
                filename.display()
            ));
        }

        let osr = bindings::open(
            filename
                .to_str()
                .ok_or(format_err!("Error: Path to &str"))?,
        )?;

        let mut property_map = HashMap::<String, String>::new();
        for name in bindings::get_property_names(osr)? {
            property_map.insert(name.clone(), bindings::get_property_value(osr, &name)?);
        }
        let predefined_properties = PredefinedProperties::new(&property_map);

        Ok(OpenSlide {
            osr,
            predefined_properties,
        })
    }

    /// Get the number of levels in the whole slide image.
    pub fn get_level_count(&self) -> Result<u32, Error> {
        let num_levels = bindings::get_level_count(self.osr)?;

        if num_levels < -1 {
            Err(format_err!(
                "Error: Number of levels is {}, this is an unknown error from OpenSlide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
                num_levels
            ))
        } else if num_levels == -1 {
            Err(format_err!(
                "Error: Number of levels is -1, this is a known error from OpenSlide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
            ))
        } else {
            Ok(num_levels as u32)
        }
    }

    /// Get the dimensions of level 0 (the largest level).
    ///
    /// This method returns the (width, height) number of pixels of the level 0 whole slide image.
    ///
    /// This is the same as calling get_level_dimensions(level) with level=0.
    pub fn get_level0_dimensions(&self) -> Result<(u64, u64), Error> {
        let (width, height) = bindings::get_level0_dimensions(self.osr)?;

        if width < -1 {
            return Err(format_err!(
                "Error: Width is {}, this is an unknown error from OpenSlide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
                width
            ));
        } else if width == -1 {
            return Err(format_err!(
                "Error: Width is -1, this is a known error from OpenSlide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
            ));
        }

        if height < -1 {
            return Err(format_err!(
                "Error: Height is {}, this is an unknown error from OpenSlide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
                width
            ));
        } else if height == -1 {
            return Err(format_err!(
                "Error: Height is -1, this is a known error from OpenSlide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
            ));
        }

        Ok((width as u64, height as u64))
    }

    /// Get the dimensions of level 0 (the largest level).
    ///
    /// This method returns the (width, height) number of pixels of the whole slide image at the
    /// specified level. Returns an error if the level is invalid
    pub fn get_level_dimensions<T: Integer + ToPrimitive + Debug + Display + Clone + Copy>(
        &self,
        level: T,
    ) -> Result<(u64, u64), Error> {
        self.assert_level_validity(level)?;
        let level = level
            .to_i32()
            .ok_or(format_err!("Conversion to primitive error"))?;

        let (width, height) = bindings::get_level_dimensions(self.osr, level)?;

        if width < -1 {
            return Err(format_err!(
                "Error: Width is {}, this is an unknown error from OpenSlide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
                width
            ));
        } else if width == -1 {
            return Err(format_err!(
                "Error: Width is -1, this is a known error from openslide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
            ));
        }

        if height < -1 {
            return Err(format_err!(
                "Error: Height is {}, this is an unknown error from OpenSlide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
                width
            ));
        } else if height == -1 {
            return Err(format_err!(
                "Error: Height is -1, this is a known error from openslide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
            ));
        }

        Ok((width as u64, height as u64))
    }

    /// Get the downsampling factor of a given level.
    pub fn get_level_downsample<T: Integer + ToPrimitive + Debug + Display + Clone + Copy>(
        &self,
        level: T,
    ) -> Result<f64, Error> {
        self.assert_level_validity(level)?;
        let level = level
            .to_i32()
            .ok_or(format_err!("Conversion to primitive error"))?;
        let downsample_factor = bindings::get_level_downsample(self.osr, level)?;

        if downsample_factor < 0.0 {
            return Err(format_err!(
                "Error: When trying to get a downsample factor for level {},\
                 OpenSlide returned a downsample factor {}, this is an error from \
                 OpenSlide. OpenSlide returns -1.0 if an error occured. \
                 See OpenSlide C API documentation.",
                level,
                downsample_factor
            ));
        }

        Ok(downsample_factor)
    }

    /// Get the best level to use for displaying the given downsample factor.
    pub fn get_best_level_for_downsample<
        T: Num + ToPrimitive + PartialOrd + Debug + Display + Clone + Copy,
    >(
        &self,
        downsample_factor: T,
    ) -> Result<u32, Error> {
        if downsample_factor < zero() {
            return Err(format_err!(
                "Error: Only non-negative downsample factor is allowed. \
                 You specified {}. ",
                downsample_factor
            ));
        }

        let level = bindings::get_best_level_for_downsample(
            self.osr,
            downsample_factor
                .to_f64()
                .ok_or(format_err!("Conversion to primitive error"))?,
        )?;

        if level < -1 {
            Err(format_err!(
                "Error: Returned level is {}, this is an unknown error from OpenSlide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
                level
            ))
        } else if level == -1 {
            Err(format_err!(
                "Error: Returned level is -1, this is a known error from openslide. \
                 OpenSlide returns -1 if an error occured. \
                 See OpenSlide C API documentation.",
            ))
        } else {
            Ok(level as u32)
        }
    }

    /// Return (new_height, new_width) where
    ///
    /// new_height = min(height, max_height)
    /// new_width = min(width, max_width)
    ///
    /// and max_{height, width} are computed based on the top left corner coordinates and the
    /// dimensions of the image.
    fn get_feasible_dimensions<
        T: Integer + Unsigned + ToPrimitive + Debug + Display + Clone + Copy,
    >(
        &self,
        top_left_lvl0_row: T,
        top_left_lvl0_col: T,
        level: T,
        height: T,
        width: T,
    ) -> Result<(u64, u64), Error> {
        let (max_width, max_height) = self.get_level_dimensions(level)?;
        let downsample_factor = self.get_level_downsample(level)?;

        let tl_row_this_lvl = top_left_lvl0_row
            .to_f64()
            .ok_or(format_err!("Conversion to primitive error"))?
            / downsample_factor;
        let tl_col_this_lvl = top_left_lvl0_col
            .to_f64()
            .ok_or(format_err!("Conversion to primitive error"))?
            / downsample_factor;

        let new_height = height
            .to_u64()
            .ok_or(format_err!("Conversion to primitive error"))?
            .min(max_height - tl_row_this_lvl.round() as u64);
        let new_width = width
            .to_u64()
            .ok_or(format_err!("Conversion to primitive error"))?
            .min(max_width - tl_col_this_lvl.round() as u64);

        if new_height
            < height
                .to_u64()
                .ok_or(format_err!("Conversion to primitive error"))?
        {
            println!(
                "WARNING: Requested region height is changed from {} to {} in order to fit",
                height, new_height
            );
        }
        if new_width
            < width
                .to_u64()
                .ok_or(format_err!("conversion to primitive error"))?
        {
            println!(
                "WARNING: Requested region width is changed from {} to {} in order to fit",
                width, new_width
            );
        }

        if new_height > max_height {
            return Err(format_err!(
                "Requested height {} exceeds maximum {}",
                height,
                max_height
            ));
        }

        if new_width > max_width {
            return Err(format_err!(
                "Requested width {} exceeds maximum {}",
                width,
                max_width
            ));
        }

        Ok((new_height, new_width))
    }

    /// Copy pre-multiplied ARGB data from a whole slide image.
    ///
    /// This function reads and decompresses a region of a whole slide image into an RGBA image
    ///
    /// Args:
    ///     top_left_lvl0_row: Row coordinate (increasing downwards) of top left pixel position
    ///     top_left_lvl0_col: Column coordinate (increasing to the right) of top left pixel
    ///                        position
    ///     level: At which level to grab the region from
    ///     height: Height in pixels of the outputted region
    ///     width: Width in pixels of the outputted region
    pub fn read_region<T: Integer + Unsigned + ToPrimitive + Debug + Display + Clone + Copy>(
        &self,
        top_left_lvl0_row: T,
        top_left_lvl0_col: T,
        level: T,
        height: T,
        width: T,
    ) -> Result<RgbaImage, Error> {
        let (height, width) = self.get_feasible_dimensions(
            top_left_lvl0_row,
            top_left_lvl0_col,
            level,
            height,
            width,
        )?;

        let buffer = bindings::read_region(
            self.osr,
            top_left_lvl0_col
                .to_i64()
                .ok_or(format_err!("Conversion to primitive error"))?,
            top_left_lvl0_row
                .to_i64()
                .ok_or(format_err!("Conversion to primitive error"))?,
            level
                .to_i32()
                .ok_or(format_err!("Conversion to primitive error"))?,
            width
                .to_i64()
                .ok_or(format_err!("Conversion to primitive error"))?,
            height
                .to_i64()
                .ok_or(format_err!("Conversion to primitive error"))?,
        )?;
        let word_repr = utils::WordRepresentation::BigEndian;
        utils::decode_buffer(&buffer, height, width, word_repr)
    }

    /// Get a dictionary of properties associated with the current slide
    ///
    /// There are some standard properties to every slide, but also a lot of vendor-specific
    /// properties. This method returns a HashMap with all key-value pairs of the properties
    /// associated with the slide.
    pub fn get_properties(&self) -> Result<HashMap<String, String>, Error> {
        let mut properties = HashMap::<String, String>::new();
        for name in bindings::get_property_names(self.osr)? {
            properties.insert(name.clone(), bindings::get_property_value(self.osr, &name)?);
        }
        Ok(properties)
    }

    /// Check if the given level is valid
    fn assert_level_validity<T: Integer + ToPrimitive>(&self, level: T) -> Result<(), Error> {
        let max_num_levels = self.get_level_count()?;
        let level = level
            .to_u32()
            .ok_or(format_err!("Conversion to primitive error"))?;
        if level >= max_num_levels {
            return Err(format_err!(
                "Error: Specified level {} is larger than the max slide level {}",
                level,
                max_num_levels - 1,
            ));
        }
        Ok(())
    }
}
