//! This module contains the bindings of the OpenSlide library and its rust wrappers.
//!
//! The api of the wrappers should be consistent with the C API of OpenSlide
//! `https://openslide.org/api/openslide_8h.html`.
//!
//! For a more rust convenient api, use the OpenSlide struct.
//!

use failure::Error;
use libc;
use std::{self, ffi, str};

/// Dummy type for the openslide_t type in OpenSlide
pub enum OpenSlideT {}

#[link(name = "openslide")]
extern "C" {

    // ---------------
    // Basic usage
    // ---------------

    fn openslide_detect_vendor(filename: *const libc::c_char) -> *const libc::c_char;

    fn openslide_open(filename: *const libc::c_char) -> *const OpenSlideT;

    fn openslide_close(osr: *const OpenSlideT) -> libc::c_void;

    fn openslide_get_level_count(osr: *const OpenSlideT) -> libc::int32_t;

    fn openslide_get_level0_dimensions(
        osr: *const OpenSlideT,
        w: *mut libc::int64_t,
        h: *mut libc::int64_t,
    ) -> libc::c_void;

    fn openslide_get_level_dimensions(
        osr: *const OpenSlideT,
        level: libc::int32_t,
        w: *mut libc::int64_t,
        h: *mut libc::int64_t,
    ) -> libc::c_void;

    fn openslide_get_level_downsample(
        osr: *const OpenSlideT,
        level: libc::int32_t,
    ) -> libc::c_double;

    fn openslide_get_best_level_for_downsample(
        slide: *const OpenSlideT,
        downsample_factor: libc::c_double,
    ) -> libc::int32_t;

    fn openslide_read_region(
        osr: *const OpenSlideT,
        dest: *mut libc::uint32_t,
        x: libc::int64_t,
        y: libc::int64_t,
        level: libc::int32_t,
        w: libc::int64_t,
        h: libc::int64_t,
    ) -> libc::c_void;

    // ---------------
    // Error handling
    // ---------------

    // fn openslide_get_error(
    //     osr: *const OpenSlideT
    // ) -> *const libc::c_char;

    // ---------------
    // Properties
    // ---------------

    fn openslide_get_property_names(osr: *const OpenSlideT) -> *const *const libc::c_char;

    fn openslide_get_property_value(
        osr: *const OpenSlideT,
        name: *const libc::c_char,
    ) -> *const libc::c_char;
}

// ---------------
// Basic usage
// ---------------

/// Quickly determine whether a whole slide image is recognized.
pub fn detect_vendor(filename: &str) -> Result<String, Error> {
    let c_filename = ffi::CString::new(filename)?;
    let vendor = unsafe {
        let c_vendor = openslide_detect_vendor(c_filename.as_ptr());
        ffi::CStr::from_ptr(c_vendor).to_string_lossy().into_owned()
    };
    Ok(vendor)
}

/// Open a whole slide image.
pub fn open(filename: &str) -> Result<*const OpenSlideT, Error> {
    let c_filename = ffi::CString::new(filename)?;
    let slide = unsafe { openslide_open(c_filename.as_ptr()) };
    Ok(slide)
}

/// Close an OpenSlide object.
pub unsafe fn close(osr: *const OpenSlideT) {
    openslide_close(osr); // This is unsafe
}

/// Get the number of levels in the whole slide image.
pub unsafe fn get_level_count(osr: *const OpenSlideT) -> Result<i32, Error> {
    let num_levels = openslide_get_level_count(osr); // This is unsafe
    Ok(num_levels)
}

/// Get the dimensions of level 0 (the largest level).
pub unsafe fn get_level0_dimensions(osr: *const OpenSlideT) -> Result<(i64, i64), Error> {
    let mut width: libc::int64_t = 0;
    let mut height: libc::int64_t = 0;
    openslide_get_level0_dimensions(osr, &mut width, &mut height); // This is unsafe
    Ok((width, height))
}

/// Get the dimensions of a level.
pub unsafe fn get_level_dimensions(
    osr: *const OpenSlideT,
    level: i32,
) -> Result<(i64, i64), Error> {
    let mut width: libc::int64_t = 0;
    let mut height: libc::int64_t = 0;
    openslide_get_level_dimensions(osr, level, &mut width, &mut height); // This is unsafe
    Ok((width, height))
}

/// Get the downsampling factor of a given level.
pub unsafe fn get_level_downsample(osr: *const OpenSlideT, level: i32) -> Result<f64, Error> {
    let downsampling_factor = openslide_get_level_downsample(osr, level); // This is unsafe
    Ok(downsampling_factor)
}

/// Get the best level to use for displaying the given downsample.
pub unsafe fn get_best_level_for_downsample(
    osr: *const OpenSlideT,
    downsample: f64,
) -> Result<i32, Error> {
    let level = openslide_get_best_level_for_downsample(osr, downsample); // This is unsafe
    Ok(level)
}

/// Copy pre-multiplied ARGB data from a whole slide image.
pub unsafe fn read_region(
    osr: *const OpenSlideT,
    x: i64,
    y: i64,
    level: i32,
    w: i64,
    h: i64,
) -> Result<Vec<u32>, Error> {
    let mut buffer: Vec<libc::uint32_t> = Vec::with_capacity((h * w) as usize);
    let p_buffer = buffer.as_mut_ptr();
    openslide_read_region(osr, p_buffer, x, y, level, w, h); // This is unsafe
    buffer.set_len((h * w) as usize);
    Ok(buffer)
}

// ---------------
// Error handling
// ---------------

/* FIXME Keep commented as long as it is not working. Gets segmentation fault core dumped
/// Get the current error string.
pub fn get_error(
    osr: *const OpenSlideT
) -> Result<String, Error> {
    let msg = unsafe {
        let c_msg = openslide_get_error(osr);
        ffi::CStr::from_ptr(c_msg).to_string_lossy().into_owned()
    };
    Ok(msg)
}
*/

// ---------------
// Properties
// ---------------

/// Get the NULL-terminated array of property names.
pub unsafe fn get_property_names(osr: *const OpenSlideT) -> Result<Vec<String>, Error> {
    let string_values = {
        let null_terminated_array_ptr = openslide_get_property_names(osr);
        let mut counter = 0;
        let mut loc = null_terminated_array_ptr;
        while !(*loc).is_null() {
            counter += 1;
            loc = loc.offset(1);
        }
        //let c_array = ffi::CStr::from_ptr(null_terminated_array_ptr);
        let values = std::slice::from_raw_parts(null_terminated_array_ptr, counter as usize);
        values
            .iter()
            .map(|&p| ffi::CStr::from_ptr(p)) // iterator of &CStr
            .map(|cs| cs.to_bytes()) // iterator of &[u8]
            .map(|bs| str::from_utf8(bs).unwrap()) // iterator of &str
            .map(|ss| ss.to_owned())
            .collect()
    };
    Ok(string_values)
}

/// Get the value of a single property.
pub unsafe fn get_property_value(osr: *const OpenSlideT, name: &str) -> Result<String, Error> {
    let c_name = ffi::CString::new(name)?;
    let value = {
        let c_value = openslide_get_property_value(osr, c_name.as_ptr());
        ffi::CStr::from_ptr(c_value).to_string_lossy().into_owned()
    };
    Ok(value)
}
