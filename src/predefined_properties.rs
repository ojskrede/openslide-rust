//! Properties from various slides
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

/// This struct defines an inferface to the various common properties.
///
/// It agrees with the predefined properties in the openslide c api
/// [https://openslide.org/api/openslide_8h.html](https://openslide.org/api/openslide_8h.html).
///
/// These properties are also available as a `HashMap<String, String>` which can be obtained with
/// the `OpenSlide::get_properties()` method. However, this way of interacting with the property
/// values is not ideal, hence this struct. Motivation:
///
///   - Get the values with fitting types in stead of Strings, ready to be used straight away.
///   - Every property is easier to document.
///   - More convenient naming (arguable).
///
///
/// FIXME
/// Many formats implements openslide properties (`openslide.<property>` in the HashMap returned by
/// the `OpenSlide::get_properties()` method), and many formats also implements some Tiff
/// properties (`openslide.<property>` in the HashMap returned by the `OpenSlide::get_properties()`
/// method). You can print the result of the `OpenSlide::get_properties()` method, or use the
/// `Properties::print_available()` method (recommended).
///
/// Common properties that are available under the name `openslide.<property>` in the HashMap
/// returned from the `OpenSlide::get_properties()` method.
#[derive(Clone, Debug)]
pub struct PredefinedProperties {
    pub comment: Option<String>,
    pub vendor: Option<String>,
    pub quickhash_1: Option<String>,
    pub mpp_x: Option<f32>,
    pub mpp_y: Option<f32>,
    pub objective_power: Option<u32>,
    pub level_count: Option<u32>,
    pub levels: Option<Vec<LevelProperties>>,
}

