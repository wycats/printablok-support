use amplify_derive::{From, Wrapper};
use rpt::glm::DVec3;

pub type Position = (f64, f64, f64);
pub type RptPosition = rpt::glm::TVec3<f64>;
pub type RptOffset = rpt::glm::DVec3;

#[derive(Default, Wrapper, From)]
pub struct Location {
    inner: Position,
}

impl Location {
    pub const ORIGIN: Location = Location {
        inner: (0f64, 0f64, 0f64),
    };

    pub fn new(x: f64, y: f64, z: f64) -> Location {
        Location { inner: (x, y, z) }
    }

    pub fn to_offset(self) -> RptOffset {
        DVec3::new(self.inner.0, self.inner.1, self.inner.2)
    }
}

impl Into<Location> for (i32, i32, i32) {
    fn into(self) -> Location {
        let (x, y, z) = self;
        Location {
            inner: (x.into(), y.into(), z.into()),
        }
    }
}

impl Into<RptPosition> for Location {
    fn into(self) -> RptPosition {
        let (x, y, z) = self.inner;

        rpt::glm::TVec3::new(x, y, z)
    }
}

pub(crate) fn position_to_rpt(position: Position) -> RptPosition {
    let (x, y, z) = position;

    rpt::glm::TVec3::new(x, y, z)
}
