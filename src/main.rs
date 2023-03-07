extern crate nalgebra as na;

use std::num;

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

struct Triangle {
    vertices: [Vec3; 3],
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
    /// Checks if a ray intersects this object, and returns the point of intersection if it does.
    /// Uses a mutable reference to the closest intersection so far, so the caller can pass in
    /// closer objects, letting this returning None if there's a closer intersection.
    fn intersected_by(&self, ray: &Ray, t_closest: &mut f64) -> Option<Vec3>;

    fn color(&self, point: Vec3, view_direction: Vec3, scene: &Scene) -> Vec3;

    fn normal(&self, point: Vec3) -> Vec3;
}

impl Object for Sphere {
    fn intersected_by(&self, ray: &Ray, t_closest: &mut f64) -> Option<Vec3> {
        let d = ray.direction.normalize();
        let o = ray.origin;
        let c = self.center;

        // B and C factors of circle equation
        let b = 2.0 * (d.x * o.x - d.x * c.x + d.y * o.y - d.y * c.y + d.z * o.z - d.z * c.z);
        let c = o.x.powi(2) - 2.0 * o.x * c.x + c.x.powi(2) + o.y.powi(2) - 2.0 * o.y * c.y
            + c.y.powi(2)
            + o.z.powi(2)
            - 2.0 * o.z * c.z
            + c.z.powi(2)
            - self.radius.powi(2);

        // Discriminant of quadratic euqation
        let discriminant = b.powi(2) - 4.0 * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find nearest root
        let mut root = (-b - discriminant.sqrt()) / 2.0;
        if root <= 0.0 || *t_closest < root {
            root = (-b + discriminant.sqrt()) / 2.0;
            if root <= 0.0 || *t_closest < root {
                return None;
            }
        }

        *t_closest = root;
        Some(ray.at(root))
    }

    fn color(&self, point: Vec3, view_direction: Vec3, scene: &Scene) -> Vec3 {
        let normal = &self.normal(point);
        let v = -view_direction;
        let dl = scene.direction_to_light.normalize();
        let ndl = normal.dot(&dl);
        let r = 2.0 * normal * normal.dot(&dl) - dl;

        let ambient = self.k_a * scene.ambient_light.component_mul(&self.o_d);
        let diffuse = self.k_d * scene.light_color.component_mul(&self.o_d) * ndl.max(0.0);
        let specular = self.k_s
            * scene.light_color.component_mul(&self.o_s)
            * v.dot(&r).max(0.0).powf(self.k_gls);

        let shadow_ray = Ray {
            origin: point + normal * 0.000001,
            direction: (scene.direction_to_light - point + normal * 0.000001).normalize(),
        };
        if let Some((_, _, t_closest)) = shadow_ray.nearest_hit(scene) {
            if t_closest > 0.0000001
                && t_closest < (scene.direction_to_light - point + normal * 0.000001).norm()
            {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }

        let res = if ndl > 0.0 {
            ambient + diffuse + specular
        } else {
            ambient + diffuse
        };
        res
    }

    fn normal(&self, point: Vec3) -> Vec3 {
        ((point - self.center) / self.radius).normalize()
    }
}

impl Object for Triangle {
    fn intersected_by(&self, ray: &Ray, t_closest: &mut f64) -> Option<Vec3> {
        let normal = self.normal(Vec3::zeros());
        let d = -(normal.dot(&self.vertices[0]));
        let v_d = normal.dot(&ray.direction.normalize());
        if v_d == 0.0 {
            return None;
        }
        let v_o = -(normal.dot(&ray.origin) + d);
        let t = v_o / v_d;
        if t < 0.0 || *t_closest < t {
            return None;
        }
        let dominant_index = normal.iamax();
        let r = ray.at(t).remove_row(dominant_index);
        let pv = self.vertices.map(|v| v.remove_row(dominant_index) - r);
        let mut num_crossings = 0;
        for i in 0..pv.len() {
            let next = (i + 1) % pv.len();
            let signholder = if pv[i].y < 0.0 { -1 } else { 1 };
            let next_signholder = if pv[next].y < 0.0 { -1 } else { 1 };
            if signholder != next_signholder {
                if pv[i].x > 0.0 && pv[next].x > 0.0 {
                    num_crossings += 1;
                } else if pv[i].x > 0.0 || pv[next].x > 0.0 {
                    let u_cross =
                        pv[i].x - pv[i].y * (pv[next].x - pv[i].x) / (pv[next].y - pv[i].y);
                    if u_cross > 0.0 {
                        num_crossings += 1;
                    }
                }
            }
        }

        match num_crossings % 2 == 1 {
            false => None,
            true => {
                *t_closest = t;
                Some(ray.at(t))
            }
        }
    }

