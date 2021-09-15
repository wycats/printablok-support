use rpt::Bounded;

use crate::{angle::Angle, location::RptPosition};

#[derive(Copy, Clone)]
pub struct Rotation {
    x: Angle,
    y: Angle,
    z: Angle,
}

impl Rotation {
    pub const ZERO: Rotation = Rotation {
        x: Angle::ZERO,
        y: Angle::ZERO,
        z: Angle::ZERO,
    };

    // Related to the golden ratio, computed by hand with some geometry
    // from https://github.com/ekzhang/rpt/blob/815b21c23a23fa98dec2b9ea931f71daa2974513/examples/compound.rs
    fn magic() -> f64 {
        ((3.0 * 5.0_f64.sqrt() - 1.0) / 8.0).acos()
    }

    pub const fn all(angle: Angle) -> Rotation {
        Rotation {
            x: angle,
            y: angle,
            z: angle,
        }
    }

    pub const fn x(self, x: Angle) -> Rotation {
        Rotation { x, ..self }
    }

    pub const fn y(self, y: Angle) -> Rotation {
        Rotation { y, ..self }
    }

    pub const fn z(self, z: Angle) -> Rotation {
        Rotation { z, ..self }
    }

    pub(crate) fn rotate_rpt(
        self,
        mesh: rpt::Transformed<rpt::Mesh>,
    ) -> rpt::Transformed<rpt::Mesh> {
        let center_before = center(mesh.bounding_box());

        let after = mesh.rotate(
            -Rotation::magic(),
            &rpt::glm::vec3(self.x.into(), self.y.into(), self.z.into()),
        );

        let center_after = center(after.bounding_box());

        // let move_diff = diff(center_before, center_after);

        after
            .translate(&rpt::glm::vec3(center_before.x - center_after.x, 0.0, 0.0))
            .translate(&rpt::glm::vec3(
                0.0,
                (center_after.y - center_before.y) * 1.5,
                0.0,
            ))
            .translate(&rpt::glm::vec3(0.0, 0.0, center_after.z - center_before.z))
    }
}

fn center(rpt::BoundingBox { p_min, p_max }: rpt::BoundingBox) -> RptPosition {
    let x_size = p_max.x - p_min.x;
    let y_size = p_max.y - p_min.y;
    let z_size = p_max.z - p_min.z;

    let center_x = p_min.x + (x_size / 2.0);
    let center_y = p_min.x + (y_size / 2.0);
    let center_z = p_min.x + (z_size / 2.0);

    rpt::glm::vec3(center_x, center_y, center_z)
}
