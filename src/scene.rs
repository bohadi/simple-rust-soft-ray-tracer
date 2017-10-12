use cgmath::prelude::*;
use cgmath::Point3;
use cgmath::Vector3;
use image::Rgba;

use render::*;

pub struct Light {
    pub direction: Vector3<f64>,
    pub color: Rgba<f64>,
    pub intensity: f64,
}

pub struct Plane {
    pub origin: Point3<f64>,
    pub normal: Vector3<f64>,
    pub color: Rgba<f64>,
    pub albedo: f64,
}

impl Plane {
    pub fn new(origin: Point3<f64>, normal: Vector3<f64>, color: Rgba<f64>, albedo: f64) -> Plane {
        Plane { origin: origin, normal: normal.normalize(), color: color, albedo: albedo}
    }
}

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub color:  Rgba<f64>,
    pub albedo: f64,
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self) -> &Rgba<f64> {
        match *self {
            Element::Sphere(ref s) => &s.color,
            Element::Plane(ref p)  => &p.color,
        }
    }
    pub fn albedo(&self) -> f64 {
        match *self {
            Element::Sphere(ref s) => s.albedo,
            Element::Plane(ref p)  => p.albedo,
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Point3<f64>) -> Vector3<f64>;
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p)  => p.intersect(ray),
        }
    }

    fn surface_normal(&self, hit_point: &Point3<f64>) -> Vector3<f64> {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p)  => p.surface_normal(hit_point),
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l: Vector3<f64> = self.center - ray.origin;
        let adj = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 { return None; }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;
        if t0 < 0.0 && t1 < 0.0 { return None; }
        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }

    fn surface_normal(&self, hit_point: &Point3<f64>) -> Vector3<f64> {
        (*hit_point - self.center).normalize()
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let denom = self.normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(self.normal) / denom;
            if distance >= 0.0 { return Some(distance); }
        }
        None
    }

    fn surface_normal(&self, _hit_point: &Point3<f64>) -> Vector3<f64> {
        -self.normal
    }
}

pub const RESO_W: u32 = 1600;
pub const RESO_H: u32 = 900;
pub const ASPECT: f64 = (RESO_W as f64) / (RESO_H as f64);
pub const FOV:    f64 = 70.0;
pub const SHADOW_BIAS: f64 = 1e-13;
pub const _MAP_X:  u16 = 10;
pub const _MAP_Y:  u16 = 10;
pub const _BLOCK_DIM: u8 = 1;

pub const MAX_DEPTH: u8 = 10;
pub const BLACK: Rgba<u8> = Rgba { data: [0, 0, 0, 0] };

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    //pub spheres: [Sphere; 3],
    //pub ground: Plane,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,

    //pub hmap: [i32; (MAP_X * MAP_Y) as usize],
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        if !distance.is_finite() { panic!("Intersection must have finite distance") }
        Intersection {
            distance: distance,
            element: element,
        }
    }
}