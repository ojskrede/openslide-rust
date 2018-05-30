//! Rust bindings to the OpenSlide C library.
//!

extern crate libc;
extern crate failure;
extern crate image;

/*
pub use bindings::{OpenSlideT,
                   detect_vendor,
                   open,
                   close,
                   get_level_count,
                   get_level0_dimensions,
                   get_level_dimensions,
                   get_level_downsample,
                   get_best_level_for_downsample,
                   read_region,
                   get_error,
                   get_property_names,
                   get_property_value,
};
*/

pub use convenience::{OpenSlide,
};

pub mod bindings;
pub mod utils;
mod convenience;
