use ndarray::{arr2, Array2};

use crate::structs::vector2d::Vector2D;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z}
    }

    pub fn to_tuple(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    pub fn add_scalar_z2(&self) -> Vector3D {
        Vector3D {
            x: self.x,
            y: self.y,
            z: self.z + 2,
        }
    }

    pub fn mirror_z(&self) -> Vector3D {
        Vector3D {
            x: self.x,
            y: self.y,
            z: -self.z,
        }
    }

    pub fn from_2d(vec: &Vector2D, z: i32) -> Vector3D {
        Vector3D {
            x: vec.x,
            y: vec.y,
            z,
        }
    }

    pub fn to_2d(&self) -> Vector2D {
        Vector2D {
            x: self.x,
            y: self.y,
        }
    }

    // Immutable borrowing
    fn x(&self) -> &i32 {
            &self.x
        }
        
}


impl Into<Array2<i32>> for Vector3D {
    fn into(self) -> Array2<i32> {
        arr2(&[[self.x], [self.y], [self.z]])
    }
}

