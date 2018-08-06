//! Openslide properties
//!

#[derive(Clone, Debug, Default)]
struct LevelProperties {
    level: Option<u32>,
    downsample: Option<f32>,
    height: Option<u32>,
    width: Option<u32>,
    tile_height: Option<u32>,
    tile_width: Option<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct OpenSlide {
    level: Option<u32>,
    levels: Option<Vec<LevelProperties>>,
    vendor: Option<String>,
    quickhash_1: Option<String>,
    level_count: Option<u32>,
    objective_power: Option<u32>,
    pub mpp_x: Option<f32>,
    pub mpp_y: Option<f32>,
    comment: Option<String>,
}

/*
impl OpenSlide {
    fn parse_property_name(&mut self, name: &str, value: &str) {
        match name {
            "openslide.level-count" => self.level_count = Some(String::from(value)),
        }
    }
}
*/
