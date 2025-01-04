use std::ops::Deref;

pub mod add;

pub struct Vector3d {
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl Vector3d {
    pub fn new(dx: f64, dy: f64, dz: f64) -> Vector3d {
        Vector3d { dx, dy, dz }
    }

    pub fn new_x(dx: f64) -> Vector3d {
        Vector3d { dx, dy: 0.0, dz: 0.0 }
    }

    pub fn new_y(dy: f64) -> Vector3d {
        Vector3d { dx: 0.0, dy, dz: 0.0 }
    }

    pub fn new_z(dz: f64) -> Vector3d {
        Vector3d { dx: 0.0, dy: 0.0, dz }
    }

    pub fn length(&self) -> f64 {
        (self.dx * self.dx + self.dy * self.dy + self.dz * self.dz).sqrt()
    }

}

// impl Deref for Vector3d {
//     type Target = Vector3d;

//     fn deref(&self) -> &Self::Target {
//         self
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

}