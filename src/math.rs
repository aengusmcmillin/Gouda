use std::ops::Mul;
use std::f32;

pub struct Vec4 {
    pub v1: f32,
    pub v2: f32,
    pub v3: f32,
    pub v4: f32
}

pub struct Mat4x4 {
    pub data: [[f32; 4]; 4]
}

impl Mat4x4 {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Self {
            data
        }
    }

    pub fn get_val(&self, row: usize, col: usize) {
        self.data[row][col];
    }

    pub fn raw_data(&self) -> [f32; 16] {
        return [
            self.data[0][0], self.data[0][1], self.data[0][2], self.data[0][3],
            self.data[1][0], self.data[1][1], self.data[1][2], self.data[1][3],
            self.data[2][0], self.data[2][1], self.data[2][2], self.data[2][3],
            self.data[3][0], self.data[3][1], self.data[3][2], self.data[3][3],
        ];
    }

    pub fn to_vec(&self) -> Vec<f32> {
        return vec![
        self.data[0][0], self.data[0][1], self.data[0][2], self.data[0][3],
        self.data[1][0], self.data[1][1], self.data[1][2], self.data[1][3],
        self.data[2][0], self.data[2][1], self.data[2][2], self.data[2][3],
        self.data[3][0], self.data[3][1], self.data[3][2], self.data[3][3]];
    }
}

impl Mul for Mat4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data = [
            [
                self.data[0][0] * rhs.data[0][0] + self.data[0][1] * rhs.data[1][0] + self.data[0][2] * rhs.data[2][0] + self.data[0][3] * rhs.data[3][0],
                self.data[0][0] * rhs.data[0][1] + self.data[0][1] * rhs.data[1][1] + self.data[0][2] * rhs.data[2][1] + self.data[0][3] * rhs.data[3][1],
                self.data[0][0] * rhs.data[0][2] + self.data[0][1] * rhs.data[1][2] + self.data[0][2] * rhs.data[2][2] + self.data[0][3] * rhs.data[3][2],
                self.data[0][0] * rhs.data[0][3] + self.data[0][1] * rhs.data[1][3] + self.data[0][2] * rhs.data[2][3] + self.data[0][3] * rhs.data[3][3],
            ],
            [
                self.data[1][0] * rhs.data[0][0] + self.data[1][1] * rhs.data[1][0] + self.data[1][2] * rhs.data[2][0] + self.data[1][3] * rhs.data[3][0],
                self.data[1][0] * rhs.data[0][1] + self.data[1][1] * rhs.data[1][1] + self.data[1][2] * rhs.data[2][1] + self.data[1][3] * rhs.data[3][1],
                self.data[1][0] * rhs.data[0][2] + self.data[1][1] * rhs.data[1][2] + self.data[1][2] * rhs.data[2][2] + self.data[1][3] * rhs.data[3][2],
                self.data[1][0] * rhs.data[0][3] + self.data[1][1] * rhs.data[1][3] + self.data[1][2] * rhs.data[2][3] + self.data[1][3] * rhs.data[3][3],
            ],
            [
                self.data[1][0] * rhs.data[0][0] + self.data[2][1] * rhs.data[1][0] + self.data[2][2] * rhs.data[2][0] + self.data[2][3] * rhs.data[3][0],
                self.data[1][0] * rhs.data[0][1] + self.data[2][1] * rhs.data[1][1] + self.data[2][2] * rhs.data[2][1] + self.data[2][3] * rhs.data[3][1],
                self.data[1][0] * rhs.data[0][2] + self.data[2][1] * rhs.data[1][2] + self.data[2][2] * rhs.data[2][2] + self.data[2][3] * rhs.data[3][2],
                self.data[1][0] * rhs.data[0][3] + self.data[2][1] * rhs.data[1][3] + self.data[2][2] * rhs.data[2][3] + self.data[2][3] * rhs.data[3][3],
            ],
            [
                self.data[1][0] * rhs.data[0][0] + self.data[3][1] * rhs.data[1][0] + self.data[3][2] * rhs.data[2][0] + self.data[3][3] * rhs.data[3][0],
                self.data[1][0] * rhs.data[0][1] + self.data[3][1] * rhs.data[1][1] + self.data[3][2] * rhs.data[2][1] + self.data[3][3] * rhs.data[3][1],
                self.data[1][0] * rhs.data[0][2] + self.data[3][1] * rhs.data[1][2] + self.data[3][2] * rhs.data[2][2] + self.data[3][3] * rhs.data[3][2],
                self.data[1][0] * rhs.data[0][3] + self.data[3][1] * rhs.data[1][3] + self.data[3][2] * rhs.data[2][3] + self.data[3][3] * rhs.data[3][3],
            ],
        ];
        return Mat4x4{data};
    }
}

pub fn create_transformation_matrix(translate: [f32; 3], x: f32, y: f32, z: f32, scale: f32) -> Mat4x4 {
    let transform_mat = Mat4x4 {
        data: [
            [scale, 0., 0., translate[0]],
            [0., scale, 0., translate[1]],
            [0., 0., scale, translate[2]],
            [0., 0., 0., 1.],
        ]
    };

    let x = x * f32::consts::PI/180.0;
    let xrot = Mat4x4 {
        data: [
            [1., 0., 0., 0.],
            [0., x.cos(), -x.sin(), 0.],
            [0., x.sin(), x.cos(), 0.],
            [0., 0., 0., 1.],
        ]
    };

    let y = y * f32::consts::PI/180.0;
    let yrot = Mat4x4 {
        data: [
            [y.cos(), 0., y.sin(), 0.],
            [0., 1., 0., 0.],
            [-y.sin(), 0., y.cos(), 0.],
            [0., 0., 0., 1.],
        ]
    };

    let z = z * f32::consts::PI/180.0;
    let zrot = Mat4x4 {
        data: [
            [z.cos(), -z.sin(), 0., 0.],
            [z.sin(), z.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]
    };

    let transform_mat = transform_mat * xrot * yrot * zrot;
    return transform_mat;
}
