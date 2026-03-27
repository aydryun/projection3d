use std::fmt::{self, Display, Formatter};

use crate::geometry::axis::Axis;

pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn from_vec(coords_xyz: &[f32]) -> Self {
        if coords_xyz.len() > 3 {
            panic!("Error: vac param lenght shouldn't be longer than 3 (xyz)")
        }

        Point3D {
            x: coords_xyz[0],
            y: coords_xyz[1],
            z: coords_xyz[2],
        }
    }

    // Translation sur axe
    pub fn translate(&mut self, amount: f32, axis: Axis) {
        match axis {
            Axis::X => self.x += amount,
            Axis::Y => self.y += amount,
            Axis::Z => self.z += amount,
        }
    }

    /*     fn rotate_to_angle(&self, axis: Axis, angle: f32) {
         match axis {
               Axis::X =>,
               Axis::Y =>,
               Axis::Z => ,
         }
       }
    */
    pub fn set_y_rotation(&mut self, angle_radian: f32) {
        // hypot() = sqrt(x*x + z*z)
        let radius = self.x.hypot(self.z);

        self.x = angle_radian.cos() * radius;
        self.z = angle_radian.sin() * radius;
    }

    pub fn rotate_xz(&mut self, angle: &f32) {
        let sin = angle.sin();
        let cos = angle.cos();
        let old_x = self.x;
        let old_z = self.z;

        self.x = old_x * cos - old_z * sin;
        self.z = old_x * sin + old_z * cos;
    }

    pub fn project(&self) -> Point {
        Point {
            x: self.x / self.z,
            y: self.y / self.z,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "point [x: {}, y: {}]", self.x, self.y)
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "point3d [x: {}, y: {}, z: {}]", self.x, self.y, self.z)
    }
}
