extern crate image;
extern crate cgmath;

mod scene;
mod render;

use std::fs::OpenOptions;
use image::{DynamicImage, GenericImage, Rgba, ImageFormat};
use cgmath::Point3;
use cgmath::Vector3;

use scene::*;
use render::{Ray, cast_ray};

fn ftou(f: Rgba<f64>) -> Rgba<u8> {
    let r = (f.data[0] * 255.0) as u8;
    let g = (f.data[1] * 255.0) as u8;
    let b = (f.data[2] * 255.0) as u8;
    let a = (f.data[3] * 255.0) as u8;
    Rgba { data: [r, g, b, a] }
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 00..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            image.put_pixel(x, y, ftou(cast_ray(scene, &ray, 0)));
        }
    }
    image
}

pub fn test_render_scene() {
    let scene = Scene {
        width:  scene::RESO_W,
        height: scene::RESO_H,
        fov:    scene::FOV,
        elements: vec!
        [Element::Sphere(Sphere {
            center: Point3::new(0.0, 0.5, -4.0),
            radius: 1.0,
            color : Rgba { data: [1.0, 1.0, 1.0, 1.0] },
            diffuse: 0.0,
            specular: 1.0,
        }),
        Element::Sphere(Sphere {
            center: Point3::new(-3.0, 2.0, -6.0),
            radius: 1.5,
            color : Rgba { data: [1.0, 0.4, 0.4, 1.0] },
            diffuse: 1.0,
            specular: 0.0,
        }),
        Element::Sphere(Sphere {
            center: Point3::new(1.2, 2.0, -4.0),
            radius: 0.7,
            color : Rgba { data: [0.4, 0.4, 1.0, 1.0] },
            diffuse: 1.0,
            specular: 0.0,
        }),
        Element::Plane(Plane {
            origin: Point3::new(0.0, -0.5, 0.0),
            normal: Vector3::new(0.0, -1.0, 0.0),
            color : Rgba { data: [0.1, 0.3, 0.1, 1.0] },
            diffuse: 1.0,
            specular: 0.0,
        }),
        Element::Plane(Plane {
            origin: Point3::new(0.0, 0.0, -20.0),
            normal: Vector3::new(0.0, 0.0, -1.0),
            color : Rgba { data: [0.8, 0.8, 1.0, 1.0] },
            diffuse: 10.0,
            specular: 0.0,
        })],
        lights: vec![
        // Light::Directional(DirectionalLight {
        //     direction: Vector3::new(-1.0, -1.0, 0.3),
        //     color: Rgba { data: [1.0, 1.0, 1.0, 1.0] },
        //     intensity: 1.0,
        //  }),
         Light::Spherical(SphericalLight {
             position: Point3::new(2.0, 7.0, -5.0),
             color: Rgba { data: [1.0, 1.0, 1.0, 1.0] },
             intensity: 500.,
        }),
        //TODO intensity bug
         Light::Spherical(SphericalLight {
             position: Point3::new(2.0, 7.0, 0.0),
             color: Rgba { data: [1.0, 1.0, 1.0, 1.0] },
             intensity: 500.,
        })],
    };

    let img: DynamicImage = render(&scene);
    let mut imgfile = OpenOptions::new().write(true).create(true).open("render.png").unwrap();
    img.save(&mut imgfile, ImageFormat::PNG).expect("some error");
}