//! Openslide properties
//!

use num::Num;
use std::collections::HashMap;
use std::f32;
use std::u32;

/// Properties defined for every level
#[derive(Clone, Debug, Default)]
pub struct LevelProperties {
    downsample: Option<f32>,
    height: Option<u32>,
    width: Option<u32>,
    tile_height: Option<u32>,
    tile_width: Option<u32>,
}

impl LevelProperties {
    /// Print available properties (key, value) (where the value is not `None`).
    ///
    /// # Level properties
    pub fn print_available(&self, level: usize) {
        if let Some(ref val) = self.downsample {
            println!("Level {} downsample factor: {}", level, val)
        }
        if let Some(ref val) = self.height {
            println!("Level {} height: {}", level, val)
        }
        if let Some(ref val) = self.width {
            println!("Level {} width: {}", level, val)
        }
        if let Some(ref val) = self.tile_height {
            println!("Level {} tile height: {}", level, val)
        }
        if let Some(ref val) = self.tile_width {
            println!("Level {} tile width: {}", level, val)
        }
    }

    /// Downsample factor
    pub fn downsample(&self) -> Option<f32> {
        self.downsample
    }

    /// Slide height at this zoom level
    pub fn height(&self) -> Option<u32> {
        self.height
    }

    /// Slide width at this zoom level
    pub fn width(&self) -> Option<u32> {
        self.width
    }

    /// Tile height at this zoom level
    pub fn tile_height(&self) -> Option<u32> {
        self.tile_height
    }

    /// Tile width at this zoom level
    pub fn tile_width(&self) -> Option<u32> {
        self.tile_width
    }
}

/// Common properties that are available under the name `openslide.<property>` in the HashMap
/// returned from the `OpenSlide::get_properties()` method.
#[derive(Clone, Debug)]
pub struct OpenSlide {
    pub vendor: Option<String>,
    pub quickhash_1: Option<String>,
    pub mpp_x: Option<f32>,
    pub mpp_y: Option<f32>,
    pub objective_power: Option<u32>,
    pub comment: Option<String>,
    pub level_count: Option<u32>,
    pub levels: Option<Vec<LevelProperties>>,
}

impl OpenSlide {
    /// Initialises the OpenSlide properties.
    ///
    /// This needs a property map in order to compute the number of levels. This is needed because
    /// of the properties that are listed as `openslide.level[<level>].<property>`.
    pub fn new(property_map: &HashMap<String, String>) -> Self {
        let computed_level_count = find_max_level(property_map);
        let level_count = match property_map.get("openslide.level-count") {
            Some(val) => {
                let level_count = match u32::from_str_radix(val, 10) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                };
                if level_count != computed_level_count {
                    println!("WARNING: Computed level count is different from stated property");
                }
                level_count
            }
            None => computed_level_count,
        };

        // Fill levels with default level properties so that it can be filled afterwards in
        // arbitrary order
        let levels = match level_count {
            Some(num_levels) => {
                let mut level_vec = Vec::<LevelProperties>::new();
                for _ in 0..num_levels {
                    level_vec.push(LevelProperties::default());
                }
                Some(level_vec)
            }
            None => None,
        };

        OpenSlide {
            vendor: None,
            quickhash_1: None,
            mpp_x: None,
            mpp_y: None,
            objective_power: None,
            comment: None,
            level_count,
            levels,
        }
    }

    pub fn parse_property_name(&mut self, name: &str, value: &str) {
        match name {
            "openslide.vendor" => self.vendor = Some(String::from(value)),
            "openslide.quickhash-1" => self.quickhash_1 = Some(String::from(value)),
            "openslide.mpp-x" => self.mpp_x = Some(f32::from_str_radix(value, 10).unwrap()),
            "openslide.mpp-y" => self.mpp_y = Some(f32::from_str_radix(value, 10).unwrap()),
            "openslide.objective-power" => {
                self.objective_power = Some(u32::from_str_radix(value, 10).unwrap())
            }
            "openslide.comment" => self.comment = Some(String::from(value)),
            "openslide.level-count" => {
                self.level_count = Some(u32::from_str_radix(value, 10).unwrap())
            }
            _ => {
                if name.contains("level[") {
                    let level = {
                        let starts_with_number = name.split("level[").last().unwrap();
                        let number_as_string = starts_with_number.split("]").nth(0).unwrap();
                        u32::from_str_radix(number_as_string, 10).unwrap() as usize
                    };
                    match self.levels {
                        Some(ref mut vector) => {
                            let last_part = name
                                .split(&format!("openslide.level[{}].", level))
                                .last()
                                .unwrap();
                            match last_part {
                                "downsample" => {
                                    vector[level].downsample =
                                        Some(f32::from_str_radix(value, 10).unwrap())
                                }
                                "height" => {
                                    vector[level].height =
                                        Some(u32::from_str_radix(value, 10).unwrap())
                                }
                                "width" => {
                                    vector[level].width =
                                        Some(u32::from_str_radix(value, 10).unwrap())
                                }
                                "tile-height" => {
                                    vector[level].tile_height =
                                        Some(u32::from_str_radix(value, 10).unwrap())
                                }
                                "tile-width" => {
                                    vector[level].tile_width =
                                        Some(u32::from_str_radix(value, 10).unwrap())
                                }
                                //_ => println!(
                                //    "Could not parse property with name {} and value {}",
                                //    name, value
                                //),
                                _ => {},
                            }
                        }
                        None => println!("hello"),
                    }
                } else {
                    println!(
                        "Could not parse property with name {} and value {}",
                        name, value
                    );
                }
            }
        }
    }

    /// Print available properties (key, value) (where the value is not `None`).
    pub fn print_available(&self) {
        if let Some(ref val) = self.vendor {
            println!("Vendor: {}", val)
        }
        if let Some(ref val) = self.quickhash_1 {
            println!("Quickhash 1: {}", val)
        }
        if let Some(ref val) = self.mpp_x {
            println!("Microns per pixel x: {}", val)
        }
        if let Some(ref val) = self.mpp_y {
            println!("Microns per pixel y: {}", val)
        }
        if let Some(ref val) = self.objective_power {
            println!("Objective power: {}", val)
        }
        if let Some(ref val) = self.comment {
            println!("Comment: {}", val)
        }
        if let Some(ref val) = self.level_count {
            println!("Number of levels: {}", val)
        }
        if let Some(ref val) = self.levels {
            for (number, level) in val.iter().enumerate() {
                level.print_available(number);
            }
        }
    }

    // TODO: Consider implementing getter functions and make struct variables private.
}

/// Find the max level from the `openslide.level[<level>].<level-property>` properties.
fn find_max_level(property_map: &HashMap<String, String>) -> Option<u32> {
    let mut found_levels = Vec::<u32>::new();
    for (key, _) in property_map {
        if key.contains("level[") {
            let starts_with_number = key.split("level[").last().unwrap();
            let number_as_string = starts_with_number.split("]").nth(0).unwrap();
            match u32::from_str_radix(number_as_string, 10) {
                Ok(val) => found_levels.push(val),
                Err(_) => {}
            }
        }
    }
    match found_levels.iter().max() {
        Some(val) => Some(*val + 1),
        None => None,
    }
}
