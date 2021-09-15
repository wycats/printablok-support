use crate::location::{position_to_rpt, Position};

pub struct Camera {
    eye: Position,
    direction: Position,
    up: Position,
    fov: f64,
    aperture: f64,
    focal_distance: f64,
}

// const SIXTH_CIRCLE: f64 = std::f64::consts::FRAC_PI_6;
const EIGHTH_CIRCLE: f64 = std::f64::consts::FRAC_PI_8;

impl Camera {
    const DEFAULT: Camera = Self {
        eye: (0.0, 0.0, 10.0),
        direction: (0.0, 0.0, -1.0),
        up: (0.0, 1.0, 0.0),
        fov: EIGHTH_CIRCLE,
        aperture: 0.0,
        focal_distance: 0.0,
    };
}

// Cribbed from the implementation of Default for rpt::Camera.
impl Default for Camera {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Into<rpt::Camera> for Camera {
    fn into(self) -> rpt::Camera {
        rpt::Camera {
            eye: position_to_rpt(self.eye),
            direction: position_to_rpt(self.direction),
            up: position_to_rpt(self.up),
            fov: self.fov,
            aperture: self.aperture,
            focal_distance: self.focal_distance,
        }
    }
}
