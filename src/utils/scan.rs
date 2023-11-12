
pub struct RadarCell {
    pub groundscatter: i32,             // gsct in RST
    pub power_lag_zero: f64,            // p_0 in RST
    pub power_error_lag_zero: f64,      // p_0_e in RST
    pub velocity: f64,                  // v in RST
    pub velocity_error: f64,            // v_e in RST
    pub spectral_width_lin: f64,        // w_l in RST
    pub spectral_width_lin_error: f64,  // w_l_e in RST
    pub power_lin: f64,                 // p_l in RST
    pub power_lin_error: f64,           // p_l_e in RST
    pub phi_zero: f64,                  // phi0 in RST
    pub elevation: f64                  // elv in RST
}

pub struct RadarBeam {
    pub scan: i32,                  // scan in RST
    pub beam: i32,                  // bm in RST
    pub beam_azimuth: f32,          // bmazm in RST
    pub time: f64,                  // time in RST
    pub program_id: i32,            // cpid in RST
    pub integration_time_s: i32,    // intt.sc in RST
    pub integration_time_us: i32,   // intt.us in RST
    pub num_averages: i32,          // nave in RST
    pub first_range: i32,           // frang in RST
    pub range_sep: i32,             // rsep in RST
    pub rx_rise: i32,               // rxrise in RST
    pub freq: i32,                  // freq in RST
    pub noise: i32,                 // noise in RST
    pub attenuation: i32,           // atten in RST
    pub channel: i32,               // channel in RST
    pub num_ranges: i32,            // nrang in RST
    pub sct: Vec<u8>,               // sct in RST
    pub cells: Vec<RadarCell>,      // rng in RST
}

pub struct RadarScan {
    pub station_id: i32,        // stid in RST
    pub version_major: i32,     // version.major in RST
    pub version_minor: i32,     // version.minor in RST
    pub start_time: f64,        // st_time in RST
    pub end_time: f64,          // ed_time in RST
    pub num: i32,               // num in RST
    pub beams: Vec<RadarBeam>,  // bm in RST
}