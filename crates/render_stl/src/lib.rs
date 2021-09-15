pub mod angle;
pub mod camera;
pub mod color;
pub mod direction;
pub mod light_source;
pub mod location;
pub mod material;
pub mod mesh;
pub mod model;
pub mod rotation;

pub use crate::mesh::{Mesh, MeshSource};

pub use crate::{
    angle::Angle, color::Color, light_source::LightSource, location::Location, material::Material,
    model::Model, rotation::Rotation,
};

// struct Project {
//     root: PathAbs,
// }

// impl Project {
//     pub fn load(root: impl AsRef<str>) -> Result<Project, Failure> {
//         Ok(Project {
//             root: PathDir::new(root.as_ref())?.into(),
//         })
//     }

//     pub fn load_model(&self, file: impl AsRef<str>) -> Result<Mesh, Box<dyn Error>> {
//         Ok(MeshSource::DynamicFile(self.models_root().join(file.as_ref())).into())
//     }

//     fn models_root(&self) -> PathAbs {
//         self.root.join("data/models")
//     }
// }
