pub mod object;
pub mod scene;
pub mod utils;

use object::*;
use scene::*;
use utils::*;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageData};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut(i32)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub fn animate(ctx: CanvasRenderingContext2d, width: usize, height: usize) -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut last_time = 0;
    let scene = Rc::new(RefCell::new(test_scene(width, height)));
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: i32| {
        let dt: f64 = (timestamp - last_time) as f64 / 1000.;
        last_time = timestamp;
        let angle = dt;
        let mut scene = scene.borrow_mut();
        scene.update(
            angle,
            Vector3 {
                x: 0.,
                y: 0.,
                z: -1.,
            },
        );
        draw(&ctx, &scene).unwrap();
        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(i32)>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

pub fn draw(ctx: &CanvasRenderingContext2d, scene: &Scene) -> Result<(), JsValue> {
    let mut colors = vec![0; scene.height * scene.width * 4];
    for (x, y, color) in scene.render() {
        let pos = y * scene.width as usize + x;
        colors[4 * pos + 0] = color.r;
        colors[4 * pos + 1] = color.g;
        colors[4 * pos + 2] = color.b;
        colors[4 * pos + 3] = 0xFF;
    }
    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut colors),
        scene.width as u32,
        scene.height as u32,
    )?;

    ctx.put_image_data(&data, 0.0, 0.0)
}

pub fn test_scene(width: usize, height: usize) -> Scene {
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
    let mut scene = Scene::new(height, width, std::f64::consts::PI / 2., light, plane);
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

    scene
}
