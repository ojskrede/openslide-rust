//! Properties from various slides
//!

mod openslide;
mod tiff;
mod aperio;
mod hamamatsu;

use std::collections::HashMap;

/// This struct defines an inferface to the various properties of the various formats.
///
/// These properties are also available as a `HashMap<String, String>` which can be obtained with
/// the `OpenSlide::get_properties()` method. However, this way of interacting with the property
/// values is not ideal, hence this struct. Motivation:
///
///   - Get the values with fitting types in stead of Strings, ready to be used straight away.
///   - Every property is easier to document.
///   - More convenient naming (arguable).
///
/// Many formats implements openslide properties (`openslide.<property>` in the HashMap returned by
/// the `OpenSlide::get_properties()` method), and many formats also implements some Tiff
/// properties (`openslide.<property>` in the HashMap returned by the `OpenSlide::get_properties()`
/// method). Then there are properties that are unique to each format (it may be the same
/// properties, but with different naming conventions etc.). This interface gives the programmer
/// access to all (known) properties (if some exists and are not implemented here, this is a bug).
/// If some property does not exist for some slide, the method for this property returns `None`.
/// What properties that are available to each slide is somewhat arbitrary (or, at least unknown to
/// the author of this library per now), so in order to discover available properties, you can
/// print the result of the `OpenSlide::get_properties()` method, or use the
/// `Properties::print_available()` method (recommended).
///
#[derive(Clone, Debug)]
pub struct Properties {
    openslide_properties: openslide::OpenSlide,
    tiff_properties: tiff::Tiff,
    aperio_properties: aperio::Aperio,
}

impl Properties {

    /// Initialises a new `Properties` struct.
    ///
    /// This is done by submitting a property_map, which is obtained from the
    /// `OpenSlide::get_properties()` method, but this is abstracted away from the user, and
    /// happens automatically when defining an `OpenSlide` struct.
    pub fn new(property_map: &HashMap<String, String>) -> Self {
        let mut tiff_properties = tiff::Tiff::default();
        // Openslide properties requires special treatement because we need to find out how many
        // levels there are in the initialization.
        let mut openslide_properties = openslide::OpenSlide::new(property_map);
        let mut aperio_properties = aperio::Aperio::default();

        for (key, value) in property_map {
            let parent = key.split('.').nth(0);
            match parent {
                Some("openslide") => openslide_properties.parse_property_name(key, value),
                Some("tiff") => tiff_properties.parse_property_name(key, value),
                Some("aperio") => aperio_properties.parse_property_name(key, value),
                _ => println!("Could not parse {}", key),
            }
        }

        Properties {
            tiff_properties: tiff_properties,
            openslide_properties: openslide_properties,
            aperio_properties: aperio_properties,
        }
    }

    /// Print available properties (key, value) (where the value is not `None`).
    ///
    /// # OpenSlide properties
    pub fn print_available(&self) {
        self.openslide_properties.print_available();
        self.tiff_properties.print_available();
        self.aperio_properties.print_available();
    }

    // Openslide properties (the markdown header is on the method above)

    /// Micrometer (microns) per pixel in the x (horisontal (TODO: or vertical)) direction.
    pub fn mpp_x(&self) -> Option<f32> {
        self.openslide_properties.mpp_x
    }

    /// Micrometer (microns) per pixel in the y (vertical (TODO: or horisontal)) direction.
    ///
    /// # Tiff properties
    pub fn mpp_y(&self) -> Option<f32> {
        self.openslide_properties.mpp_y
    }

    // Tiff properties (the markdown header is on the method above)

    pub fn image_description(&self) -> Option<String> {
        self.tiff_properties.image_description.clone()
    }

    pub fn software(&self) -> Option<String> {
        self.tiff_properties.software.clone()
    }

    /// Model name
    pub fn model(&self) -> Option<String> {
        self.tiff_properties.model.clone()
    }

    pub fn date_time(&self) -> Option<String> {
        self.tiff_properties.date_time.clone()
    }

    pub fn make(&self) -> Option<String> {
        self.tiff_properties.make.clone()
    }

    /// Resolution in the (TODO: horizontal?) direction
    pub fn x_resolution(&self) -> Option<f32> {
        self.tiff_properties.x_resolution
    }

    /// Resolution in the (TODO: vertical?) direction
    pub fn y_resolution(&self) -> Option<f32> {
        self.tiff_properties.y_resolution
    }

    /// Resolution unit (e.g. centimeter or inch)
    pub fn resolution_unit(&self) -> Option<String> {
        self.tiff_properties.resolution_unit.clone()
    }
}
