use std::ops::{Add, Div, Mul, Neg, Sub};

// Type aliases
pub use Vec3 as Color;
pub use Vec3 as Point3;
use crate::random_f64;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn cross(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    /// Applies `f` to each element of the vector in turn, giving a new vector.
    #[inline]
    pub fn map(self, mut f: impl FnMut(f64) -> f64) -> Self {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    /// Create a random vector with its coordinates in a range specified by [min, max).
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_f64(min, max),
            y: random_f64(min, max),
            z: random_f64(min, max),
        }
    }

    /// Returns a random point in a unit radius sphere.
    pub fn random_in_unit_sphere() -> Point3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    /// Used for hemispherical scattering.
    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        return if in_unit_sphere.dot(normal) > 0.0 { // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        };
    }

    /// Returns true if the vector is close to zero in all dimensions.
    pub fn near_zero(self) -> bool {
        let epsilon = 1e-8;

        return (f64::abs(self.x) < epsilon) && (f64::abs(self.x) < epsilon)
            && (f64::abs(self.x) < epsilon);
    }

    /// Reflects the `v` vector around a surface normal.
    pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
        return v - 2.0 * v.dot(normal) * normal;
    }
}

// Operator overloading (+, -, unary -, *)
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}


/// Implements scalar * vector.
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::from(self) * other
    }
}

/// Implements vector * vector (Hadamard product).
impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        self.map(|x| x / other)
    }
}

impl From<f64> for Vec3 {
    /// Creates a vector (x,x,x) from a single value x.
    fn from(v: f64) -> Vec3 {
        Vec3 { x: v, y: v, z: v }
    }
}