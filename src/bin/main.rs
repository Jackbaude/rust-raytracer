use std::time::Instant;
use raytrace::options::Options;
use raytrace::intersectable::sphere::Sphere;
use raytrace::camera::Camera;
use raytrace::vector::Vec3;
use raytrace::material::Material;
use raytrace::color::Color;
use raytrace::light::{Light, LightType};
use raytrace::scene::Scene;
use raytrace::intersectable::plane::Plane;

fn main() {
    let options = Options {
        max_rays: 4,
        gamma: 0.85,
        diffuse: true,
        specular: true,
        shadows: true,
        reflections: true,
    };

    let width = 1920;
    let height = 1080;
    let aspect_ratio = width as f64 / height as f64;

    let scene = Scene {
        width,
        height,
        camera: Camera::new(
            Vec3::new(0., -3., 5.),
            Vec3::new(0., 0., -100.),
            60.,
            aspect_ratio,
            0.,
        ),
        objects: vec![
            Box::new(Sphere {
                position: Vec3::new(-3.0, -5.0, -16.0),
                radius: 2.8,
                material: Material {
                    color: Color::from_u8(0xff, 0x55, 0x55),
                    diffuse: 0.6,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 0.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(0.0, -5.0, -13.0),
                radius: 2.0,
                material: Material {
                    color: Color::from_u8(0x40, 0xe0, 0xd0),
                    diffuse: 0.6,
                    specular: 5.0,
                    specular_exponent: 500.0,
                    reflectiveness: 0.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(3.0, -5.0, -17.0),
                radius: 2.8,
                material: Material {
                    color: Color::from_u8(0x77, 0xbb, 0x77),
                    diffuse: 0.5,
                    specular: 0.2,
                    specular_exponent: 2.0,
                    reflectiveness: 0.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(0.0, -4.0, -20.0),
                radius: 3.0,
                material: Material {
                    color: Color::from_u8(0x2f, 0x8d, 0xff),
                    diffuse: 0.6,
                    specular: 3.0,
                    specular_exponent: 50.0,
                    reflectiveness: 0.0,
                },
            }),
            Box::new(Sphere {
                position: Vec3::new(-10.0, 5.0, -20.0),
                radius: 5.0,
                material: Material {
                    color: Color::new(0.1, 0.1, 0.1),
                    diffuse: 0.0,
                    specular: 50.0,
                    specular_exponent: 100.0,
                    reflectiveness: 1.0,
                },
            }),
            Box::new(Plane {
                position: Vec3::new(0.0, -20.0, -10.0),
                normal: Vec3::new(0.0, -1.0, 0.0),
                material: Material {
                    color: Color::from_u8(0x66, 0x33, 0x66),
                    diffuse: 0.8,
                    specular: 0.2,
                    specular_exponent: 5.0,
                    reflectiveness: 0.6,
                },
            }),
        ],
        lights: vec![
            Light {
                light_type: LightType::Point,
                position: Vec3::new(-40.0, 20.0, 20.0),
                intensity: 1.0,
                color: Color::white(),
            },
            // Light {
            //     light_type: LightType::Point,
            //     position: Vec3::new(40.0, 20.0, 20.0),
            //     intensity: 0.8,
            //     color: Color::new(0.66, 0.0, 0.66),
            // },
            // Light {
            //     light_type: LightType::Point,
            //     position: Vec3::new(00.0, 50.0, 0.0),
            //     intensity: 0.8,
            //     color: Color::from_u8(0xa6, 0x7c, 0x00),
            // },
            // Light {
            //     light_type: LightType::Ambient,
            //     position: Vec3::zero(),
            //     intensity: 0.25,
            //     color: Color::white(),
            // },
        ],
        bg_color: Color::black(),
        options,
    };

    let now = Instant::now();

    scene.render("result3.png".to_string());

    let duration = now.elapsed();

    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
}