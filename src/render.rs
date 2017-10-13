use cgmath::prelude::*;
use cgmath::Point3;
use cgmath::Vector3;
use image::Rgba;
use std::f64::consts::PI;

use scene::*;

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        let sensor_x = ((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0;
        let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0;
        assert!(scene.width > scene.height);
        let fov_adj = (scene.fov.to_radians() / 2.0).tan();
        let sensor_x = sensor_x * ASPECT * fov_adj;
        let sensor_y = sensor_y * fov_adj;
        
        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(sensor_x, sensor_y, -1.0).normalize(),
        }
    }
}

//TODO implement overloaded mul, add image::Rgba
fn scale_colors(ec: &Rgba<f64>, lc: &Rgba<f64>, factor: f64) -> Rgba<f64> {
    let r = ec.data[0] * lc.data[0] * factor;
    let g = ec.data[1] * lc.data[1] * factor;
    let b = ec.data[2] * lc.data[2] * factor;
    let a = ec.data[3] * lc.data[3] * factor;
    Rgba { data: [r, g, b, a]}
}
fn accumulate_color(acc: &Rgba<f64>, lit: &Rgba<f64>) -> Rgba<f64> {
    let r = (acc.data[0] + lit.data[0]).max(0.).min(1.);
    let g = (acc.data[1] + lit.data[1]).max(0.).min(1.);
    let b = (acc.data[2] + lit.data[2]).max(0.).min(1.);
    let a = (acc.data[3] + lit.data[3]).max(0.).min(1.);
    Rgba { data: [r, g, b, a]}
}

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u8) -> Rgba<f64> {
    if depth > MAX_DEPTH { return BLACK; }
    let intersection = scene.trace(ray);
    if intersection.is_none() { return BLACK; } else {
        let i = intersection.unwrap();
        let hit_point = ray.origin + (ray.direction * i.distance);
        let surface_normal = i.element.surface_normal(&hit_point);
        let mut color_acc: Rgba<f64> = BLACK;
        for light in &scene.lights {
            if i.element.diffuse() > 0.0 {
                let light_direction = light.direction_from(&hit_point);
                let shadow_ray = Ray {
                    origin: hit_point + (surface_normal * SHADOW_BIAS),
                    direction: light_direction,
                };
                let shadow_intersection = scene.trace(&shadow_ray);
                let in_light = shadow_intersection.is_none() ||
                    shadow_intersection.unwrap().distance > light.distance(&hit_point);
                let light_intensity = if in_light { light.intensity(&hit_point) } else { 0.0 };
                let light_power = (surface_normal.dot(light_direction)).max(0.0) * light_intensity;
                let diffuse_proportion = i.element.diffuse() / PI;
                let diffuse = scale_colors(&i.element.color(), &light.color(), light_power * diffuse_proportion);
                color_acc = accumulate_color(&color_acc, &diffuse);
            }
            if i.element.specular() > 0.0 {
                let reflection_ray = Ray {
                    origin: hit_point + (surface_normal * SHADOW_BIAS),
                    direction: ray.direction - (2.0 * ray.direction.dot(surface_normal) * surface_normal),
                };
                let specular = cast_ray(scene, &reflection_ray, depth+1);
                color_acc = accumulate_color(&color_acc, &scale_colors(&WHITE, &specular, i.element.specular()));
            }
        }
        return color_acc;
    }
}