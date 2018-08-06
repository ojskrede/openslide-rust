//! Properties from various slides
//!

pub mod openslide;
mod tiff;
mod aperio;
mod hamamatsu;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Properties {
    openslide_properties: openslide::OpenSlide,
    tiff_properties: tiff::Tiff,
}

impl Properties {
    pub fn new(property_map: &HashMap<String, String>) -> Self {
        let mut tiff_properties = tiff::Tiff::default();
        let mut openslide_properties = openslide::OpenSlide::default();

        for (key, value) in property_map {
            let parent = key.split('.').nth(0);
            match parent {
                //"openslide" => openslide_properties.parse_property_name(key, value),
                Some("tiff") => tiff_properties.parse_property_name(key, value),
                _ => println!("Could not parse {}", key),
            }
        }

        Properties {
            tiff_properties: tiff_properties,
            openslide_properties: openslide_properties,
        }
    }

    /// OpenSlide properties

    pub fn mpp_x(&self) -> Option<f32> {
        self.openslide_properties.mpp_x
    }

    pub fn mpp_y(&self) -> Option<f32> {
        self.openslide_properties.mpp_y
    }

    /// Tiff properties

    pub fn model(&self) -> Option<String> {
        self.tiff_properties.model.clone()
    }

    pub fn resolution_unit(&self) -> Option<String> {
        self.tiff_properties.resolution_unit.clone()
    }
}
