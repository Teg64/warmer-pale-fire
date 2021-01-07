#![allow(clippy::declare_interior_mutable_const, dead_code, unused_attributes)]
#![rustfmt::skip]

use once_cell::unsync::Lazy;
use tincture::{Hue, Oklch};

// Colors are from https://github.com/bbatsov/zenburn_emacs/blob/master/zenburn_theme.el

pub(crate) const ZENBURN_FG_PLUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.9957194, 0.020837266, 106.87106));
pub(crate) const ZENBURN_FG: Lazy<Oklch> = Lazy::new(|| oklch(0.8901145, 0.021415712, 106.913895));
pub(crate) const ZENBURN_FG_MINUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.50194794, 0.024493799, 107.29638));

pub(crate) const ZENBURN_BG_MINUS_2: Lazy<Oklch> = Lazy::new(|| oklch(0.0, 0.0, 0.0));
pub(crate) const ZENBURN_BG_MINUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.28908, 0.0, 0.0));
pub(crate) const ZENBURN_BG_MINUS_05: Lazy<Oklch> = Lazy::new(|| oklch(0.34069705, 0.0, 0.0));
pub(crate) const ZENBURN_BG: Lazy<Oklch> = Lazy::new(|| oklch(0.36768097, 0.0, 0.0));
pub(crate) const ZENBURN_BG_PLUS_05: Lazy<Oklch> = Lazy::new(|| oklch(0.4053975, 0.0, 0.0));
pub(crate) const ZENBURN_BG_PLUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.42760777, 0.0, 0.0));
pub(crate) const ZENBURN_BG_PLUS_2: Lazy<Oklch> = Lazy::new(|| oklch(0.4854972, 0.0, 0.0));
pub(crate) const ZENBURN_BG_PLUS_3: Lazy<Oklch> = Lazy::new(|| oklch(0.5417056, 0.0, 0.0));

const RED_CHROMA: f32 = 0.07;
const RED_HUE: f32 = 19.5;
pub(crate) const ZENBURN_RED_PLUS_2: Lazy<Oklch> = Lazy::new(|| oklch(0.8188251, RED_CHROMA, RED_HUE));
pub(crate) const ZENBURN_RED_PLUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.76894647, RED_CHROMA, RED_HUE));
pub(crate) const ZENBURN_RED: Lazy<Oklch> = Lazy::new(|| oklch(0.7183426, RED_CHROMA, RED_HUE));
pub(crate) const ZENBURN_RED_MINUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.6669594, RED_CHROMA, RED_HUE));
pub(crate) const ZENBURN_RED_MINUS_2: Lazy<Oklch> = Lazy::new(|| oklch(0.61474013, RED_CHROMA, RED_HUE));
pub(crate) const ZENBURN_RED_MINUS_3: Lazy<Oklch> = Lazy::new(|| oklch(0.5616291, RED_CHROMA, RED_HUE));
pub(crate) const ZENBURN_RED_MINUS_4: Lazy<Oklch> = Lazy::new(|| oklch(0.5075808, RED_CHROMA, RED_HUE));
pub(crate) const ZENBURN_RED_MINUS_5: Lazy<Oklch> = Lazy::new(|| oklch(0.45258188, RED_CHROMA, RED_HUE));
pub(crate) const ZENBURN_RED_MINUS_6: Lazy<Oklch> = Lazy::new(|| oklch(0.39670327, RED_CHROMA, RED_HUE));

pub(crate) const ZENBURN_ORANGE: Lazy<Oklch> = Lazy::new(|| oklch(0.78973556, 0.0710855, 55.112522));

pub(crate) const ZENBURN_YELLOW: Lazy<Oklch> = Lazy::new(|| oklch(0.9056917, 0.06499734, 90.87234));
pub(crate) const ZENBURN_YELLOW_MINUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.85676247, 0.06566793, 90.86996));
pub(crate) const ZENBURN_YELLOW_MINUS_2: Lazy<Oklch> = Lazy::new(|| oklch(0.8071518, 0.0663619, 90.86176));

const GREEN_CHROMA: f32 = 0.065;
const GREEN_HUE: f32 = 145.0;
pub(crate) const ZENBURN_GREEN_MINUS_5: Lazy<Oklch> = Lazy::new(|| oklch(0.39327696, GREEN_CHROMA, GREEN_HUE));
pub(crate) const ZENBURN_GREEN_MINUS_4: Lazy<Oklch> = Lazy::new(|| oklch(0.4512071, GREEN_CHROMA, GREEN_HUE));
pub(crate) const ZENBURN_GREEN_MINUS_3: Lazy<Oklch> = Lazy::new(|| oklch(0.50761265, GREEN_CHROMA, GREEN_HUE));
pub(crate) const ZENBURN_GREEN_MINUS_2: Lazy<Oklch> = Lazy::new(|| oklch(0.5626681, GREEN_CHROMA, GREEN_HUE));
pub(crate) const ZENBURN_GREEN_MINUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.6165255, GREEN_CHROMA, GREEN_HUE));
pub(crate) const ZENBURN_GREEN: Lazy<Oklch> = Lazy::new(|| oklch(0.6693128, GREEN_CHROMA, GREEN_HUE));
pub(crate) const ZENBURN_GREEN_PLUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.7279649, GREEN_CHROMA, GREEN_HUE));
pub(crate) const ZENBURN_GREEN_PLUS_2: Lazy<Oklch> = Lazy::new(|| oklch(0.7854586, GREEN_CHROMA, GREEN_HUE));
pub(crate) const ZENBURN_GREEN_PLUS_3: Lazy<Oklch> = Lazy::new(|| oklch(0.8419188, GREEN_CHROMA, GREEN_HUE));
pub(crate) const ZENBURN_GREEN_PLUS_4: Lazy<Oklch> = Lazy::new(|| oklch(0.8974478, GREEN_CHROMA, GREEN_HUE));

pub(crate) const ZENBURN_CYAN_MINUS_4: Lazy<Oklch> = Lazy::new(|| oklch(0.45856485, 0.04705447, 195.68336));
pub(crate) const ZENBURN_CYAN_MINUS_3: Lazy<Oklch> = Lazy::new(|| oklch(0.51914865, 0.041178934, 202.58775));
pub(crate) const ZENBURN_CYAN_MINUS_2: Lazy<Oklch> = Lazy::new(|| oklch(0.595962, 0.048358206, 201.39072));
pub(crate) const ZENBURN_CYAN_MINUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.6703748, 0.05529149, 200.55424));
pub(crate) const ZENBURN_CYAN: Lazy<Oklch> = Lazy::new(|| oklch(0.74277675, 0.062019896, 199.93712));
pub(crate) const ZENBURN_CYAN_PLUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.81345516, 0.06857484, 199.46292));
pub(crate) const ZENBURN_CYAN_PLUS_2: Lazy<Oklch> = Lazy::new(|| oklch(0.85783637, 0.07598806, 199.01628));
pub(crate) const ZENBURN_CYAN_PLUS_3: Lazy<Oklch> = Lazy::new(|| oklch(0.87170947, 0.05355117, 200.70901));

pub(crate) const ZENBURN_BLUE_MINUS_1: Lazy<Oklch> = Lazy::new(|| oklch(0.7928748, 0.08759226, 253.56883));
pub(crate) const ZENBURN_BLUE: Lazy<Oklch> = Lazy::new(|| oklch(0.8875058, 0.045272067, 231.736));

pub(crate) const ZENBURN_MAGENTA: Lazy<Oklch> = Lazy::new(|| oklch(0.73893946, 0.1192834, 339.25165));

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h),
    }
}
