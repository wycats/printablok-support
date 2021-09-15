use amplify_derive::{From, Wrapper};
use rpt::glm::DVec3;

#[derive(Wrapper, From, Default)]
pub struct Direction {
    inner: DVec3,
}

impl Into<Direction> for (f64, f64, f64) {
    fn into(self) -> Direction {
        let (x, y, z) = self;
        Direction {
            inner: DVec3::new(x, y, z).normalize(),
        }
    }
}

impl Into<Direction> for (i32, i32, i32) {
    fn into(self) -> Direction {
        let (x, y, z) = self;
        Direction {
            inner: DVec3::new(x as f64, y as f64, z as f64).normalize(),
        }
    }
}
