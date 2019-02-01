//! Aperio properties
//!

use num::Num;
use std::{f32, u32};

#[derive(Clone, Debug, Default)]
pub struct Aperio {
    pub filename: Option<String>,
    pub title: Option<String>,
    pub image_id: Option<String>,
    pub dsr_id: Option<String>,
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
            "aperio.Title" => self.title = Some(String::from(value)),
            "aperio.ImageID" => self.image_id = Some(String::from(value)),
            "aperio.DSR ID" => self.dsr_id = Some(String::from(value)),
            "aperio.ScanScope ID" => self.scan_scope_id = Some(String::from(value)),
            "aperio.Date" => self.date = Some(String::from(value)),
            "aperio.Time" => self.time = Some(String::from(value)),
            "aperio.Time Zone" => self.time_zone = Some(String::from(value)),
            "aperio.User" => self.user = Some(String::from(value)),
            "aperio.ICC Profile" => self.icc_profile = Some(String::from(value)),
            "aperio.Parmset" => self.parmset = Some(String::from(value)),
            "aperio.OriginalHeight" => {
                self.original_height = Some(u32::from_str_radix(value, 10).unwrap())
            }
            "aperio.OriginalWidth" => {
                self.original_width = Some(u32::from_str_radix(value, 10).unwrap())
            }
            "aperio.Top" => self.top = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.Left" => self.left = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.MPP" => self.mpp = Some(f32::from_str_radix(value, 10).unwrap()),
            "aperio.LineCameraSkew" => {
                self.line_camera_skew = Some(f32::from_str_radix(value, 10).unwrap())
            }
            "aperio.LineAreaXOffset" => {
                self.line_area_x_offset = Some(f32::from_str_radix(value, 10).unwrap())
            }
            "aperio.LineAreaYOffset" => {
                self.line_area_y_offset = Some(f32::from_str_radix(value, 10).unwrap())
            }
            "aperio.Focus Offset" => {
                self.focus_offset = Some(f32::from_str_radix(value, 10).unwrap())
            }
            "aperio.AppMag" => self.app_mag = Some(u32::from_str_radix(value, 10).unwrap()),
            "aperio.StripeWidth" => {
                self.stripe_width = Some(u32::from_str_radix(value, 10).unwrap())
            }
            "aperio.Filtered" => self.filtered = Some(u32::from_str_radix(value, 10).unwrap()),
            "aperio.DisplayColor" => {
                self.display_color = Some(u32::from_str_radix(value, 10).unwrap())
            }
            "aperio.Exposure Time" => {
                self.exposure_time = Some(u32::from_str_radix(value, 10).unwrap())
            }
            "aperio.Exposure Scale" => {
                self.exposure_scale = Some(f32::from_str_radix(value, 10).unwrap())
            }
            "aperio.SessonMode" => self.sesson_mode = Some(String::from(value)),
            //_ => println!("Could not parse property name {} and value {}", name, value),
            _ => {},
        }
    }

    /// Print available properties (key, value) (where the value is not `None`).
    pub fn print_available(&self) {
        if let Some(ref val) = self.filename {
            println!("Filename: {}", val)
        }
        if let Some(ref val) = self.title {
            println!("Title: {}", val)
        }
        if let Some(ref val) = self.image_id {
            println!("Image ID: {}", val)
        }
        if let Some(ref val) = self.dsr_id {
            println!("DSR ID: {}", val)
        }
        if let Some(ref val) = self.scan_scope_id {
            println!("ScanScope ID: {}", val)
        }
        if let Some(ref val) = self.date {
            println!("Date: {}", val)
        }
        if let Some(ref val) = self.time {
            println!("Time: {}", val)
        }
        if let Some(ref val) = self.time_zone {
            println!("Time: {}", val)
        }
        if let Some(ref val) = self.user {
            println!("User: {}", val)
        }
        if let Some(ref val) = self.icc_profile {
            println!("ICC Profile: {}", val)
        }
        if let Some(ref val) = self.parmset {
            println!("Parmset: {}", val)
        }
        if let Some(ref val) = self.original_height {
            println!("Original height: {}", val)
        }
        if let Some(ref val) = self.original_width {
            println!("Original width: {}", val)
        }
        if let Some(ref val) = self.top {
            println!("Top: {}", val)
        }
        if let Some(ref val) = self.left {
            println!("Left: {}", val)
        }
        if let Some(ref val) = self.mpp {
            println!("Microns per pixel: {}", val)
        }
        if let Some(ref val) = self.line_camera_skew {
            println!("Line camera skew: {}", val)
        }
        if let Some(ref val) = self.line_area_x_offset {
            println!("Line area x offset: {}", val)
        }
        if let Some(ref val) = self.line_area_y_offset {
            println!("Line area y offset: {}", val)
        }
        if let Some(ref val) = self.focus_offset {
            println!("Focus offset: {}", val)
        }
        if let Some(ref val) = self.app_mag {
            println!("AppMag: {}", val)
        }
        if let Some(ref val) = self.stripe_width {
            println!("Stripe width: {}", val)
        }
        if let Some(ref val) = self.filtered {
            println!("Filtered: {}", val)
        }
        if let Some(ref val) = self.display_color {
            println!("Display color: {}", val)
        }
        if let Some(ref val) = self.exposure_time {
            println!("Exposure time: {}", val)
        }
        if let Some(ref val) = self.exposure_scale {
            println!("Exposure scale: {}", val)
        }
        if let Some(ref val) = self.sesson_mode {
            println!("Sesson mode: {}", val)
        }
    }
}
