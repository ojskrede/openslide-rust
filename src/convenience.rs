//! This module contains functions that often are more convenient to use than the raw OpenSlide
//! wrappers
//!

use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::path::Path;

use image::RgbaImage;
use num::{zero, Integer, Num, ToPrimitive, Unsigned};

use crate::error::{Error, ErrorKind};
use {bindings, predefined_properties::PredefinedProperties, utils};

/// Quickly determine whether a whole slide image is recognized.
pub fn detect_vendor(filename: &Path) -> Result<String, Error> {
    bindings::detect_vendor(&filename.as_os_str().to_string_lossy())
}

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
        let osr = bindings::open(&filename.as_os_str().to_string_lossy())?;

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

        if num_levels < 0 {
            Err(Error::new(ErrorKind::ReturnValue {
                in_function: "get_level_count".to_string(),
                message: format!("returned num_levels = {}", num_levels),
            }))
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

        if width < 0 {
            return Err(Error::new(ErrorKind::ReturnValue {
                in_function: "get_level0_dimensions".to_string(),
                message: format!("returned width = {}", width),
            }));
        }

        if height < 0 {
            return Err(Error::new(ErrorKind::ReturnValue {
                in_function: "get_level0_dimensions".to_string(),
                message: format!("returned height = {}", height),
            }));
        }

        Ok((width as u64, height as u64))
    }

    /// Get the dimensions of level 0 (the largest level).
    ///
    /// This method returns the (width, height) number of pixels of the whole slide image at the
    /// specified level. Returns an error if the level is invalid
    pub fn get_level_dimensions<T>(&self, level: T) -> Result<(u64, u64), Error>
    where
        T: Integer + ToPrimitive + Debug + Display + Clone + Copy,
    {
        self.assert_level_validity(level)?;
        let level = utils::to_i32(level)?;

        let (width, height) = bindings::get_level_dimensions(self.osr, level)?;

        if width < 0 {
            return Err(Error::new(ErrorKind::ReturnValue {
                in_function: "get_level_dimensions".to_string(),
                message: format!("returned width = {}", width),
            }));
        }

        if height < 0 {
            return Err(Error::new(ErrorKind::ReturnValue {
                in_function: "get_level_dimensions".to_string(),
                message: format!("returned height = {}", height),
            }));
        }

        Ok((width as u64, height as u64))
    }

    /// Get the downsampling factor of a given level.
    pub fn get_level_downsample<T>(&self, level: T) -> Result<f64, Error>
    where
        T: Integer + ToPrimitive + Debug + Display + Clone + Copy,
    {
        self.assert_level_validity(level)?;
        let level = utils::to_i32(level)?;
        let downsample_factor = bindings::get_level_downsample(self.osr, level)?;

        if downsample_factor < 0.0 {
            return Err(Error::new(ErrorKind::ReturnValue {
                in_function: "get_level_downsample".to_string(),
                message: format!("returned downsample_factor = {}", downsample_factor),
            }));
        }

        Ok(downsample_factor)
    }

    /// Get the best level to use for displaying the given downsample factor.
    pub fn get_best_level_for_downsample<T>(&self, downsample_factor: T) -> Result<u32, Error>
    where
        T: Num + ToPrimitive + PartialOrd + Debug + Display + Clone + Copy,
    {
        if downsample_factor < zero() {
            return Err(Error::new(ErrorKind::OutOfBounds {
                message: format!(
                    "Specified downsample factor is negative: {}",
                    downsample_factor
                ),
            }));
        }

        let level =
            bindings::get_best_level_for_downsample(self.osr, utils::to_f64(downsample_factor)?)?;

        if level < 0 {
            Err(Error::new(ErrorKind::ReturnValue {
                in_function: "get_best_level_for_downsample".to_string(),
                message: format!("returned level = {}", level),
            }))
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
    fn get_feasible_dimensions<T>(
        &self,
        top_left_lvl0_row: T,
        top_left_lvl0_col: T,
        level: T,
        height: T,
        width: T,
    ) -> Result<(u64, u64), Error>
    where
        T: Integer + Unsigned + ToPrimitive + Debug + Display + Clone + Copy,
    {
        let (max_width, max_height) = self.get_level_dimensions(level)?;
        let downsample_factor = self.get_level_downsample(level)?;

        let tl_row_this_lvl = utils::to_f64(top_left_lvl0_row)? / downsample_factor;
        let tl_col_this_lvl = utils::to_f64(top_left_lvl0_col)? / downsample_factor;

        let new_height = utils::to_u64(height)?.min(max_height - tl_row_this_lvl.round() as u64);
        let new_width = utils::to_u64(width)?.min(max_width - tl_col_this_lvl.round() as u64);

        if new_height < utils::to_u64(height)? {
            println!(
                "WARNING: Requested region height is changed from {} to {} in order to fit",
                height, new_height
            );
        }
        if new_width < utils::to_u64(width)? {
            println!(
                "WARNING: Requested region width is changed from {} to {} in order to fit",
                width, new_width
            );
        }

        if new_height > max_height {
            return Err(Error::new(ErrorKind::OutOfBounds {
                message: format!("Requested height {} exceeds maximum {}", height, max_height),
            }));
        }

        if new_width > max_width {
            return Err(Error::new(ErrorKind::OutOfBounds {
                message: format!("Requested width {} exceeds maximum {}", width, max_width),
            }));
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
    pub fn read_region<T>(
        &self,
        top_left_lvl0_row: T,
        top_left_lvl0_col: T,
        level: T,
        height: T,
        width: T,
    ) -> Result<RgbaImage, Error>
    where
        T: Integer + Unsigned + ToPrimitive + Debug + Display + Clone + Copy,
    {
        let (height, width) = self.get_feasible_dimensions(
            top_left_lvl0_row,
            top_left_lvl0_col,
            level,
            height,
            width,
        )?;

        let buffer = bindings::read_region(
            self.osr,
            utils::to_i64(top_left_lvl0_col)?,
            utils::to_i64(top_left_lvl0_row)?,
            utils::to_i32(level)?,
            utils::to_i64(width)?,
            utils::to_i64(height)?,
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
        let level = utils::to_u32(level)?;
        if level >= max_num_levels {
            return Err(Error::new(ErrorKind::OutOfBounds {
                message: format!(
                    "Specified level {} is larger than max slide level {}",
                    level,
                    max_num_levels - 1
                ),
            }));
        }
        Ok(())
    }
}
