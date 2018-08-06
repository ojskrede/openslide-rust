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
    value: Option<String>, // [u8; 256] ?
    ploidy: Option<String>, // [u8; 256] ?
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
    slant: Option<Slant>,
    fine_focus_pitch: Option<u32>,
    coarse_focus_pitch: Option<u32>,
    ahex: Option<Vec<Ahex>>,
    exposure: Option<Exposure>,
    valid: Option<Valid>,
    target_white_intensity: Option<u32>,
    source_lens: Option<u32>,
    pshv: Option<Pshv>,
    product: Option<String>,
    roi: Option<Roi>,
    z_coarse: Option<Vec<ZCoarse>>,
    z_fine: Option<Vec<ZFine>>,
    system_version: Option<String>,
    x_offset_from_slide_centre: Option<i64>,
    y_offset_from_slide_centre: Option<i64>,
    macro_s_n: Option<String>,
    ndp_s_n: Option<String>,
    focal_plane: Option<FocalPlane>,
    objective_lens_magnificant: Option<f32>,
    calibration_version: Option<u32>,
    lane_shift_amount: Option<i32>,
    variable_exposure_time: Option<u32>,
    cube_kind: Option<u32>,
    reference: Option<String>,
    yrnp: Option<Vec<Yrnp>>,
    color_filter_id: Option<String>,
    stage_center: Option<(u32, u32)>,
    updated: Option<String>, // Date
    created: Option<String>, // Date
    slide_thickness: Option<u32>,
}
