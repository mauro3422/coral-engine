use cgmath::Vector3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis3D {
    X,
    Y,
    Z,
}

impl Axis3D {
    pub const ALL: [Axis3D; 3] = [Axis3D::X, Axis3D::Y, Axis3D::Z];

    pub const fn index(self) -> usize {
        match self {
            Axis3D::X => 0,
            Axis3D::Y => 1,
            Axis3D::Z => 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CoordinateSystem3D {
    pub up_axis: Axis3D,
    pub forward_axis: Axis3D,
}

impl CoordinateSystem3D {
    pub const fn standard() -> Self {
        Self {
            up_axis: Axis3D::Y,
            forward_axis: Axis3D::Z,
        }
    }

    pub fn right_axis(&self) -> Axis3D {
        for axis in Axis3D::ALL {
            if axis != self.up_axis && axis != self.forward_axis {
                return axis;
            }
        }

        Axis3D::X
    }

    pub fn horizontal_axes(&self) -> (Axis3D, Axis3D) {
        (self.right_axis(), self.forward_axis)
    }

    pub fn axis_vector(axis: Axis3D) -> Vector3<f32> {
        match axis {
            Axis3D::X => Vector3::new(1.0, 0.0, 0.0),
            Axis3D::Y => Vector3::new(0.0, 1.0, 0.0),
            Axis3D::Z => Vector3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn axis_color(axis: Axis3D) -> [f32; 3] {
        match axis {
            Axis3D::X => [0.95, 0.25, 0.25],
            Axis3D::Y => [0.25, 0.90, 0.35],
            Axis3D::Z => [0.25, 0.45, 0.95],
        }
    }

    pub fn muted_axis_color(axis: Axis3D) -> [f32; 3] {
        let c = Self::axis_color(axis);
        [c[0] * 0.45, c[1] * 0.45, c[2] * 0.45]
    }

    pub fn plane_offset(&self) -> f32 {
        0.02
    }

    pub fn axis_component(point: [f32; 3], axis: Axis3D) -> f32 {
        point[axis.index()]
    }

    pub fn set_axis_component(point: &mut [f32; 3], axis: Axis3D, value: f32) {
        point[axis.index()] = value;
    }
}

impl Default for CoordinateSystem3D {
    fn default() -> Self {
        Self::standard()
    }
}
