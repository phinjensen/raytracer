extern crate nalgebra as na;

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
    fn intersected_by(&self, ray: &Ray, t_closest: &mut f64) -> Option<Vec3>;

    fn color(&self, normal: Vec3, view_direction: Vec3, scene: &Scene) -> Vec3;
}

impl Object for Sphere {
    fn intersected_by(&self, ray: &Ray, t_closest: &mut f64) -> Option<Vec3> {
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
        if root <= 0.0 || *t_closest < root {
            root = (-b + disc.sqrt()) / 2.0;
            if root <= 0.0 || *t_closest < root {
                return None;
            }
        }
        *t_closest = root;
        Some(((ray.at(root) - self.center) / self.radius).normalize())
    }

    fn color(&self, normal: Vec3, view_direction: Vec3, scene: &Scene) -> Vec3 {
        let v = -view_direction;
        let dl = scene.direction_to_light.normalize();
        let ndl = normal.dot(&dl);
        let r = 2.0 * normal * normal.dot(&dl) - dl;
        let ambient = self.k_a * scene.ambient_light.component_mul(&self.o_d);
        let diffuse = self.k_d * scene.light_color.component_mul(&self.o_d) * ndl.max(0.0);
        let specular = self.k_s
            * scene.light_color.component_mul(&self.o_s)
            * v.dot(&r).max(0.0).powf(self.k_gls);
        let res = if ndl > 0.0 {
            ambient + diffuse + specular
        } else {
            ambient + diffuse
        };
        if res.x > 1.0 || res.y > 1.0 || res.z > 1.0 {
            eprintln!("{res}");
        }
        res
    }
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction.normalize() * t
    }

    fn hits<'a>(&self, scene: &'a Scene) -> Option<(Vec3, &'a Box<dyn Object>)> {
        let mut hit = None;
        let mut t_closest = f64::INFINITY;
        for object in &scene.objects {
            if let Some(n) = object.intersected_by(&self, &mut t_closest) {
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
const WIDTH: f64 = 600.0;
const HEIGHT: f64 = WIDTH / ASPECT;

const VIEWPORT_WIDTH: f64 = 3.0;
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
    let scene_1_objects: Vec<Box<dyn Object>> = vec![Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 0.4,
        k_d: 0.7,
        k_s: 0.2,
        k_a: 0.1,
        o_d: Vec3::new(1.0, 0.0, 1.0),
        o_s: Vec3::new(1.0, 1.0, 1.0),
        k_gls: 16.0,
    })];
    let scene_2_objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.45, 0.0, -0.15),
            radius: 0.15,
            k_d: 0.8,
            k_s: 0.1,
            k_a: 0.3,
            o_d: Vec3::new(1.0, 1.0, 1.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 4.0,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -0.1),
            radius: 0.2,
            k_d: 0.6,
            k_s: 0.3,
            k_a: 0.1,
            o_d: Vec3::new(1.0, 0.0, 0.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 32.0,
        }),
        Box::new(Sphere {
            center: Vec3::new(-0.6, 0.0, 0.0),
            radius: 0.3,
            k_d: 0.7,
            k_s: 0.2,
            k_a: 0.1,
            o_d: Vec3::new(0.0, 1.0, 0.0),
            o_s: Vec3::new(0.5, 1.0, 0.5),
            k_gls: 64.0,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -10000.5, 0.0),
            radius: 10000.0,
            k_d: 0.9,
            k_s: 0.0,
            k_a: 0.1,
            o_d: Vec3::new(0.0, 0.0, 1.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 16.0,
        }),
    ];
    let scene_3_objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 0.3,
            k_d: 0.7,
            k_s: 0.2,
            k_a: 0.1,
            o_d: Vec3::new(0.78, 0.38, 0.129),
            o_s: Vec3::new(0.5, 1.0, 0.5),
            k_gls: 64.0,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.25, 0.1, 0.0),
            radius: 0.1,
            k_d: 0.7,
            k_s: 0.2,
            k_a: 0.1,
            o_d: Vec3::new(0.78, 0.38, 0.129),
            o_s: Vec3::new(0.5, 1.0, 0.5),
            k_gls: 64.0,
        }),
        Box::new(Sphere {
            center: Vec3::new(-0.25, 0.1, 0.0),
            radius: 0.1,
            k_d: 0.7,
            k_s: 0.2,
            k_a: 0.1,
            o_d: Vec3::new(0.78, 0.38, 0.129),
            o_s: Vec3::new(0.5, 1.0, 0.5),
            k_gls: 64.0,
        }),
        Box::new(Sphere {
            center: Vec3::new(-0.05, 0.08, 0.25),
            radius: 0.05,
            k_d: 0.5,
            k_s: 0.4,
            k_a: 0.1,
            o_s: Vec3::new(1.0, 1.0, 1.0),
            o_d: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 16.0,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.05, 0.08, 0.25),
            radius: 0.05,
            k_d: 0.5,
            k_s: 0.4,
            k_a: 0.1,
            o_s: Vec3::new(1.0, 1.0, 1.0),
            o_d: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 16.0,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.00, -0.08, 0.25),
            radius: 0.1,
            k_d: 0.9,
            k_s: 0.0,
            k_a: 0.1,
            o_d: Vec3::new(1.0, 0.0, 0.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 64.0,
        }),
    ];

    let scene_1 = Scene {
        direction_to_light: Vec3::new(0.0, 1.0, 0.0),
        light_color: Vec3::new(1.0, 1.0, 1.0),
        ambient_light: Vec3::new(0.0, 0.0, 0.0),
        background_color: Vec3::new(0.2, 0.2, 0.2),
        objects: scene_1_objects,
    };
    let scene_2 = Scene {
        direction_to_light: Vec3::new(1.0, 1.0, 1.0),
        light_color: Vec3::new(1.0, 1.0, 1.0),
        ambient_light: Vec3::new(0.1, 0.1, 0.1),
        background_color: Vec3::new(0.2, 0.2, 0.2),
        objects: scene_2_objects,
    };
    let scene_3 = Scene {
        direction_to_light: Vec3::new(1.0, 1.0, 1.0),
        light_color: Vec3::new(1.0, 1.0, 1.0),
        ambient_light: Vec3::new(0.2, 0.2, 0.2),
        background_color: Vec3::new(0.2, 0.2, 0.2),
        objects: scene_3_objects,
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
            let color = r.color(&scene_3);
            print_color(&color);
        }
        println!("");
    }
}
