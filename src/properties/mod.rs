//! Properties from various slides
//!

mod aperio;
mod openslide;
mod tiff;
//mod hamamatsu;

use std::collections::HashMap;

use self::openslide::LevelProperties;

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
            tiff_properties,
            openslide_properties,
            aperio_properties,
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

    /// Slide vendor
    pub fn vendor(&self) -> Option<String> {
        self.openslide_properties.vendor.clone()
    }

    /// Quickhash 1
    pub fn quickhash_1(&self) -> Option<String> {
        self.openslide_properties.quickhash_1.clone()
    }

    /// Micrometer (microns) per pixel in the x direction.
    pub fn mpp_x(&self) -> Option<f32> {
        // TODO: Replace x / y direction with horisontal / vertical in documentation
        self.openslide_properties.mpp_x
    }

    /// Micrometer (microns) per pixel in the y direction.
    pub fn mpp_y(&self) -> Option<f32> {
        // TODO: Replace x / y direction with horisontal / vertical in documentation
        self.openslide_properties.mpp_y
    }

    /// Objective power
    pub fn objective_power(&self) -> Option<u32> {
        self.openslide_properties.objective_power
    }

    /// Comment
    pub fn comment(&self) -> Option<String> {
        self.openslide_properties.comment.clone()
    }

    /// Number of zoom levels
    pub fn level_count(&self) -> Option<u32> {
        self.openslide_properties.level_count
    }

    /// Vector of level-dependent properties. The position in the returned vector corresponds to
    /// the zoom level.
    ///
    /// # Tiff properties
    pub fn levels(&self) -> Option<Vec<LevelProperties>> {
        self.openslide_properties.levels.clone()
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

    /// Resolution in the x direction
    pub fn x_resolution(&self) -> Option<f32> {
        // TODO: Replace x / y direction with horisontal / vertical in documentation
        self.tiff_properties.x_resolution
    }

    /// Resolution in the y direction
    pub fn y_resolution(&self) -> Option<f32> {
        // TODO: Replace x / y direction with horisontal / vertical in documentation
        self.tiff_properties.y_resolution
    }

    /// Resolution unit (e.g. centimeter or inch)
    ///
    /// # Aperio properties
    pub fn resolution_unit(&self) -> Option<String> {
        self.tiff_properties.resolution_unit.clone()
    }

    // Aperio properties (the markdown header is on the method above)

    /// Slide filename
    pub fn filename(&self) -> Option<String> {
        self.aperio_properties.filename.clone()
    }

    /// Slide image title
    pub fn title(&self) -> Option<String> {
        self.aperio_properties.title.clone()
    }

    /// Slide image id
    pub fn image_id(&self) -> Option<String> {
        self.aperio_properties.image_id.clone()
    }

    /// DSR id
    pub fn dsr_id(&self) -> Option<String> {
        self.aperio_properties.dsr_id.clone()
    }

    /// ScanScope id
    pub fn scan_scope_id(&self) -> Option<String> {
        self.aperio_properties.scan_scope_id.clone()
    }

    /// Date of creation (mm/dd/yy)
    pub fn date(&self) -> Option<String> {
        // TODO: Change this to a rust date type
        self.aperio_properties.date.clone()
    }

    /// Time of creation (hh:mm:ss)
    pub fn time(&self) -> Option<String> {
        // TODO: Change this to a rust time type
        self.aperio_properties.time.clone()
    }

    /// Time zone
    pub fn time_zone(&self) -> Option<String> {
        self.aperio_properties.time_zone.clone()
    }

    /// User
    pub fn user(&self) -> Option<String> {
        self.aperio_properties.user.clone()
    }

    /// ICC profile
    pub fn icc_profile(&self) -> Option<String> {
        self.aperio_properties.icc_profile.clone()
    }

    /// Parmset
    pub fn parmset(&self) -> Option<String> {
        self.aperio_properties.parmset.clone()
    }

    /// Slide height
    pub fn original_height(&self) -> Option<u32> {
        self.aperio_properties.original_height
    }

    /// Slide width
    pub fn original_width(&self) -> Option<u32> {
        self.aperio_properties.original_height
    }

    pub fn top(&self) -> Option<f32> {
        self.aperio_properties.top
    }

    pub fn left(&self) -> Option<f32> {
        self.aperio_properties.left
    }

    /// Micrometer per pixel
    pub fn mpp(&self) -> Option<f32> {
        self.aperio_properties.mpp
    }

    /// Line camera skew
    pub fn line_camera_skew(&self) -> Option<f32> {
        self.aperio_properties.line_camera_skew
    }

    /// Line area offset in horizontal(?) direction
    pub fn line_area_x_offset(&self) -> Option<f32> {
        self.aperio_properties.line_area_x_offset
    }

    /// Line area offset in vertical(?) direction
    pub fn line_area_y_offset(&self) -> Option<f32> {
        self.aperio_properties.line_area_y_offset
    }

    /// Focus offset
    pub fn focus_offset(&self) -> Option<f32> {
        self.aperio_properties.focus_offset
    }

    pub fn app_mag(&self) -> Option<u32> {
        self.aperio_properties.app_mag
    }

    /// Scan stripe width
    pub fn stripe_width(&self) -> Option<u32> {
        self.aperio_properties.stripe_width
    }

    pub fn filtered(&self) -> Option<u32> {
        self.aperio_properties.filtered
    }

    pub fn display_color(&self) -> Option<u32> {
        self.aperio_properties.display_color
    }

    pub fn exposure_time(&self) -> Option<u32> {
        self.aperio_properties.exposure_time
    }

    pub fn exposure_scale(&self) -> Option<f32> {
        self.aperio_properties.exposure_scale
    }

    pub fn sesson_mode(&self) -> Option<String> {
        self.aperio_properties.sesson_mode.clone()
    }
}
