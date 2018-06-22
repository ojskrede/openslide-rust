//! This module contains functions that often are more convenient to use than the raw OpenSlide
//! wrappers
//!

use std::path::Path;
use std::collections::HashMap;
use failure::{err_msg, Error};
use image::{RgbaImage};

use ::{utils, bindings};

/// A convenient OpenSlide object with the ordinary OpenSlide functions as methods
///
/// This wraps the bindings found in the bindings module, but has a more (in my opinion) convenient
/// API for rust. It also contains some other convenience methods.
#[derive(Clone)]
pub struct OpenSlide {
    osr: *const bindings::OpenSlideT,
}

impl Drop for OpenSlide {
    /// This method is called when the object in dropped, and tries to close the slide.
    fn drop(
        &mut self
    ) {
        bindings::close(self.osr);
    }
}

impl OpenSlide {
    /// This method tries to open the slide at the given filename location.
    ///
    /// This function can be expensive; avoid calling it unnecessarily. For example, a tile server
    /// should not create a new object on every tile request. Instead, it should maintain a cache
    /// of OpenSlide objects and reuse them when possible.
    pub fn new(
        filename: &Path
    ) -> Result<OpenSlide, Error> {
        if !filename.exists() {
            return Err(err_msg(format!("Error: Nonexisting path: {}", filename.display())));
        }

        let osr = bindings::open(filename.to_str().ok_or(err_msg("Error: Path to &str"))?)?;

        Ok(OpenSlide {
            osr: osr,
        })
    }

    /// Get the number of levels in the whole slide image.
    pub fn get_level_count(
        &self
    ) -> Result<u8, Error> {
        let num_levels = bindings::get_level_count(self.osr)?;

        if num_levels < -1 {
            Err(err_msg(format!("Error: Number of levels is {}, this is an unknown error from OpenSlide. \
                                 OpenSlide returns -1 if an error occured. \
                                 See OpenSlide C API documentation.", num_levels)))
        } else if num_levels == -1 {
            Err(err_msg("Error: Number of levels is -1, this is a known error from OpenSlide. \
                         OpenSlide returns -1 if an error occured. \
                         See OpenSlide C API documentation."))
        } else if num_levels >= 0 && num_levels < 256 {
            Ok(num_levels as u8)
        } else {
            Err(err_msg(format!("Error: Number of levels is {}. This is more than for any supported \
                                 vendors, and can indicate an error", num_levels)))
        }
    }

    /// Get the dimensions of level 0 (the largest level).
    ///
    /// This method returns the (width, height) number of pixels of the level 0 whole slide image.
    ///
    /// This is the same as calling get_level_dimensions(level) with level=0.
    pub fn get_level0_dimensions(
        &self
    ) -> Result<(u64, u64), Error> {
        let (width, height) = bindings::get_level0_dimensions(self.osr)?;

        if width < -1 {
            return Err(err_msg(format!("Error: Width is {}, this is an unknown error from OpenSlide. \
                                        OpenSlide returns -1 if an error occured. \
                                        See OpenSlide C API documentation.", width)))
        } else if width == -1 {
            return Err(err_msg("Error: Width is -1, this is a known error from OpenSlide. \
                                OpenSlide returns -1 if an error occured. \
                                See OpenSlide C API documentation."))
        }

        if height < -1 {
            return Err(err_msg(format!("Error: Height is {}, this is an unknown error from OpenSlide. \
                                        OpenSlide returns -1 if an error occured. \
                                        See OpenSlide C API documentation.", width)))
        } else if height == -1 {
            return Err(err_msg("Error: Height is -1, this is a known error from OpenSlide. \
                                OpenSlide returns -1 if an error occured. \
                                See OpenSlide C API documentation."))
        }

        Ok((width as u64, height as u64))
    }

