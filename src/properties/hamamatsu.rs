//! Hamamatsu properties
//!

#[derive(Clone, Debug)]
struct Slant {
    left_top: Option<(u64, u64, u64)>,
    left_bottom: Option<(u64, u64, u64)>,
    right_top: Option<(u64, u64, u64)>,
    right_bottom: Option<(u64, u64, u64)>,
}

#[derive(Clone, Debug)]
struct FocalPlane {
    left_top: Option<(u64, u64, u64)>,
    left_bottom: Option<(u64, u64, u64)>,
    right_top: Option<(u64, u64, u64)>,
    right_bottom: Option<(u64, u64, u64)>,
}

#[derive(Clone, Debug)]
struct Ahex {
    value: Option<String>,        // [u8; 256] ?
    ploidy: Option<String>,       // [u8; 256] ?
    fluorescence: Option<String>, // [u8; 256] ?
}

#[derive(Clone, Debug)]
struct Exposure {
    barcode_macro: Option<u32>,
    slide_darkfield_macro: Option<u32>,
    slide_macro: Option<u32>,
}

#[derive(Clone, Debug)]
struct Roi {
    barcode_macro: Option<(u32, u32, u32, u32)>,
    slide_macro: Option<(u32, u32, u32, u32)>,
}

#[derive(Clone, Debug)]
struct Valid {
    dltp: Option<u32>,
    ddkp: Option<u32>,
    dshp: Option<u32>,
}

#[derive(Clone, Debug)]
struct Pshv {
    value: Option<u32>,
    magn_10x: Option<u32>,
    magn_40x: Option<u32>,
    ploidy: Option<u32>,
    ploidy_10x: Option<u32>,
    ploidy_40x: Option<u32>,
}

#[derive(Clone, Debug)]
struct ZCoarse {
    value: Option<(u32, u32, u32, u32)>,
}

#[derive(Clone, Debug)]
struct ZFine {
    value: Option<(u32, u32, u32, u32)>,
}

#[derive(Clone, Debug)]
struct Yrnp {
    value: Option<(u32, u32, u32, u32)>,
}

#[derive(Clone, Debug)]
struct Ccd {
    width: Option<u32>,
    width_ploidy: Option<u32>,
    height: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Hamamatsu {
    product: Option<String>,
    system_version: Option<String>,
    updated: Option<String>, // Date
    created: Option<String>, // Date
    color_filter_id: Option<String>,
    reference: Option<String>,
    calibration_version: Option<u32>,
    fine_focus_pitch: Option<u32>,
    coarse_focus_pitch: Option<u32>,
    stage_center: Option<(u32, u32)>,
    slide_thickness: Option<u32>,
    lane_shift_amount: Option<i32>,
    variable_exposure_time: Option<u32>,
    cube_kind: Option<u32>,
    target_white_intensity: Option<u32>,
    source_lens: Option<u32>,
    objective_lens_magnificant: Option<f32>,
    x_offset_from_slide_centre: Option<i64>,
    y_offset_from_slide_centre: Option<i64>,
    macro_s_n: Option<String>,
    ndp_s_n: Option<String>,
    slant: Option<Slant>,
    exposure: Option<Exposure>,
    ahex: Option<Vec<Ahex>>,
    valid: Option<Valid>,
    pshv: Option<Pshv>,
    roi: Option<Roi>,
    z_coarse: Option<Vec<ZCoarse>>,
    z_fine: Option<Vec<ZFine>>,
    focal_plane: Option<FocalPlane>,
    yrnp: Option<Vec<Yrnp>>,
}
