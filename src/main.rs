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
    camera: Camera,
    objects: Vec<Sphere>,
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    fn hits(&self, sphere: &Sphere) -> Option<Vec3> {
        let d = self.direction.normalize();
        let o = self.origin;
        let c = sphere.center;
        let b = 2.0 * (d.x * o.x - d.x * c.x + d.y * o.y - d.y * c.y + d.z * o.z - d.z * c.z);
        let c = o.x.powi(2) - 2.0 * o.x * c.x + c.x.powi(2) + o.y.powi(2) - 2.0 * o.y * c.y
            + c.y.powi(2)
            + o.z.powi(2)
            - 2.0 * o.z * c.z
            + c.z.powi(2)
            - sphere.radius.powi(2);
        let disc = b.powi(2) - 4.0 * c;
        if disc < 0.0 {
            return None;
        }
        let t;
        let t0 = (-b - disc.sqrt()) / 2.0;
        if t0 <= 0.0 {
            let t1 = (-b + disc.sqrt()) / 2.0;
            if t1 <= 0.0 {
                return None;
            } else {
                t = t1;
            }
        } else {
            t = t0;
        }
        Some(self.at(t))
    }

    fn color(&self, sphere: &Sphere) -> Vec3 {
        let unit_direction = self.direction.normalize();
        if let Some(p) = self.hits(sphere) {
            let n = (p - sphere.center).normalize();
            0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
        } else {
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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
    let camera_origin = Vec3::new(0.0, 0.0, 1.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left =
        camera_origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let sphere_1 = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 0.4,
        k_d: 0.7,
        k_s: 0.2,
        k_a: 0.1,
        o_d: Vec3::new(1.0, 0.0, 1.0),
        o_s: Vec3::new(1.0, 1.0, 1.0),
        k_gls: 16.0,
    };

    println!("P3\n{WIDTH} {HEIGHT}\n255");
    for r in (0..HEIGHT as i64).rev() {
        for c in 0..WIDTH as i64 {
            let u = c as f64 / (WIDTH - 1.0);
            let v = r as f64 / (HEIGHT - 1.0);
            let r = Ray {
                origin: camera_origin,
                direction: lower_left + u * horizontal + v * vertical - camera_origin,
            };
            let color = r.color(&sphere_1);
            print_color(&color);
        }
        println!("");
    }
}
