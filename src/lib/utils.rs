#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn clamp(x: f64) -> u8 {
    if x > 255. {
        255
    } else if x < 0. {
        0
    } else {
        x as u8
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, factor: f64) -> Color {
        Color {
            r: clamp((self.r as f64) * factor),
            g: clamp((self.g as f64) * factor),
            b: clamp((self.b as f64) * factor),
        }
    }
}

fn mult_u8(a: u8, b: u8) -> u8 {
    ((a as u32) * (b as u32) / 255) as u8
}

#[test]
fn test_mult_colors() {
    let left = Color {
        r: 0xAA,
        g: 0x55,
        b: 0x22,
    };

    assert_eq!(
        left * left,
        Color {
            r: 113,
            g: 28,
            b: 4,
        },
    )
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: mult_u8(self.r, other.r),
            g: mult_u8(self.g, other.g),
            b: mult_u8(self.b, other.b),
        }
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn reflect(&self, hit_point: Vector3, normal: Vector3) -> Ray {
        let incident = hit_point - self.origin;
        let d = normal * (incident ^ normal * 2.);
        Ray {
            origin: hit_point,
            direction: incident - d,
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::BitXor for Vector3 {
    type Output = f64;

    fn bitxor(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Vector3 {
    pub fn norm(self) -> f64 {
        (self ^ self).sqrt()
    }

    pub fn normalize(self) -> Vector3 {
        let norm = self.norm();
        if norm.abs() < 1e-13 {
            Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            }
        } else {
            self * (1. / norm)
        }
    }

    pub fn rotateZ(&self, angle: f64) -> Vector3 {
        let (s, c) = angle.sin_cos();
        Vector3 {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
            z: self.z,
        }
    }

    pub fn rotateY(&self, angle: f64) -> Vector3 {
        let (s, c) = angle.sin_cos();
        Vector3 {
            x: self.x * c - self.z * s,
            y: self.y,
            z: self.x * s + self.z * c,
        }
    }

    pub fn rotateX(&self, angle: f64) -> Vector3 {
        let (s, c) = angle.sin_cos();
        Vector3 {
            x: self.x,
            y: self.y * c - self.z * s,
            z: self.y * s + self.z * c,
        }
    }
}
