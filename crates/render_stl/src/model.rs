use std::io::Write;

use project::{Nothing, Outcome};
use rpt::image::{DynamicImage, ImageOutputFormat, RgbImage};
use rpt::{Scene, SceneAdd};

use crate::camera::Camera;
use crate::light_source::LightSource;
use crate::location::Location;
use crate::material::Material;
use crate::mesh::Mesh;
use crate::rotation::Rotation;

pub struct Model {
    scene: Scene,
    camera: Camera,
    mesh: Mesh,
}

impl Model {
    pub fn new(mesh: impl Into<Mesh>) -> Model {
        Model {
            scene: Scene::new(),
            camera: Camera::default(),
            mesh: mesh.into(),
        }
    }

    pub fn add_light(mut self, light_source: impl Into<LightSource>) -> Self {
        let light_source = light_source.into();

        self.scene.add(light_source);
        self
    }

    pub fn scale(mut self, ratio: f64) -> Self {
        self.mesh = self.mesh.scale(ratio);
        self
    }

    pub fn rotate(mut self, rotation: Rotation) -> Self {
        self.mesh = self.mesh.rotate(rotation);
        self
    }

    pub fn translate(mut self, offset: impl Into<Location>) -> Self {
        self.mesh = self.mesh.translate(offset);
        self
    }

    pub fn material(mut self, material: impl Into<Material>) -> Self {
        self.mesh = self.mesh.material(material);
        self
    }

    pub fn render(self, target: &mut impl Write) -> Outcome {
        let Self {
            mut scene,
            camera,
            mesh,
        } = self;

        scene.add(mesh);

        // scene.add(Light::Point(
        //     glm::vec3(80.0, 80.0, 80.0),
        //     glm::vec3(0.0, 5.0, 5.0),
        // ));

        // scene.add(Light::Directional(
        //     glm::vec3(2.0, 2.0, 2.0),
        //     glm::vec3(1.0, -1.0, 0.0).normalize(),
        // ));

        let image = rpt::Renderer::new(&scene, camera.into())
            .max_bounces(4)
            .num_samples(5)
            .exposure_value(2.5)
            .width(400)
            .height(300)
            .render();

        encode(image, target)
    }
}

pub fn encode(image: RgbImage, target: &mut impl Write) -> Outcome {
    let image = DynamicImage::ImageRgb8(image);
    image.write_to(target, ImageOutputFormat::Png)?;

    Ok(Nothing)
}
