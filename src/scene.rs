use crate::geometry::Vec3;
use crate::objects::{Material, Object, Sphere, Triangle};

pub struct Scene {
    pub direction_to_light: Vec3,
    pub light_color: Vec3,
    pub ambient_light: Vec3,
    pub background_color: Vec3,
    //camera: Camera,
    pub objects: Vec<Box<dyn Object>>,
}

pub fn get_scene_1() -> Scene {
    Scene {
        direction_to_light: Vec3::new(0.0, 1.0, 0.0),
        light_color: Vec3::new(1.0, 1.0, 1.0),
        ambient_light: Vec3::new(0.0, 0.0, 0.0),
        background_color: Vec3::new(0.2, 0.2, 0.2),
        objects: vec![
            Box::new(Sphere {
                // reflective sphere
                center: Vec3::new(0.0, 0.3, -1.0),
                radius: 0.25,
                material: Material {
                    k_d: 0.0,
                    k_s: 0.1,
                    k_a: 0.1,
                    o_d: Vec3::new(0.75, 0.75, 0.75),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 10.0,
                    refl: 0.9,
                },
            }),
            Box::new(Triangle {
                // blue triangle
                vertices: [
                    Vec3::new(0.0, -0.7, -0.5),
                    Vec3::new(1.0, 0.4, -1.0),
                    Vec3::new(0.0, -0.7, -1.5),
                ],
                material: Material {
                    k_d: 0.9,
                    k_s: 1.0,
                    k_a: 0.1,
                    o_d: Vec3::new(0.0, 0.0, 1.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 0.0,
                },
            }),
            Box::new(Triangle {
                // yellow triangle
                vertices: [
                    Vec3::new(0.0, -0.7, -0.5),
                    Vec3::new(0.0, -0.7, -1.5),
                    Vec3::new(-1.0, 0.4, -1.0),
                ],
                material: Material {
                    k_d: 0.9,
                    k_s: 1.0,
                    k_a: 0.1,
                    o_d: Vec3::new(1.0, 1.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 0.0,
                },
            }),
        ],
    }
}

pub fn get_scene_2() -> Scene {
    Scene {
        direction_to_light: Vec3::new(1.0, 0.0, 0.0),
        light_color: Vec3::new(1.0, 1.0, 1.0),
        ambient_light: Vec3::new(0.1, 0.1, 0.1),
        background_color: Vec3::new(0.2, 0.2, 0.2),
        objects: vec![
            Box::new(Sphere {
                // white sphere
                center: Vec3::new(0.5, 0.0, -0.15),
                radius: 0.05,
                material: Material {
                    k_d: 0.8,
                    k_s: 0.1,
                    k_a: 0.3,
                    o_d: Vec3::new(1.0, 1.0, 1.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 0.0,
                },
            }),
            Box::new(Sphere {
                // red sphere
                center: Vec3::new(0.3, 0.0, -0.1),
                radius: 0.08,
                material: Material {
                    k_d: 0.8,
                    k_s: 0.8,
                    k_a: 0.1,
                    o_d: Vec3::new(1.0, 0.0, 0.0),
                    o_s: Vec3::new(0.5, 1.0, 0.5),
                    k_gls: 32.0,
                    refl: 0.0,
                },
            }),
            Box::new(Sphere {
                // green sphere
                center: Vec3::new(-0.6, 0.0, 0.0),
                radius: 0.3,
                material: Material {
                    k_d: 0.7,
                    k_s: 0.5,
                    k_a: 0.1,
                    o_d: Vec3::new(0.0, 1.0, 0.0),
                    o_s: Vec3::new(0.5, 1.0, 0.5),
                    k_gls: 64.0,
                    refl: 0.0,
                },
            }),
            Box::new(Sphere {
                // reflective sphere
                center: Vec3::new(0.1, -0.55, 0.25),
                radius: 0.3,
                material: Material {
                    k_d: 0.0,
                    k_s: 0.1,
                    k_a: 0.1,
                    o_d: Vec3::new(0.75, 0.75, 0.75),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 10.0,
                    refl: 0.9,
                },
            }),
            Box::new(Triangle {
                // blue triangle
                vertices: [
                    Vec3::new(0.3, -0.3, -0.4),
                    Vec3::new(0.0, 0.3, -0.1),
                    Vec3::new(-0.3, -0.3, 0.2),
                ],
                material: Material {
                    k_d: 0.9,
                    k_s: 0.9,
                    k_a: 0.1,
                    o_d: Vec3::new(0.0, 0.0, 1.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 32.0,
                    refl: 0.0,
                },
            }),
            Box::new(Triangle {
                // yellow triangle
                vertices: [
                    Vec3::new(-0.2, 0.1, 0.1),
                    Vec3::new(-0.2, -0.5, 0.2),
                    Vec3::new(-0.2, 0.1, -0.3),
                ],
                material: Material {
                    k_d: 0.9,
                    k_s: 0.5,
                    k_a: 0.1,
                    o_d: Vec3::new(1.0, 1.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 0.0,
                },
            }),
        ],
    }
}

pub fn get_scene_3() -> Scene {
    Scene {
        direction_to_light: Vec3::new(1.0, 1.0, 1.0),
        light_color: Vec3::new(1.0, 1.0, 1.0),
        ambient_light: Vec3::new(0.2, 0.2, 0.2),
        background_color: Vec3::new(0.2, 0.2, 0.2),
        objects: vec![
            Box::new(Triangle {
                // mirror one
                vertices: [
                    Vec3::new(-0.40, 0.0, -1.0),
                    Vec3::new(-0.60, 0.18, 0.0),
                    Vec3::new(-0.60, -0.18, 0.0),
                ],
                material: Material {
                    k_d: 0.9,
                    k_s: 1.0,
                    k_a: 0.1,
                    o_d: Vec3::new(1.0, 0.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 0.9,
                },
            }),
            Box::new(Triangle {
                // mirror one
                vertices: [
                    Vec3::new(-0.20, 0.0, -1.0),
                    Vec3::new(-0.40, 0.18, 0.0),
                    Vec3::new(-0.40, -0.18, 0.0),
                ],
                material: Material {
                    k_d: 0.9,
                    k_s: 1.0,
                    k_a: 0.1,
                    o_d: Vec3::new(1.0, 0.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 0.9,
                },
            }),
            Box::new(Triangle {
                // mirror one
                vertices: [
                    Vec3::new(0.0, 0.0, -1.0),
                    Vec3::new(-0.20, 0.18, 0.0),
                    Vec3::new(-0.20, -0.18, 0.0),
                ],
                material: Material {
                    k_d: 0.9,
                    k_s: 1.0,
                    k_a: 0.1,
                    o_d: Vec3::new(1.0, 0.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 0.9,
                },
            }),
            Box::new(Triangle {
                // mirror one
                vertices: [
                    Vec3::new(0.20, 0.0, -1.0),
                    Vec3::new(0.00, 0.18, 0.0),
                    Vec3::new(0.00, -0.18, 0.0),
                ],
                material: Material {
                    k_d: 0.9,
                    k_s: 1.0,
                    k_a: 0.1,
                    o_d: Vec3::new(1.0, 0.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 0.9,
                },
            }),
            Box::new(Triangle {
                // mirror one
                vertices: [
                    Vec3::new(0.20, 0.20, -1.0),
                    Vec3::new(0.20, -0.20, -1.0),
                    Vec3::new(0.40, -0.18, -1.0),
                ],
                material: Material {
                    k_d: 0.9,
                    k_s: 1.0,
                    k_a: 0.1,
                    o_d: Vec3::new(1.0, 0.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 0.9,
                },
            }),
            Box::new(Sphere {
                // mirror one
                center: Vec3::new(0.0, 0.0, -0.5),
                radius: 0.05,
                material: Material {
                    k_d: 0.9,
                    k_s: 1.0,
                    k_a: 0.1,
                    o_d: Vec3::new(1.0, 0.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 0.9,
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(0.2, 0.2, -0.5),
                radius: 0.1,
                material: Material {
                    k_d: 0.8,
                    k_s: 0.1,
                    k_a: 0.3,
                    o_d: Vec3::new(0.0, 0.0, 1.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 4.0,
                    refl: 1.0,
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(0.2, 0.1, -0.5),
                radius: 0.08,
                material: Material {
                    k_d: 0.8,
                    k_s: 0.1,
                    k_a: 0.3,
                    o_d: Vec3::new(0.0, 1.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 32.0,
                    refl: 1.0,
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(0.2, 0.02, -0.5),
                radius: 0.06,
                material: Material {
                    k_d: 0.8,
                    k_s: 0.1,
                    k_a: 0.3,
                    o_d: Vec3::new(1.0, 0.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 64.0,
                    refl: 1.0,
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, 1.0, 0.0),
                radius: 0.75,
                material: Material {
                    k_d: 0.8,
                    k_s: 0.1,
                    k_a: 0.3,
                    o_d: Vec3::new(0.0, 0.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 64.0,
                    refl: 1.0,
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -1.0, 0.0),
                radius: 0.75,
                material: Material {
                    k_d: 0.8,
                    k_s: 0.1,
                    k_a: 0.3,
                    o_d: Vec3::new(0.0, 0.0, 0.0),
                    o_s: Vec3::new(1.0, 1.0, 1.0),
                    k_gls: 64.0,
                    refl: 1.0,
                },
            }),
        ],
    }
}
