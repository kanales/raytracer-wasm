mod lib;

use lib::object::*;
use lib::utils::*;
use lib::*;
use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let light = Light {
        direction: (Vector3 {
            x: 0.,
            y: -0.3,
            z: -1.,
        })
        .normalize(),
        color: Color {
            r: 255,
            g: 255,
            b: 255,
        },
        intensity: 3.,
    };
    let plane = Plane {
        origin: Vector3 {
            x: 0.,
            y: -1.,
            z: 0.,
        },
        normal: Vector3 {
            x: 0.,
            y: 1.,
            z: 0.,
        }
        .normalize(),
        material: Material {
            albedo: 1.,
            reflectivity: 0.1,
            color: Color {
                r: 0x66,
                g: 0xCC,
                b: 0x66,
            },
        },
    };
    let mut scene = Scene::new(600, 800, std::f64::consts::PI / 2., light, plane);
    scene.add_sphere(Sphere {
        center: Vector3 {
            x: 0.,
            y: 0.25,
            z: -2.,
        },
        radius: 0.5,
        material: Material {
            color: Color {
                r: 0xCC,
                g: 0xCC,
                b: 0xCC,
            },
            albedo: 1.,
            reflectivity: 0.8,
        },
    });
    scene.add_sphere(Sphere {
        center: Vector3 {
            x: 0.75,
            y: 0.75,
            z: -1.25,
        },
        radius: 0.75,
        material: Material {
            color: Color {
                r: 0xFF,
                g: 0x55,
                b: 0x55,
            },
            albedo: 1.,
            reflectivity: 0.2,
        },
    });
    scene.add_sphere(Sphere {
        center: Vector3 {
            x: -1.,
            y: 1.,
            z: -3.,
        },
        radius: 1.,
        material: Material {
            color: Color {
                r: 0x55,
                g: 0x55,
                b: 0xFF,
            },
            albedo: 1.,
            reflectivity: 0.3,
        },
    });
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(scene.width as u32, scene.height as u32);
    let pixels = scene.render();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let color = pixels[(y as usize) * scene.width + (x as usize)];
        *pixel = image::Rgb([color.r, color.g, color.b]);
    }

    imgbuf.save("render.png").unwrap();
}
