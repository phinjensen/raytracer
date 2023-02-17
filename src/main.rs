extern crate nalgebra as na;

use std::mem::discriminant;

use na::Vector3;

type Vec3 = Vector3<f64>;

struct Camera {
    look_at: Vec3,
    look_from: Vec3,
    look_up: Vec3,
    fov: f64,
}

struct Sphere {
    center: Vec3,
    radius: f64,
    k_d: f64,
    k_s: f64,
    k_a: f64,
    k_gls: f64,
    o_d: Vec3,
    o_s: Vec3,
}

struct Scene {
    direction_to_light: Vec3,
    light_color: Vec3,
    ambient_light: Vec3,
    background_color: Vec3,
    //camera: Camera,
    objects: Vec<Box<dyn Object>>,
}

trait Object {
    fn intersected_by(&self, ray: &Ray, t_closest: f64) -> Option<Vec3>;

    fn color(&self, normal: Vec3, view_direction: Vec3, scene: &Scene) -> Vec3;
}

impl Object for Sphere {
    fn intersected_by(&self, ray: &Ray, t_closest: f64) -> Option<Vec3> {
        let d = ray.direction.normalize();
        let o = ray.origin;
        let c = self.center;
        let b = 2.0 * (d.x * o.x - d.x * c.x + d.y * o.y - d.y * c.y + d.z * o.z - d.z * c.z);
        let c = o.x.powi(2) - 2.0 * o.x * c.x + c.x.powi(2) + o.y.powi(2) - 2.0 * o.y * c.y
            + c.y.powi(2)
            + o.z.powi(2)
            - 2.0 * o.z * c.z
            + c.z.powi(2)
            - self.radius.powi(2);
        let disc = b.powi(2) - 4.0 * c;
        if disc < 0.0 {
            return None;
        }
        let mut root = (-b - disc.sqrt()) / 2.0;
        if root <= 0.0 || t_closest < root {
            root = (-b + disc.sqrt()) / 2.0;
            if root <= 0.0 || t_closest < root {
                return None;
            }
        }
        Some((ray.at(root) - self.center).normalize())
    }

    fn color(&self, normal: Vec3, view_direction: Vec3, scene: &Scene) -> Vec3 {
        let r = scene.direction_to_light - 2.0 * normal * scene.direction_to_light.dot(&normal);
        let facing_light = normal.dot(&scene.direction_to_light);
        self.k_a * scene.ambient_light
            + self.k_d * self.o_d * facing_light
            + if facing_light > 0.0 {
                self.k_s * self.o_s * r.dot(&view_direction).powf(self.k_gls)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
    }
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    fn hits<'a>(&self, scene: &'a Scene) -> Option<(Vec3, &'a Box<dyn Object>)> {
        let mut hit = None;
        for object in &scene.objects {
            if let Some(n) = object.intersected_by(&self, f64::INFINITY) {
                hit = Some((n, object));
            }
        }
        hit
    }

    fn color(&self, scene: &Scene) -> Vec3 {
        let unit_direction = self.direction.normalize();
        if let Some((n, object)) = self.hits(scene) {
            object.color(n, unit_direction, scene)
        } else {
            scene.background_color
        }
    }
}

const ASPECT: f64 = 16.0 / 9.0;
const WIDTH: f64 = 400.0;
const HEIGHT: f64 = WIDTH / ASPECT;

const VIEWPORT_WIDTH: f64 = 2.0;
const VIEWPORT_HEIGHT: f64 = VIEWPORT_WIDTH / ASPECT;
const FOCAL_LENGTH: f64 = 1.0;

fn print_color(color: &Vec3) {
    print!(
        "{} {} {} ",
        (color.x * 255.0) as u8,
        (color.y * 255.0) as u8,
        (color.z * 255.0) as u8
    );
}

fn main() {
    let objects: Vec<Box<dyn Object>> = vec![Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 0.4,
        k_d: 0.7,
        k_s: 0.2,
        k_a: 0.1,
        o_d: Vec3::new(1.0, 0.0, 1.0),
        o_s: Vec3::new(1.0, 1.0, 1.0),
        k_gls: 16.0,
    })];
    let scene = Scene {
        direction_to_light: Vec3::new(0.0, 1.0, 0.0),
        light_color: Vec3::new(1.0, 1.0, 1.0),
        ambient_light: Vec3::new(1.0, 1.0, 1.0),
        background_color: Vec3::new(0.2, 0.2, 0.2),
        objects,
    };

    let camera_origin = Vec3::new(0.0, 0.0, 1.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left =
        camera_origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    println!("P3\n{WIDTH} {HEIGHT}\n255");
    for r in (0..HEIGHT as i64).rev() {
        for c in 0..WIDTH as i64 {
            let u = c as f64 / (WIDTH - 1.0);
            let v = r as f64 / (HEIGHT - 1.0);
            let r = Ray {
                origin: camera_origin,
                direction: lower_left + u * horizontal + v * vertical - camera_origin,
            };
            let color = r.color(&scene);
            print_color(&color);
        }
        println!("");
    }
}
