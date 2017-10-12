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
fn get_reflect_color(ec: &Rgba<f64>, lc: &Rgba<f64>, power: f64, reflect: f64) -> Rgba<u8> {
    let r = (ec.data[0] * lc.data[0] * power * reflect * 255.0) as u8;
    let g = (ec.data[1] * lc.data[1] * power * reflect * 255.0) as u8;
    let b = (ec.data[2] * lc.data[2] * power * reflect * 255.0) as u8;
    let a = (ec.data[3] * lc.data[3] * power * reflect * 255.0) as u8;
    Rgba { data: [r, g, b, a]}
}
fn accumulate_color(acc: &Rgba<u8>, lit: &Rgba<u8>) -> Rgba<u8> {
    let r = (acc.data[0] + lit.data[0]).min(255);
    let g = (acc.data[1] + lit.data[1]).min(255);
    let b = (acc.data[2] + lit.data[2]).min(255);
    let a = (acc.data[3] + lit.data[3]).min(255);
    Rgba { data: [r, g, b, a]}
}

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u8) -> Rgba<u8> {
    if depth > MAX_DEPTH { return BLACK; }
    let intersection = scene.trace(ray);
    match intersection {
        Some(i) => {
            let hit_point = ray.origin + (ray.direction * i.distance);
            let surface_normal = i.element.surface_normal(&hit_point);
            let mut color_acc: Rgba<u8> = Rgba { data: [0, 0, 0, 0] };
            for light in &scene.lights {
                let light_direction = -light.direction.normalize();
                let shadow_ray = Ray {
                    origin: hit_point + (surface_normal * SHADOW_BIAS),
                    direction: light_direction,
                };
                let ambient: f64 = 0.1;
                let in_light = scene.trace(&shadow_ray).is_none();
                let light_intensity = if in_light { light.intensity } else { ambient };
                let light_power = (surface_normal.dot(light_direction)).max(0.0) * light_intensity;
                let reflect_proportion = i.element.albedo() / PI;
                let lit = get_reflect_color(&i.element.color(), &light.color, light_power, reflect_proportion);
                color_acc = accumulate_color(&color_acc, &lit);
            }
            color_acc
        },
        None => BLACK
    }
}