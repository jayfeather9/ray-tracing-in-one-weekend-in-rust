use crate::utils;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn same(v: f64) -> Self {
        Self { x: v, y: v, z: v }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random() -> Self {
        Self::new(
            utils::random_double(),
            utils::random_double(),
            utils::random_double(),
        )
    }

    pub fn random_in(min: f64, max: f64) -> Self {
        Self::new(
            utils::random_double_in(min, max),
            utils::random_double_in(min, max),
            utils::random_double_in(min, max),
        )
    }

    // pub fn random_unit() -> Self {
    //     let a = utils::random_double_in(0.0, 2.0 * utils::PI);
    //     let z = utils::random_double_in(-1.0, 1.0);
    //     let r = (1.0 - z * z).sqrt();
    //     Self::new(r * a.cos(), r * a.sin(), z)
    // }

    pub fn random_unit() -> Self {
        loop {
            let p = Self::random_in(-1.0, 1.0);
            if p.dot_square() < 1.0 {
                return p.unit();
            }
        }
    }

    pub fn random_on_hemi(normal: &Self) -> Self {
        let rand_in_unit = Self::random_unit();
        if rand_in_unit.dot(normal) > 0.0 {
            rand_in_unit
        } else {
            -rand_in_unit
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        self.dot_square().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn dot_square(&self) -> f64 {
        self.dot(self)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            -(self.x * rhs.z - self.z * rhs.x),
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
