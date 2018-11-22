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
        // TODO: Find elegant way of writing
        //
        // match self.property {
        //     Some(ref val) => println!("Property: {}", val),
        //     None => {},
        // }
        //

        self.filename.clone().map(|val| println!("Filename: {}", val) );
        self.image_id.clone().map(|val| println!("Image ID: {}", val) );
        self.scan_scope_id.clone().map(|val| println!("ScanScope ID: {}", val) );
        self.date.clone().map(|val| println!("Date: {}", val) );
        self.time.clone().map(|val| println!("Time: {}", val) );
        self.time_zone.clone().map(|val| println!("Time: {}", val) );
        self.user.clone().map(|val| println!("User: {}", val) );
        self.icc_profile.clone().map(|val| println!("ICC Profile: {}", val) );
        self.parmset.clone().map(|val| println!("Parmset: {}", val) );
        self.original_height.map(|val| println!("Original height: {}", val) );
        self.original_width.map(|val| println!("Original width: {}", val) );
        self.top.map(|val| println!("Top: {}", val) );
        self.left.map(|val| println!("Left: {}", val) );
        self.mpp.map(|val| println!("Microns per pixel: {}", val) );
        self.line_camera_skew.map(|val| println!("Line camera skew: {}", val) );
        self.line_area_x_offset.map(|val| println!("Line area x offset: {}", val) );
        self.line_area_y_offset.map(|val| println!("Line area y offset: {}", val) );
        self.focus_offset.map(|val| println!("Focus offset: {}", val) );
        self.app_mag.map(|val| println!("AppMag: {}", val) );
        self.stripe_width.map(|val| println!("Stripe width: {}", val) );
        self.filtered.map(|val| println!("Filtered: {}", val) );
        self.display_color.map(|val| println!("Display color: {}", val) );
        self.exposure_time.map(|val| println!("Exposure time: {}", val) );
        self.exposure_scale.map(|val| println!("Exposure scale: {}", val) );
        self.sesson_mode.clone().map(|val| println!("Sesson mode: {}", val) );
    }

}
