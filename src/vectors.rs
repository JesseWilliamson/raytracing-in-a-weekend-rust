use std::{io::Write, ops};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
pub type Color = Vec3;
pub type Point3 = Vec3;

impl Color {
    pub fn write_color<W: Write>(self, out: &mut W) -> Result<(), std::io::Error> {
        let r = self.x;
        let g = self.y;
        let b = self.z;

        let r_byte = (255.999 * r) as i32;
        let g_byte = (255.999 * g) as i32;
        let b_byte = (255.999 * b) as i32;

        write!(out, "{} {} {}\n", r_byte, g_byte, b_byte)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

pub fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc = r.origin() - center;
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(oc, r.direction());
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn color(self) -> Color {
        let unit_direction = unit_vector(self.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    pub fn ray_color(self) -> Color {
        if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, self) {
            Color::new(1.0, 0.0, 0.0)
        } else {
            let unit_direction = unit_vector(self.direction());
            let a = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + Color::new(0.5, 0.7, 1.0) * a
        }
    }
}
