use raytracer::geometry::Vec3;
use raytracer::rays::Ray;
use raytracer::scene::get_scene_1;

struct Camera {
    look_at: Vec3,
    look_from: Vec3,
    look_up: Vec3,
    fov: f64,
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
            let r = Ray::new(
                camera_origin,
                lower_left + u * horizontal + v * vertical - camera_origin,
            );
            let color = r.color(&get_scene_1());
            print_color(&color);
        }
        println!("");
    }
}
