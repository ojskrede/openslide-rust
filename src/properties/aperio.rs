//! Aperio properties
//!

use std::{f32, u32};
use num::Num;

#[derive(Clone, Debug, Default)]
pub struct Aperio {
    pub filename: Option<String>,
    pub image_id: Option<String>,
    pub scan_scope_id: Option<String>,
    pub date: Option<String>,
    pub time: Option<String>,
    pub time_zone: Option<String>,
    pub user: Option<String>,
    pub icc_profile: Option<String>,
    pub parmset: Option<String>,
    pub original_height: Option<u32>,
    pub original_width: Option<u32>,
    pub top: Option<f32>,
    pub left: Option<f32>,
    pub mpp: Option<f32>,
    pub line_camera_skew: Option<f32>,
    pub line_area_x_offset: Option<f32>,
    pub line_area_y_offset: Option<f32>,
    pub focus_offset: Option<f32>,
    pub app_mag: Option<u32>,
    pub stripe_width: Option<u32>,
    pub filtered: Option<u32>,
    pub display_color: Option<u32>,
    pub exposure_time: Option<u32>,
    pub exposure_scale: Option<f32>,
    pub sesson_mode: Option<String>,
}

impl Aperio {
    pub fn parse_property_name(&mut self, name: &str, value: &str) {
        match name {
            "aperio.Filename" => self.filename = Some(String::from(value)),
            "aperio.ImageID" => self.image_id = Some(String::from(value)),
            "aperio.ScanScope ID" => self.scan_scope_id = Some(String::from(value)),
            "aperio.Date" => self.date = Some(String::from(value)),
            "aperio.Time" => self.time = Some(String::from(value)),
            "aperio.Time Zone" => self.time_zone = Some(String::from(value)),
            "aperio.User" => self.user = Some(String::from(value)),
            "aperio.ICC Profile" => self.icc_profile = Some(String::from(value)),
            "aperio.Parmset" => self.parmset = Some(String::from(value)),
            "aperio.OriginalHeight" => self.original_height = Some(u32::from_str_radix(value, 10).unwrap()),
            "aperio.OriginalWidth" => self.original_width = Some(u32::from_str_radix(value, 10).unwrap()),
            "aperio.Top" => self.top = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.Left" => self.left = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.MPP" => self.mpp = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.LineCameraSkew" => self.line_camera_skew = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.LineAreaXOffset" => self.line_area_x_offset = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.LineAreaYOffset" => self.line_area_y_offset = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.Focus Offset" => self.focus_offset = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.AppMag" => self.app_mag = Some(u32::from_str_radix(value, 10).unwrap()),
            "aperio.StripeWidth" => self.stripe_width = Some(u32::from_str_radix(value, 10).unwrap()),
            "aperio.Filtered" => self.filtered = Some(u32::from_str_radix(value, 10).unwrap()),
            "aperio.DisplayColor" => self.display_color = Some(u32::from_str_radix(value, 10).unwrap()),
            "aperio.Exposure Time" => self.exposure_time = Some(u32::from_str_radix(value, 10).unwrap()),
            "aperio.Exposure Scale" => self.exposure_scale = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.SessonMode" => self.sesson_mode = Some(String::from(value)),
            _ => println!("Could not parse property name {} and value {}", name, value),
        }
    }

    /// Print available properties (key, value) (where the value is not `None`).
    pub fn print_available(&self) {
        match self.filename {
            Some(ref val) => println!("Filename: {}", val),
            None => {},
        }
        match self.image_id {
            Some(ref val) => println!("Image ID: {}", val),
            None => {},
        }
        match self.scan_scope_id {
            Some(ref val) => println!("ScanScope ID: {}", val),
            None => {},
        }
        match self.date {
            Some(ref val) => println!("Date: {}", val),
            None => {},
        }
        match self.time {
            Some(ref val) => println!("Time: {}", val),
            None => {},
        }
        match self.user {
            Some(ref val) => println!("User: {}", val),
            None => {},
        }
        match self.icc_profile {
            Some(ref val) => println!("ICC Profile: {}", val),
            None => {},
        }
        match self.parmset {
            Some(ref val) => println!("Parmset: {}", val),
            None => {},
        }
        match self.original_height {
            Some(ref val) => println!("Original height: {}", val),
            None => {},
        }
        match self.original_width {
            Some(ref val) => println!("Original width: {}", val),
            None => {},
        }
        match self.top {
            Some(ref val) => println!("Top: {}", val),
            None => {},
        }
        match self.left {
            Some(ref val) => println!("Left: {}", val),
            None => {},
        }
        match self.mpp {
            Some(ref val) => println!("Microns per pixel: {}", val),
            None => {},
        }
        match self.line_camera_skew {
            Some(ref val) => println!("Line camera skew: {}", val),
            None => {},
        }
        match self.line_area_x_offset {
            Some(ref val) => println!("Line area x offset: {}", val),
            None => {},
        }
        match self.line_area_y_offset {
            Some(ref val) => println!("Line area y offset: {}", val),
            None => {},
        }
        match self.focus_offset {
            Some(ref val) => println!("Focus offset: {}", val),
            None => {},
        }
        match self.app_mag {
            Some(ref val) => println!("AppMag: {}", val),
            None => {},
        }
        match self.stripe_width {
            Some(ref val) => println!("Stripe width: {}", val),
            None => {},
        }
        match self.filtered {
            Some(ref val) => println!("Filtered: {}", val),
            None => {},
        }
    }

}