    fn color(&self, point: Vec3, view_direction: Vec3, scene: &Scene) -> Vec3 {
        let normal = &self.normal(point);
        let v = -view_direction;
        let dl = scene.direction_to_light.normalize();
        let ndl = normal.dot(&dl);
        let r = 2.0 * normal * normal.dot(&dl) - dl;

        let ambient = self.k_a * scene.ambient_light.component_mul(&self.o_d);
        let diffuse = self.k_d * scene.light_color.component_mul(&self.o_d) * ndl.max(0.0);
        let specular = self.k_s
            * scene.light_color.component_mul(&self.o_s)
            * v.dot(&r).max(0.0).powf(self.k_gls);

        let shadow_origin = point + normal * 0.00000000001;
        let shadow_ray = Ray {
            origin: shadow_origin,
            direction: (scene.direction_to_light).normalize(),
        };
        for object in &scene.objects {
            let mut t = f64::INFINITY;
            if object.intersected_by(&shadow_ray, &mut t).is_some() {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }

        ambient + diffuse //+ specular
                          //let res = if ndl > 0.0 {
                          //} else {
                          //    ambient + diffuse
                          //};
                          //res
    }

    fn normal(&self, _point: Vec3) -> Vec3 {
        (self.vertices[1] - self.vertices[0]).cross(&(self.vertices[2] - self.vertices[1]))
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

    fn nearest_hit<'a>(&self, scene: &'a Scene) -> Option<(Vec3, &'a Box<dyn Object>, f64)> {
        let mut hit = None;
        let mut t_closest = f64::INFINITY;
        for object in &scene.objects {
            if let Some(p) = object.intersected_by(&self, &mut t_closest) {
                hit = Some((p, object, t_closest));
            }
        }
        hit
    }

    fn color(&self, scene: &Scene) -> Vec3 {
        let unit_direction = self.direction.normalize();
        if let Some((p, object, _)) = self.nearest_hit(scene) {
            object.color(p, unit_direction, scene)
        } else {
            scene.background_color
        }
    }
}

const ASPECT: f64 = 1.0;
const WIDTH: f64 = 500.0;
const HEIGHT: f64 = WIDTH / ASPECT;

const VIEWPORT_WIDTH: f64 = 1.0;
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
    let scene_1_objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere {
            // white sphere
            center: Vec3::new(0.5, 0.0, -0.15),
            radius: 0.05,
            k_d: 0.8,
            k_s: 0.1,
            k_a: 0.3,
            o_d: Vec3::new(1.0, 1.0, 1.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 4.0,
        }),
        Box::new(Sphere {
            // red sphere
            center: Vec3::new(0.3, 0.0, -0.1),
            radius: 0.08,
            k_d: 0.8,
            k_s: 0.8,
            k_a: 0.1,
            o_d: Vec3::new(1.0, 0.0, 0.0),
            o_s: Vec3::new(0.5, 1.0, 0.5),
            k_gls: 32.0,
        }),
        Box::new(Sphere {
            // green sphere
            center: Vec3::new(-0.6, 0.0, 0.0),
            radius: 0.3,
            k_d: 0.7,
            k_s: 0.5,
            k_a: 0.1,
            o_d: Vec3::new(0.0, 1.0, 0.0),
            o_s: Vec3::new(0.5, 1.0, 0.5),
            k_gls: 64.0,
        }),
        Box::new(Sphere {
            // reflective sphere
            center: Vec3::new(0.1, -0.55, 0.25),
            radius: 0.3,
            k_d: 0.0,
            k_s: 0.1,
            k_a: 0.1,
            o_d: Vec3::new(0.75, 0.75, 0.75),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 10.0,
        }),
        Box::new(Triangle {
            // blue triangle
            vertices: [
                Vec3::new(0.3, -0.3, -0.4),
                Vec3::new(0.0, 0.3, -0.1),
                Vec3::new(-0.3, -0.3, 0.2),
            ],
            k_d: 0.9,
            k_s: 0.9,
            k_a: 0.1,
            o_d: Vec3::new(0.0, 0.0, 1.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 32.0,
        }),
        Box::new(Triangle {
            // yellow triangle
            vertices: [
                Vec3::new(-0.2, 0.1, 0.1),
                Vec3::new(-0.2, -0.5, 0.2),
                Vec3::new(-0.2, 0.1, -0.3),
            ],
            k_d: 0.9,
            k_s: 0.5,
            k_a: 0.1,
            o_d: Vec3::new(1.0, 1.0, 0.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 4.0,
        }),
    ];
    let scene_2_objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere {
            // white sphere
            center: Vec3::new(0.5, 0.0, -0.15),
            radius: 0.05,
            k_d: 0.8,
            k_s: 0.1,
            k_a: 0.3,
            o_d: Vec3::new(1.0, 1.0, 1.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 4.0,
        }),
        Box::new(Sphere {
            // white sphere
            center: Vec3::new(0.2, 0.0, -0.15),
            radius: 0.05,
            k_d: 0.8,
            k_s: 0.1,
            k_a: 0.3,
            o_d: Vec3::new(1.0, 1.0, 1.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 4.0,
        }),
        Box::new(Sphere {
            // white sphere
            center: Vec3::new(-0.1, 0.0, -0.15),
            radius: 0.05,
            k_d: 0.8,
            k_s: 0.1,
            k_a: 0.3,
            o_d: Vec3::new(1.0, 1.0, 1.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 4.0,
        }),
        Box::new(Sphere {
            // white sphere
            center: Vec3::new(-0.4, 0.0, -0.15),
            radius: 0.05,
            k_d: 0.8,
            k_s: 0.1,
            k_a: 0.3,
            o_d: Vec3::new(1.0, 1.0, 1.0),
            o_s: Vec3::new(1.0, 1.0, 1.0),
            k_gls: 4.0,
        }),
    ];
    let scene_3_objects: Vec<Box<dyn Object>> = vec![];

    let scene_1 = Scene {
        direction_to_light: Vec3::new(1.0, 0.0, 0.0),
        light_color: Vec3::new(1.0, 1.0, 1.0),
        ambient_light: Vec3::new(0.1, 0.1, 0.1),
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
            let color = r.color(&scene_1);
            print_color(&color);
        }
        println!("");
    }
}