    /// Get the dimensions of level 0 (the largest level).
    ///
    /// This method returns the (width, height) number of pixels of the whole slide image at the
    /// specified level. Returns an error if the level is invalid
    pub fn get_level_dimensions(
        &self,
        level: u8,
    ) -> Result<(u64, u64), Error> {

        let max_num_levels = self.get_level_count()?;
        if level > max_num_levels {
            return Err(err_msg(format!("Error: Specified level {} is larger than the max slide level {}",
                                       level, max_num_levels)));
        }

        let (width, height) = bindings::get_level_dimensions(self.osr, level as i32)?;

        if width < -1 {
            return Err(err_msg(format!("Error: Width is {}, this is an unknown error from OpenSlide. \
                                        OpenSlide returns -1 if an error occured. \
                                        See OpenSlide C API documentation.", width)))
        } else if width == -1 {
            return Err(err_msg("Error: Width is -1, this is a known error from openslide. \
                                OpenSlide returns -1 if an error occured. \
                                See OpenSlide C API documentation."))
        }

        if height < -1 {
            return Err(err_msg(format!("Error: Height is {}, this is an unknown error from OpenSlide. \
                                        OpenSlide returns -1 if an error occured. \
                                        See OpenSlide C API documentation.", width)))
        } else if height == -1 {
            return Err(err_msg("Error: Height is -1, this is a known error from openslide. \
                                OpenSlide returns -1 if an error occured. \
                                See OpenSlide C API documentation."))
        }

        Ok((width as u64, height as u64))
    }

    /// Get the downsampling factor of a given level.
    pub fn get_level_downsample(
        &self,
        level: u8,
    ) -> Result<f64, Error> {

        let max_num_levels = self.get_level_count()?;
        if level > max_num_levels {
            return Err(err_msg(format!("Error: Specified level {} is larger than the max slide level {}",
                                       level, max_num_levels)));
        }

        let downsample_factor = bindings::get_level_downsample(self.osr, level as i32)?;

        if downsample_factor < 0.0 {
            return Err(err_msg(format!("Error: Downsample factor is {}, this is an error from \
                                        OpenSlide. OpenSlide returns -1.0 if an error occured. \
                                        See OpenSlide C API documentation.", downsample_factor)))
        }

        Ok(downsample_factor)
    }

    /// Get the best level to use for displaying the given downsample factor.
    pub fn get_best_level_for_downsample(
        &self,
        downsample_factor: f64,
    ) -> Result<u8, Error> {

        if downsample_factor < 0.0 {
            return Err(err_msg(format!("Error: Only non-negative downsample factor is allowed. \
                                        You specified {}. ", downsample_factor)))
        }

        let level = bindings::get_best_level_for_downsample(self.osr, downsample_factor)?;

        if level < -1 {
            Err(err_msg(format!("Error: Returned level is {}, this is an unknown error from OpenSlide. \
                                 OpenSlide returns -1 if an error occured. \
                                 See OpenSlide C API documentation.", level)))
        } else if level == -1 {
            Err(err_msg("Error: Returned level is -1, this is a known error from openslide. \
                         OpenSlide returns -1 if an error occured. \
                         See OpenSlide C API documentation."))
        } else if level >= 0 && level < 256 {
            Ok(level as u8)
        } else {
            Err(err_msg(format!("Error: Returned level is {}. This is more than for any supported \
                                 vendors, and can indicate an error", level)))
        }
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
    pub fn read_region(
        &self,
        top_left_lvl0_row: u32,
        top_left_lvl0_col: u32,
        level: u8,
        height: u32,
        width: u32,
    ) -> Result<RgbaImage, Error> {
        let buffer = bindings::read_region(self.osr,
                                           top_left_lvl0_col as i64,
                                           top_left_lvl0_row as i64,
                                           level as i32,
                                           width as i64,
                                           height as i64)?;
        let word_repr = utils::WordRepresentation::BigEndian;
        utils::decode_buffer(&buffer, height, width, word_repr)
    }

    /// Get a dictionary of properties associated with the current slide
    ///
    /// There are some standard properties to every slide, but also a lot of vendor-specific
    /// properties. This method returns a HashMap with all key-value pairs of the properties
    /// associated with the slide.
    pub fn get_properties(
        &self
    ) -> Result<HashMap<String, String>, Error> {
        let mut properties = HashMap::<String, String>::new();
        for name in bindings::get_property_names(self.osr)? {
            properties.insert(name.clone(), bindings::get_property_value(self.osr, &name)?);
        }
        Ok(properties)
    }


}
