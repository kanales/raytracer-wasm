use super::object::*;
use super::utils::*;
use wasm_bindgen::prelude::*;

pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f64,
}

const SHADOW_BIAS: f64 = 1e-6;

#[wasm_bindgen]
pub struct Scene {
    pub width: usize,
    pub height: usize,
    fov: f64,
    light: Light,
    objects: Vec<Sphere>,
    plane: Plane,
}

impl Scene {
    pub fn new(height: usize, width: usize, fov: f64, light: Light, plane: Plane) -> Scene {
        Scene {
            height,
            width,
            fov,
            light,
            objects: Vec::new(),
            plane: plane,
        }
    }

    pub fn add_sphere(&mut self, obj: Sphere) {
        self.objects.push(obj)
    }

    fn create_prime(&self, u: usize, v: usize) -> Ray {
        let (w, h) = (self.width as f64, self.height as f64);
        let (x, y) = (u as f64, v as f64);
        let fov_adj = (self.fov / 2.).tan();
        let aspect_ratio = w / h;
        let dir_x = (((x + 0.5) / w) * 2. - 1.) * aspect_ratio * fov_adj;
        let dir_y = (1. - 2. * ((y + 0.5) / h)) * fov_adj;

        Ray {
            origin: Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            direction: Vector3 {
                x: dir_x,
                y: dir_y,
                z: -1.,
            }
            .normalize(),
        }
    }

    pub fn render_pixel(&self, x: usize, y: usize) -> Color {
        let ray = self.create_prime(x, y);
        let color = self.handle_ray(Some(ray), 1.);

        color.unwrap_or(Color {
            r: 0x99,
            g: 0xCC,
            b: 0xFF,
        })
    }

    pub fn render(&self) -> impl Iterator<Item = (usize, usize, Color)> + '_ {
        let pairs = (0..self.width).flat_map(move |x| (0..self.height).map(move |y| (x, y)));

        pairs.map(move |(x, y)| (x, y, self.render_pixel(x, y)))
    }

    fn handle_object<T: Object>(
        &self,
        obj: &T,
        closest: f64,
        ray: &Ray,
        acc_factor: f64,
    ) -> (Option<Color>, f64) {
        let mut closest = closest;
        let mut color = None;
        let dist = obj.intersect(ray);
        if let Some(dist) = dist.filter(|x| x < &closest) {
            closest = dist;
            let hit_point = ray.direction * dist + ray.origin;

            let surf_normal = obj.normal(&hit_point);
            let light_dir = self.light.direction * -1.;

            let shadow_origin = hit_point + (surf_normal * SHADOW_BIAS);
            let shadow_ray = Ray {
                origin: shadow_origin,
                direction: light_dir,
            };

            let flag = self
                .objects
                .iter()
                .any(|o| o.intersect(&shadow_ray).is_some());
            let intensity = if flag {
                0.
            } else {
                let dot = (surf_normal ^ light_dir) * self.light.intensity;
                if dot > 0. {
                    dot
                } else {
                    0.
                }
            };
            let light_reflected = obj.material().albedo / std::f64::consts::PI;
            let mut color0 =
                (obj.material().color * self.light.color) * (intensity * light_reflected);
            let reflected_ray = Some(ray.reflect(shadow_origin, surf_normal));
            let refl = obj.material().reflectivity;

            if let Some(color_reflected) = self.handle_ray(reflected_ray, acc_factor * refl) {
                // reflection of some object
                color0 = color0 * (1. - refl) + color_reflected * refl;
            }
            color = Some(color0);
        }
        (color, closest)
    }

    fn handle_ray(&self, ray: Option<Ray>, acc_factor: f64) -> Option<Color> {
        if acc_factor < 1e-3 {
            return None;
        }
        let ray = ray?;
        let (mut color, mut closest) =
            self.handle_object(&self.plane, std::f64::INFINITY, &ray, acc_factor);
        for obj in self.objects.iter() {
            let (co, cl) = self.handle_object(obj, closest, &ray, acc_factor);
            color = co.or(color);
            closest = cl;
        }
        color
    }

    pub fn update(&mut self, angle: f64, center: Vector3) {
        for mut obj in self.objects.iter_mut() {
            obj.center = obj.center - center;
            obj.center = obj.center.rotateY(angle);
            obj.center = obj.center + center;
        }
    }
}
