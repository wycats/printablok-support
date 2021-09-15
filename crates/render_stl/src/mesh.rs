use std::fs::File;

use path_abs::{FileRead, PathAbs, PathFile};
use project::Failure;
use rpt::{
    glm::{vec3, TVec3},
    Scene, SceneAdd, Transformable,
};

use crate::{angle::Angle, location::Location, material::Material, rotation::Rotation};

pub enum Transformation {
    Rotate(f64),
    Scale(f64),
}

pub enum MeshSource {
    DynamicFile(PathFile),
    StaticFile(&'static str),
}

impl MeshSource {
    pub const fn constant(filename: &'static str) -> MeshSource {
        MeshSource::StaticFile(filename)
    }

    fn load(self) -> Result<rpt::Mesh, Failure> {
        let path = match self {
            MeshSource::DynamicFile(path) => PathAbs::new(path)?,
            MeshSource::StaticFile(file) => PathAbs::new(file)?,
        };

        let file: File = FileRead::open(path)?.into();
        Ok(rpt::load_stl(file)?)
    }
}
pub struct Mesh {
    source: MeshSource,
    material: Material,
    scale: f64,
    rotate: Rotation,
    translate: Location,
}

impl Mesh {
    pub fn new(source: MeshSource) -> Mesh {
        Mesh {
            source,
            material: Material::DEFAULT,
            scale: 1.0,
            translate: Location::ORIGIN,
            rotate: Rotation::all(Angle::degrees(0.0)),
        }
    }

    pub fn scale(mut self, ratio: impl Into<f64>) -> Mesh {
        self.scale *= ratio.into();
        self
    }

    pub fn rotate(mut self, rotation: impl Into<Rotation>) -> Mesh {
        // TODO: Multiply rotation correctly
        self.rotate = rotation.into();
        self
    }

    pub fn translate(mut self, offset: impl Into<Location>) -> Mesh {
        self.translate = offset.into();
        self
    }

    pub fn material(mut self, material: impl Into<Material>) -> Mesh {
        self.material = material.into();
        self
    }
}

impl Into<Mesh> for MeshSource {
    fn into(self) -> Mesh {
        Mesh::new(self)
    }
}

impl Mesh {}

impl SceneAdd<Mesh> for Scene {
    fn add(&mut self, node: Mesh) {
        let object: rpt::Object = node.into();

        self.add(object);
    }
}

impl Into<rpt::Object> for Mesh {
    fn into(self) -> rpt::Object {
        let material = self.material;
        let mesh = self.source.load().unwrap();
        let mesh = mesh.scale(&scale_all(self.scale));
        let mesh = self.rotate.rotate_rpt(mesh);
        let mesh = mesh.translate(&self.translate.to_offset());

        rpt::Object::new(mesh).material(material.into())
    }
}

fn scale_all(ratio: impl Into<f64>) -> TVec3<f64> {
    let ratio = ratio.into();
    vec3(ratio, ratio, ratio)
}