impl PredefinedProperties {
    /// Initialises a new `Properties` struct.
    ///
    /// This is done by submitting a property_map, which is obtained from the
    /// `OpenSlide::get_properties()` method, but this is abstracted away from the user, and
    /// happens automatically when defining an `OpenSlide` struct.
    ///
    /// This needs a property map in order to compute the number of levels. This is needed because
    /// of the properties that are listed as `openslide.level[<level>].<property>`.
    pub fn new(property_map: &HashMap<String, String>) -> Self {
        let comment = property_map
            .get("openslide.comment")
            .map(|v| String::from(v));
        let vendor = property_map
            .get("openslide.vendor")
            .map(|v| String::from(v));
        let quickhash_1 = property_map
            .get("openslide.quickhash-1")
            .map(|v| String::from(v));
        //FIXME let background_color = property_map.get("openslide.background-color").map(|v| String::from(v));
        let objective_power = match property_map.get("openslide.objective-power") {
            Some(v) => {
                match u32::from_str_radix(v, 10) {
                    Ok(x) => Some(x),
                    Err(msg) => {
                        // FIXME
                        println!("{:?}", msg);
                        None
                    }
                }
            }
            None => None,
        };
        let mpp_x = match property_map.get("openslide.mpp-x") {
            Some(v) => {
                match f32::from_str_radix(v, 10) {
                    Ok(x) => Some(x),
                    Err(msg) => {
                        // FIXME
                        println!("{:?}", msg);
                        None
                    }
                }
            }
            None => None,
        };
        let mpp_y = match property_map.get("openslide.mpp-y") {
            Some(v) => {
                match f32::from_str_radix(v, 10) {
                    Ok(x) => Some(x),
                    Err(msg) => {
                        // FIXME
                        println!("{:?}", msg);
                        None
                    }
                }
            }
            None => None,
        };
        // FIXME let bounds_x = property_map.get("openslide.bounds-x");
        // FIXME let bounds_y = property_map.get("openslide.bounds-y");
        //

        // Create a list with LevelProperty, one element per zoom level. This is not a part of the
        // c api, but is added for convenience.
        let mut level_count = match property_map.get("openslide.level-count") {
            Some(v) => {
                match u32::from_str_radix(v, 10) {
                    Ok(x) => Some(x),
                    Err(msg) => {
                        // FIXME
                        println!("{:?}", msg);
                        None
                    }
                }
            }
            None => None,
        };
        let computed_level_count = find_max_level(property_map);
        if level_count != computed_level_count {
            println!("WARNING: Computed level count is different from stated property");
            level_count = computed_level_count
        }

        // Fill levels with default level properties so that it can be filled afterwards in
        // arbitrary order
        let mut levels = level_count.map(|n| vec![LevelProperties::default(); n as usize]);

        // Fill levels with actual values
        for (name, value) in property_map {
            if name.contains("openslide.level[") {
                let level = {
                    let starts_with_number = name.split("level[").last().unwrap();
                    let number_as_string = starts_with_number.split("]").nth(0).unwrap();
                    u32::from_str_radix(number_as_string, 10).unwrap() as usize
                };
                match levels {
                    // TODO: Match name on format("openslide.level[{}].height", level) etc
                    Some(ref mut vec) => {
                        let last_part = name
                            .split(&format!("openslide.level[{}].", level))
                            .last()
                            .unwrap();
                        match last_part {
                            "downsample" => {
                                vec[level].downsample = match f32::from_str_radix(value, 10) {
                                    Ok(x) => Some(x),
                                    Err(_) => None,
                                }
                            }
                            "height" => {
                                vec[level].height = match u32::from_str_radix(value, 10) {
                                    Ok(x) => Some(x),
                                    Err(_) => None,
                                }
                            }
                            "width" => {
                                vec[level].width = match u32::from_str_radix(value, 10) {
                                    Ok(x) => Some(x),
                                    Err(_) => None,
                                }
                            }
                            "tile-height" => {
                                vec[level].tile_height = match u32::from_str_radix(value, 10) {
                                    Ok(x) => Some(x),
                                    Err(_) => None,
                                }
                            }
                            "tile-width" => {
                                vec[level].tile_width = match u32::from_str_radix(value, 10) {
                                    Ok(x) => Some(x),
                                    Err(_) => None,
                                }
                            }
                            _ => println!(
                                "Could not parse property with name {} and value {}",
                                name, value
                            ),
                        }
                    }
                    // Since we have already established that we have n levels via the existence of
                    // "openlide.leve[{}]" in the find_max_level() function
                    None => unreachable!(),
                }
            }
        }

        PredefinedProperties {
            comment,
            vendor,
            quickhash_1,
            mpp_x,
            mpp_y,
            objective_power,
            level_count,
            levels,
        }
    }

    /// Slide vendor
    pub fn vendor(&self) -> Option<String> {
        self.vendor.clone()
    }

    /// Quickhash 1
    pub fn quickhash_1(&self) -> Option<String> {
        self.quickhash_1.clone()
    }

    /// Micrometer (microns) per pixel in the x direction.
    pub fn mpp_x(&self) -> Option<f32> {
        // TODO: Replace x / y direction with horisontal / vertical in documentation
        self.mpp_x
    }

    /// Micrometer (microns) per pixel in the y direction.
    pub fn mpp_y(&self) -> Option<f32> {
        // TODO: Replace x / y direction with horisontal / vertical in documentation
        self.mpp_y
    }

    /// Objective power
    pub fn objective_power(&self) -> Option<u32> {
        self.objective_power
    }

    /// Comment
    pub fn comment(&self) -> Option<String> {
        self.comment.clone()
    }

    /// Number of zoom levels
    pub fn level_count(&self) -> Option<u32> {
        self.level_count
    }

    /// Vector of level-dependent properties. The position in the returned vector corresponds to
    /// the zoom level.
    pub fn levels(&self) -> Option<Vec<LevelProperties>> {
        self.levels.clone()
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
}

/// Find the max level from the `openslide.level[<level>].<level-property>` properties.
fn find_max_level(property_map: &HashMap<String, String>) -> Option<u32> {
    let mut found_levels = Vec::<u32>::new();
    for (key, _) in property_map {
        if key.contains("openslide.level[") {
            let starts_with_number = key.split("openslide.level[").last().unwrap();
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
