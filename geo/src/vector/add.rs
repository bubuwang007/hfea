use super::Vector3d;
use std::ops::Add;

/*
v+v
*/
impl Add for Vector3d {
    type Output = Vector3d;

    fn add(self, other: Vector3d) -> Self::Output {
        Vector3d {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
            dz: self.dz + other.dz,
        }
    }
}

/*
v+&v
*/
impl Add<&Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(mut self, other: &Vector3d) -> Self::Output {
        self.dx += other.dx;
        self.dy += other.dy;
        self.dz += other.dz;
        self
    }

}

impl Add<f64> for Vector3d {
    type Output = Vector3d;

    fn add(mut self, scalar: f64) -> Self::Output {
        self.dx += scalar;
        self.dy += scalar;
        self.dz += scalar;
        self
    }
}