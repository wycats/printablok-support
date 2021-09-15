use crate::color::Color;

pub struct Material {
    color: Color,
    refraction: f64,
    roughness: f64,
    metallic: f64,
    emittance: f64,
    transparent: bool,
}

impl Material {
    pub const DEFAULT: Material = Material::specular(Color::hex(0xff0000), 0.5);

    pub const fn specular(color: Color, roughness: f64) -> Material {
        Material {
            color,
            refraction: 1.5,
            roughness,
            metallic: 0.0,
            emittance: 0.0,
            transparent: false,
        }
    }
}

impl Into<rpt::Material> for Material {
    fn into(self) -> rpt::Material {
        let Material {
            color,
            refraction,
            roughness,
            metallic,
            emittance,
            transparent,
        } = self;

        rpt::Material {
            color: color.into(),
            index: refraction,
            roughness,
            metallic,
            emittance,
            transparent,
        }
    }
}
