//! Tiff properties
//!

use std::u32;

#[derive(Clone, Debug, Default)]
pub struct Tiff {
    pub software: Option<String>,
    pub model: Option<String>,
    pub x_resolution: Option<u32>,
    pub y_resolution: Option<u32>,
    pub resolution_unit: Option<String>,
    pub make: Option<String>,
    pub date_time: Option<String>, // TODO: date time
    pub image_description: Option<String>,
}

impl Tiff {
    pub fn parse_property_name(&mut self, name: &str, value: &str) {
        match name {
            "tiff.Software" => self.software = Some(String::from(value)),
            "tiff.Model" => self.model = Some(String::from(value)),
            "tiff.XResolution" => self.x_resolution = Some(u32::from_str_radix(value, 10).unwrap()),
            "tiff.YResolution" => self.y_resolution = Some(u32::from_str_radix(value, 10).unwrap()),
            "tiff.ResolutionUnit" => self.resolution_unit = Some(String::from(value)),
            "tiff.Make" => self.make = Some(String::from(value)),
            "tiff.DateTime" => self.date_time = Some(String::from(value)),
            "tiff.ImageDescription" => self.image_description = Some(String::from(value)),
            _ => println!("Could not parse property name {} with value {}", name, value),
        }
    }

    pub fn print_all(&self) {
        match self.image_description {
            Some(ref val) => println!("Image description: {}", val),
            None => {},
        }
        match self.software {
            Some(ref val) => println!("Software: {}", val),
            None => {},
        }
        match self.model {
            Some(ref val) => println!("Model: {}", val),
            None => {},
        }
        match self.date_time {
            Some(ref val) => println!("Date time: {}", val),
            None => {},
        }
        match self.make {
            Some(ref val) => println!("Make: {}", val),
            None => {},
        }
        match self.x_resolution {
            Some(val) => println!("X resolution: {}", val),
            None => {},
        }
        match self.y_resolution {
            Some(val) => println!("Y resolution: {}", val),
            None => {},
        }
        match self.resolution_unit {
            Some(ref val) => println!("Resolution unit: {}", val),
            None => {},
        }
    }
}
