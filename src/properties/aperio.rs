//! Aperio properties
//!



#[derive(Clone, Debug)]
pub struct Aperio {
    line_camera_skew: f32,
    parmset: String,
    focus_offset: f32,
    app_mag: u32,
    filename: String,
    original_height: u32,
    original_width: u32,
    scan_scope_id: String,
    top: f32,
    left: f32,
    line_area_x_offset: f32,
    line_area_y_offset: f32,
    date: String,
    time: String,
    stripe_width: u32,
    mpp: f32,
    filtered: u32,
    icc_profile: String,
    user: String,
    image_id: String,
}
