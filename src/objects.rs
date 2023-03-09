use crate::{geometry::Vec3, rays::Ray, scene::Scene};

pub struct Material {
    pub k_d: f64,
    pub k_s: f64,
    pub k_a: f64,
    pub k_gls: f64,
    pub o_d: Vec3,
    pub o_s: Vec3,
}

impl Material {
    fn color(&self, normal: &Vec3, view_direction: Vec3, scene: &Scene) -> Vec3 {
        let v = -view_direction;
        let dl = scene.direction_to_light.normalize();
        let ndl = normal.dot(&dl);
        let r = 2.0 * normal * normal.dot(&dl) - dl;

        let ambient = self.k_a * scene.ambient_light.component_mul(&self.o_d);
        let diffuse = self.k_d * scene.light_color.component_mul(&self.o_d) * ndl.max(0.0);
        let specular = self.k_s
            * scene.light_color.component_mul(&self.o_s)
            * v.dot(&r).max(0.0).powf(self.k_gls);

        ambient + diffuse + specular
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

pub struct Triangle {
    pub vertices: [Vec3; 3],
    pub material: Material,
}

pub trait Object {
    fn get_material(&self) -> &Material;

    /// Checks if a ray intersects this object, and returns the point of intersection if it does.
    /// Uses a mutable reference to the closest intersection so far, so the caller can pass in
    /// closer objects, letting this returning None if there's a closer intersection.
    fn intersected_by(&self, ray: &Ray, t_closest: &mut f64) -> Option<Vec3>;

    fn normal_ugly(&self, point: Vec3) -> Vec3;

    fn normal(&self, point: Vec3) -> Vec3 {
        self.normal_ugly(point).normalize()
    }

    fn color(&self, point: Vec3, view_direction: Vec3, scene: &Scene) -> Vec3 {
        let normal = &self.normal(point);
        let shadow_origin = point + normal * 0.00000000001;
        let shadow_ray = Ray::new(shadow_origin, scene.direction_to_light);
        for object in &scene.objects {
            let mut t = f64::INFINITY;
            if object.intersected_by(&shadow_ray, &mut t).is_some() {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }
        self.get_material().color(normal, view_direction, &scene)
    }
}

impl Object for Sphere {
    fn get_material(&self) -> &Material {
        &self.material
    }

    fn intersected_by(&self, ray: &Ray, t_closest: &mut f64) -> Option<Vec3> {
        let d = ray.direction();
        let o = ray.origin();
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

    fn normal_ugly(&self, point: Vec3) -> Vec3 {
        (point - self.center) / self.radius
    }
}

impl Object for Triangle {
    fn get_material(&self) -> &Material {
        &self.material
    }

    fn intersected_by(&self, ray: &Ray, t_closest: &mut f64) -> Option<Vec3> {
        let normal = self.normal(Vec3::zeros());
        let d = -(normal.dot(&self.vertices[0]));
        let v_d = normal.dot(&ray.direction());
        if v_d == 0.0 {
            return None;
        }
        let v_o = -(normal.dot(&ray.origin()) + d);
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

    fn normal_ugly(&self, _point: Vec3) -> Vec3 {
        (self.vertices[1] - self.vertices[0]).cross(&(self.vertices[2] - self.vertices[1]))
    }
}
