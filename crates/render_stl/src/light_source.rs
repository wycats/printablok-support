use crate::{color::Color, direction::Direction, location::Location};

pub enum LightSource {
    Ambient(Color),
    Directional(Color, Direction),
    Point(Color, Location),
}

impl LightSource {
    pub fn ambient(color: impl Into<Color>) -> LightSource {
        LightSource::Ambient(color.into())
    }

    pub fn directional(color: impl Into<Color>, direction: impl Into<Direction>) -> LightSource {
        LightSource::Directional(color.into(), direction.into())
    }

    pub fn point(color: impl Into<Color>, location: impl Into<Location>) -> LightSource {
        LightSource::Point(color.into(), location.into())
    }
}

impl rpt::SceneAdd<LightSource> for rpt::Scene {
    fn add(&mut self, node: LightSource) {
        let light: rpt::Light = node.into();

        self.add(light)
    }
}

impl<T> From<T> for LightSource
where
    T: Into<Color>,
{
    fn from(color: T) -> Self {
        LightSource::Ambient(color.into())
    }
}

impl Into<rpt::Light> for LightSource {
    fn into(self) -> rpt::Light {
        match self {
            LightSource::Ambient(color) => rpt::Light::Ambient(color.into()),
            LightSource::Directional(color, direction) => {
                rpt::Light::Directional(color.into(), direction.into())
            }
            LightSource::Point(color, location) => rpt::Light::Point(color.into(), location.into()),
        }
    }
}
