use amplify_derive::{From, Wrapper};
use std::f64::consts;

#[derive(Copy, Clone, Default, From, Wrapper)]
pub struct Angle {
    radians: f64,
}

impl Angle {
    pub const ZERO: Angle = Angle { radians: 0f64 };

    #[inline]
    pub fn radians(radians: f64) -> Angle {
        Angle { radians }
    }

    #[inline]
    pub fn degrees(degrees: f64) -> Angle {
        Angle {
            radians: degrees * (consts::PI / 180.0f64),
        }
    }
}
