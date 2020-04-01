use super::utils::Vector3;
use super::*;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub material: Material,
}

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub albedo: f64,
    pub reflectivity: f64,
}

pub trait Object: Copy {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn normal(&self, hit_point: &Vector3) -> Vector3;
    fn material(&self) -> &Material;
}

impl Object for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn normal(&self, hit_point: &Vector3) -> Vector3 {
        (*hit_point - self.center).normalize()
    }

    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let C = self.center;
        // ray: P = O + t * D
        let O = ray.origin;
        let D = ray.direction;

        let L = C - O;

        let d = L ^ D;
        let h2 = (L ^ L) - (d * d);
        let s2 = self.radius * self.radius - h2;

        if s2 < 0. {
            return None;
        }

        let s = s2.sqrt();
        let t0 = d - s;

        if t0 > 0. {
            Some(t0)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Plane {
    pub origin: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let proj = self.normal ^ ray.direction;

        if proj < 0. {
            let d = ((self.origin - ray.origin) ^ self.normal) / proj;
            // ray: p = ray.origin + ray.direction * d
            if d >= 0. {
                return Some(d);
            }
        }
        return None;
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn normal(&self, _: &Vector3) -> Vector3 {
        self.normal
    }
}
