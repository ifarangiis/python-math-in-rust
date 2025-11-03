#[allow(dead_code)]
unsafe extern "C" {
    pub fn acos(n: f64) -> f64;
    pub fn asin(n: f64) -> f64;
    pub fn atan(n: f64) -> f64;
    pub fn atan2(a: f64, b: f64) -> f64;
    pub fn cbrt(n: f64) -> f64;
    pub fn cbrtf(n: f32) -> f32;
    pub fn cosh(n: f64) -> f64;
    pub fn expm1(n: f64) -> f64;
    pub fn expm1f(n: f32) -> f32;
    pub fn fdim(a: f64, b: f64) -> f64;
    pub fn fdimf(a: f32, b: f32) -> f32;
    #[cfg_attr(target_env = "msvc", link_name = "_hypot")]
    pub fn hypot(x: f64, y: f64) -> f64;
    #[cfg_attr(target_env = "msvc", link_name = "_hypotf")]
    pub fn hypotf(x: f32, y: f32) -> f32;
    pub fn log1p(n: f64) -> f64;
    pub fn log1pf(n: f32) -> f32;
    pub fn sinh(n: f64) -> f64;
    pub fn tan(n: f64) -> f64;
    pub fn tanh(n: f64) -> f64;
    pub fn tgamma(n: f64) -> f64;
    pub fn tgammaf(n: f32) -> f32;
    pub fn lgamma_r(n: f64, s: &mut i32) -> f64;
    #[cfg(not(target_os = "aix"))]
    pub fn lgammaf_r(n: f32, s: &mut i32) -> f32;
    pub fn erf(n: f64) -> f64;
    pub fn erff(n: f32) -> f32;
    pub fn erfc(n: f64) -> f64;
    pub fn erfcf(n: f32) -> f32;
}
