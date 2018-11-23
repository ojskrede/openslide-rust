//! Tiff properties
//!

use num::Num;
use std::f32;

#[derive(Clone, Debug, Default)]
pub struct Tiff {
    pub image_description: Option<String>,
    pub software: Option<String>,
    pub model: Option<String>,
    pub date_time: Option<String>, // TODO: date time
    pub make: Option<String>,
    pub x_resolution: Option<f32>,
    pub y_resolution: Option<f32>,
    pub resolution_unit: Option<String>,
}

impl Tiff {
    pub fn parse_property_name(&mut self, name: &str, value: &str) {
        match name {
            "tiff.ImageDescription" => self.image_description = Some(String::from(value)),
            "tiff.Software" => self.software = Some(String::from(value)),
            "tiff.Model" => self.model = Some(String::from(value)),
            "tiff.DateTime" => self.date_time = Some(String::from(value)),
            "tiff.Make" => self.make = Some(String::from(value)),
            "tiff.XResolution" => self.x_resolution = Some(f32::from_str_radix(value, 10).unwrap()),
            "tiff.YResolution" => self.y_resolution = Some(f32::from_str_radix(value, 10).unwrap()),
            "tiff.ResolutionUnit" => self.resolution_unit = Some(String::from(value)),
            _ => println!("Could not parse property name {} and value {}", name, value),
        }
    }

    /// Print available properties (key, value) (where the value is not `None`).
    pub fn print_available(&self) {
        self.image_description
            .clone()
            .map(|val| println!("Image description: {}", val));
        self.software
            .clone()
            .map(|val| println!("Software: {}", val));
        self.model
            .clone()
            .map(|val| println!("Model: {}", val));
        self.date_time
            .clone()
            .map(|val| println!("Date time: {}", val));
        self.make
            .clone()
            .map(|val| println!("Make: {}", val));
        self.x_resolution
            .map(|val| println!("X resolution: {}", val));
        self.y_resolution
            .map(|val| println!("Y resolution: {}", val));
        self.resolution_unit
            .clone()
            .map(|val| println!("Resolution unit: {}", val));
    }
}
