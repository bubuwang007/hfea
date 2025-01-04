use super::Vector3d;
use std::ops::{Add, Div, Mul, Sub};

impl Sub for Vector3d {
    type Output = Vector3d;

    fn sub(self, other: Vector3d) -> Self::Output {
        Vector3d {
            dx: self.dx - other.dx,
            dy: self.dy - other.dy,
            dz: self.dz - other.dz,
        }
    }
}

impl Sub<f64> for Vector3d {
    type Output = Vector3d;

    fn sub(self, scalar: f64) -> Self::Output {
        Vector3d {
            dx: self.dx - scalar,
            dy: self.dy - scalar,
            dz: self.dz - scalar,
        }
    }
}

impl Mul for Vector3d {
    type Output = Vector3d;

    fn mul(self, other: Vector3d) -> Self::Output {
        Vector3d {
            dx: self.dx * other.dx,
            dy: self.dy * other.dy,
            dz: self.dz * other.dz,
        }
    }
}

impl Vector3d {
    pub fn dot(&self, other: Vector3d) -> f64 {
        self.dx * other.dx + self.dy * other.dy + self.dz * other.dz
    }

    pub fn dot_ref(&self, other: &Vector3d) -> f64 {
        self.dx * other.dx + self.dy * other.dy + self.dz * other.dz
    }

    pub fn cross(&self, other: Vector3d) -> Vector3d {
        Vector3d {
            dx: self.dy * other.dz - self.dz * other.dy,
            dy: self.dz * other.dx - self.dx * other.dz,
            dz: self.dx * other.dy - self.dy * other.dx,
        }
    }

    pub fn cross_ref(&self, other: &Vector3d) -> Vector3d {
        Vector3d {
            dx: self.dy * other.dz - self.dz * other.dy,
            dy: self.dz * other.dx - self.dx * other.dz,
            dz: self.dx * other.dy - self.dy * other.dx,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector3d {
            dx: 1.0,
            dy: 2.0,
            dz: 3.0,
        };
        let v2 = Vector3d {
            dx: 4.0,
            dy: 5.0,
            dz: 6.0,
        };
        let v3 = v1 + v2;
        assert_eq!(v3.dx, 5.0);
        assert_eq!(v3.dy, 7.0);
        assert_eq!(v3.dz, 9.0);
    }

    #[test]
    fn test_add_scalar() {
        let v1 = Vector3d {
            dx: 1.0,
            dy: 2.0,
            dz: 3.0,
        };
        let v2 = v1 + 2.0;
        assert_eq!(v2.dx, 3.0);
        assert_eq!(v2.dy, 4.0);
        assert_eq!(v2.dz, 5.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector3d {
            dx: 1.0,
            dy: 2.0,
            dz: 3.0,
        };
        let v2 = Vector3d {
            dx: 4.0,
            dy: 5.0,
            dz: 6.0,
        };
        let v3 = v1 - v2;
        assert_eq!(v3.dx, -3.0);
        assert_eq!(v3.dy, -3.0);
        assert_eq!(v3.dz, -3.0);
    }

    #[test]
    fn test_sub_scalar() {
        let v1 = Vector3d {
            dx: 1.0,
            dy: 2.0,
            dz: 3.0,
        };
        let v2 = v1 - 2.0;
        assert_eq!(v2.dx, -1.0);
        assert_eq!(v2.dy, 0.0);
        assert_eq!(v2.dz, 1.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector3d {
            dx: 1.0,
            dy: 2.0,
            dz: 3.0,
        };
        let v2 = Vector3d {
            dx: 4.0,
            dy: 5.0,
            dz: 6.0,
        };
        let dot = v1.dot(v2);
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn test_dot_ref() {
        let v1 = Vector3d {
            dx: 1.0,
            dy: 2.0,
            dz: 3.0,
        };
        let v2 = Vector3d {
            dx: 4.0,
            dy: 5.0,
            dz: 6.0,
        };
        let dot = v1.dot_ref(&v2);
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vector3d {
            dx: 1.0,
            dy: 2.0,
            dz: 3.0,
        };
        let v2 = Vector3d {
            dx: 4.0,
            dy: 5.0,
            dz: 6.0,
        };
        let v3 = v1.cross(v2);
        assert_eq!(v3.dx, -3.0);
        assert_eq!(v3.dy, 6.0);
        assert_eq!(v3.dz, -3.0);
    }

    #[test]
    fn test_cross_ref() {
        let v1 = Vector3d {
            dx: 1.0,
            dy: 2.0,
            dz: 3.0,
        };
        let v2 = Vector3d {
            dx: 4.0,
            dy: 5.0,
            dz: 6.0,
        };
        let v3 = v1.cross_ref(&v2);
        assert_eq!(v3.dx, -3.0);
        assert_eq!(v3.dy, 6.0);
        assert_eq!(v3.dz, -3.0);
    }
}
