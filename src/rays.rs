use crate::objects::Object;
use crate::{geometry::Vec3, scene::Scene};

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn nearest_hit<'a>(&self, scene: &'a Scene) -> Option<(Vec3, &'a Box<dyn Object>, f64)> {
        let mut hit = None;
        let mut t_closest = f64::INFINITY;
        for object in &scene.objects {
            if let Some(p) = object.intersected_by(&self, &mut t_closest) {
                hit = Some((p, object, t_closest));
            }
        }
        hit
    }

    pub fn color(&self, scene: &Scene) -> Vec3 {
        let unit_direction = self.direction;
        if let Some((p, object, _)) = self.nearest_hit(scene) {
            object.color(p, unit_direction, scene)
        } else {
            scene.background_color
        }
    }
}
