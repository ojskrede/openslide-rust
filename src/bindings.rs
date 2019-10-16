//! This module contains the bindings of the OpenSlide library and its rust wrappers.
//!
//! The api of the wrappers should be consistent with the C API of OpenSlide
//! `https://openslide.org/api/openslide_8h.html`.
//!
//! For a more rust convenient api, use the OpenSlide struct.
//!

use failure::{format_err, Error};
use libc;
use std::{self, ffi, str};

/// Dummy type for the opaque struct openslide_t type in OpenSlide. See
///
/// https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
///
#[repr(C)]
pub struct OpenSlideType {
    _private: [u8; 0],
}

#[link(name = "openslide")]
extern "C" {

    // ---------------
    // Basic usage
    // ---------------

    fn openslide_detect_vendor(filename: *const libc::c_char) -> *const libc::c_char;

    fn openslide_open(filename: *const libc::c_char) -> *const OpenSlideType;

    fn openslide_close(osr: *const OpenSlideType) -> libc::c_void;

    fn openslide_get_level_count(osr: *const OpenSlideType) -> libc::int32_t;

    fn openslide_get_level0_dimensions(
        osr: *const OpenSlideType,
        w: *mut libc::int64_t,
        h: *mut libc::int64_t,
    ) -> libc::c_void;

    fn openslide_get_level_dimensions(
        osr: *const OpenSlideType,
        level: libc::int32_t,
        w: *mut libc::int64_t,
        h: *mut libc::int64_t,
    ) -> libc::c_void;

    fn openslide_get_level_downsample(
        osr: *const OpenSlideType,
        level: libc::int32_t,
    ) -> libc::c_double;

    fn openslide_get_best_level_for_downsample(
        slide: *const OpenSlideType,
        downsample_factor: libc::c_double,
    ) -> libc::int32_t;

    fn openslide_read_region(
        osr: *const OpenSlideType,
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

    fn openslide_get_error(osr: *const OpenSlideType) -> *const libc::c_char;

    // ---------------
    // Properties
    // ---------------

    fn openslide_get_property_names(osr: *const OpenSlideType) -> *const *const libc::c_char;

    fn openslide_get_property_value(
        osr: *const OpenSlideType,
        name: *const libc::c_char,
    ) -> *const libc::c_char;
}

// ---------------
// Basic usage
// ---------------

// NOTE about error handling
//
// From https://github.com/openslide/openslide/blob/master/src/openslide.h about the function
//
// const char *openslide_get_error(openslide_t *osr);
//
// > For a given OpenSlide object, once this function returns a non-NULL
// > value, the only useful operation on the object is to call
// > openslide_close() to free its resources.
//
// That is:
//
// openslide_close(osr);
//
// After each call we check the status of the C OpenSlide object with the above function. If it is
// not NULL, we return immediately with and error. This then calls the Drop trait implemented for
// the OpenSlide object, which involves calling openslide_close().
//

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
pub fn open(filename: &str) -> Result<*const OpenSlideType, Error> {
    let c_filename = ffi::CString::new(filename)?;
    let osr = unsafe { openslide_open(c_filename.as_ptr()) };
    if let Some(err) = get_error(osr) {
        return Err(format_err!(
            "In function open: Non-NULL error state from openslide:\n\n{}\n\n",
            err
        ));
    }
    Ok(osr)
}

/// Close an OpenSlide object.
pub fn close(osr: *const OpenSlideType) {
    dbg!("Calling close");
    unsafe {
        openslide_close(osr);
    }
}

/// Get the number of levels in the whole slide image.
pub fn get_level_count(osr: *const OpenSlideType) -> Result<i32, Error> {
    let num_levels = unsafe { openslide_get_level_count(osr) };
    if let Some(err) = get_error(osr) {
        return Err(format_err!(
            "In function get_level_count: Non-NULL error state from openslide:\n\n{}\n\n",
            err
        ));
    }
    Ok(num_levels)
}

/// Get the dimensions of level 0 (the largest level).
pub fn get_level0_dimensions(osr: *const OpenSlideType) -> Result<(i64, i64), Error> {
    let mut width: libc::int64_t = 0;
    let mut height: libc::int64_t = 0;
    unsafe {
        openslide_get_level0_dimensions(osr, &mut width, &mut height);
    }
    if let Some(err) = get_error(osr) {
        return Err(format_err!(
            "In function get_level0_dimensions: Non-NULL error state from openslide:\n\n{}\n\n",
            err
        ));
    }
    Ok((width, height))
}

/// Get the dimensions of a level.
pub fn get_level_dimensions(osr: *const OpenSlideType, level: i32) -> Result<(i64, i64), Error> {
    let mut width: libc::int64_t = 0;
    let mut height: libc::int64_t = 0;
    unsafe {
        openslide_get_level_dimensions(osr, level, &mut width, &mut height);
    }
    if let Some(err) = get_error(osr) {
        return Err(format_err!(
            "In function get_level_dimensions: Non-NULL error state from openslide:\n\n{}\n\n",
            err
        ));
    }
    Ok((width, height))
}

/// Get the downsampling factor of a given level.
pub fn get_level_downsample(osr: *const OpenSlideType, level: i32) -> Result<f64, Error> {
    let downsampling_factor = unsafe { openslide_get_level_downsample(osr, level) };
    if let Some(err) = get_error(osr) {
        return Err(format_err!(
            "In function get_level_downsample: Non-NULL error state from openslide:\n\n{}\n\n",
            err
        ));
    }
    Ok(downsampling_factor)
}

/// Get the best level to use for displaying the given downsample.
pub fn get_best_level_for_downsample(
    osr: *const OpenSlideType,
    downsample: f64,
) -> Result<i32, Error> {
    let level = unsafe { openslide_get_best_level_for_downsample(osr, downsample) };
    if let Some(err) = get_error(osr) {
        return Err(format_err!(
            "In function get_best_level_for_downsample: Non-NULL error state from openslide:\n\n{}\n\n",
            err
        ));
    }
    Ok(level)
}

/// Copy pre-multiplied ARGB data from a whole slide image.
pub fn read_region(
    osr: *const OpenSlideType,
    x: i64,
    y: i64,
    level: i32,
    w: i64,
    h: i64,
) -> Result<Vec<u32>, Error> {
    let mut buffer: Vec<libc::uint32_t> = Vec::with_capacity((h * w) as usize);
    let p_buffer = buffer.as_mut_ptr();
    unsafe {
        openslide_read_region(osr, p_buffer, x, y, level, w, h);
    }
    if let Some(err) = get_error(osr) {
        return Err(format_err!(
            "In function read_region: Non-NULL error state from openslide:\n\n{}\n\n",
            err
        ));
    }
    unsafe {
        buffer.set_len((h * w) as usize);
    }
    Ok(buffer)
}

// ---------------
// Errors
// ---------------

/// Get error state of the slide. If it is null everything is ok and this function returns None. If
/// else it returns Some(error message).
pub fn get_error(osr: *const OpenSlideType) -> Option<String> {
    let mut return_val: Option<String> = None;
    unsafe {
        let error_state = openslide_get_error(osr);
        if !error_state.is_null() {
            return_val = Some(ffi::CStr::from_ptr(error_state).to_string_lossy().into_owned());
        }
    }
    return_val
}

// ---------------
// Properties
// ---------------

/// Get the NULL-terminated array of property names.
pub fn get_property_names(osr: *const OpenSlideType) -> Result<Vec<String>, Error> {
    let string_values = {
        let null_terminated_array_ptr = unsafe { openslide_get_property_names(osr) };
        if let Some(err) = get_error(osr) {
            return Err(format_err!(
                "In function get_property_names: Non-NULL error state from openslide:\n\n{}\n\n",
                err
            ));
        }
        let mut counter = 0;
        let mut loc = null_terminated_array_ptr;
        unsafe {
            while !(*loc).is_null() {
                counter += 1;
                loc = loc.offset(1);
            }
        }
        //let c_array = ffi::CStr::from_ptr(null_terminated_array_ptr);
        unsafe {
            let values = std::slice::from_raw_parts(null_terminated_array_ptr, counter as usize);
            values
                .iter()
                .map(|&p| ffi::CStr::from_ptr(p)) // iterator of &CStr
                .map(|cs| cs.to_bytes()) // iterator of &[u8]
                .map(|bs| str::from_utf8(bs).unwrap()) // iterator of &str
                .map(|ss| ss.to_owned())
                .collect()
        }
    };
    Ok(string_values)
}

/// Get the value of a single property.
pub fn get_property_value(osr: *const OpenSlideType, name: &str) -> Result<String, Error> {
    let c_name = ffi::CString::new(name)?;
    let value = unsafe {
        let c_value = openslide_get_property_value(osr, c_name.as_ptr());
        ffi::CStr::from_ptr(c_value).to_string_lossy().into_owned()
    };
    if let Some(err) = get_error(osr) {
        return Err(format_err!(
            "In function get_property_value: Non-NULL error state from openslide:\n\n{}\n\n",
            err
        ));
    }
    Ok(value)
}
