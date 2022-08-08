use std::ops::Mul;
use std::f32;

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

#[derive(Debug)]
pub struct Mat4x4 {
    pub data: [[f32; 4]; 4]
}

impl Mat4x4 {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Self {
            data
        }
    }

    pub fn identity() -> Self {
        return Mat4x4 {
            data: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]
        };
    }

    pub fn x_rot_matrix(x_rot_deg: f32) -> Self {
        let x_rot = x_rot_deg * f32::consts::PI/180.0;
        return Mat4x4 {
            data: [
                [1., 0., 0., 0.],
                [0., x_rot.cos(), -x_rot.sin(), 0.],
                [0., x_rot.sin(), x_rot.cos(), 0.],
                [0., 0., 0., 1.],
            ]
        };
    }

    pub fn y_rot_matrix(y_rot_deg: f32) -> Self {
        let y_rot = y_rot_deg * f32::consts::PI/180.0;
        return Mat4x4 {
            data: [
                [y_rot.cos(), 0., y_rot.sin(), 0.],
                [0., 1., 0., 0.],
                [-y_rot.sin(), 0., y_rot.cos(), 0.],
                [0., 0., 0., 1.],
            ]
        };
    }

    pub fn z_rot_matrix(z_rot_deg: f32) -> Self {
        let z_rot = z_rot_deg * f32::consts::PI/180.0;
        return Mat4x4 {
            data: [
                [z_rot.cos(), -z_rot.sin(), 0., 0.],
                [z_rot.sin(), z_rot.cos(), 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]
        };
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

pub fn create_view_matrix(pitch: f32, yaw: f32, camera_pos: [f32; 3]) -> Mat4x4 {
    let pitch_rotate = Mat4x4::x_rot_matrix(pitch);
    let yaw_rotate = Mat4x4::y_rot_matrix(yaw);
    let mut rotated = Mat4x4::identity() * pitch_rotate * yaw_rotate;
    rotated.data[0][3] = -camera_pos[0];
    rotated.data[1][3] = -camera_pos[1];
    rotated.data[2][3] = -camera_pos[2];
    return rotated;
}

pub fn create_projection_matrix(aspect: f32, fov: f32, zfar: f32, znear: f32) -> Mat4x4 {
    let fov = fov * f32::consts::PI/180.0;
    let zm = zfar - znear;
    let zp = zfar + znear;
    let xscale = (1./(fov / 2.).tan()) / aspect;
    let yscale = 1./(fov / 2.).tan();
    let zscale = -zp / zm;
    let ztranslate = -(2. * zfar * znear)/zm;

    return Mat4x4 {
        data: [
            [xscale, 0., 0., 0.],
            [0., yscale, 0., 0.],
            [0., 0., zscale, ztranslate],
            [0., 0., -1., 0.],
        ]
    };
}

pub fn create_transformation_matrix(translate: [f32; 3], rot: [f32; 3], scale: [f32; 3]) -> Mat4x4 {
    let transform_mat = Mat4x4 {
        data: [
            [1., 0., 0., translate[0]],
            [0., 1., 0., translate[1]],
            [0., 0., 1., translate[2]],
            [0., 0., 0., 1.],
        ]
    };

    let xrot = Mat4x4::x_rot_matrix(rot[0]);
    let yrot = Mat4x4::y_rot_matrix(rot[1]);
    let zrot = Mat4x4::z_rot_matrix(rot[2]);

    let scale_mat = Mat4x4 {
        data: [
            [scale[0], 0., 0., 0.],
            [0., scale[1], 0., 0.],
            [0., 0., scale[2], 0.],
            [0., 0., 0., 1.],
        ]
    };

    let transform_mat = zrot * yrot * xrot * transform_mat * scale_mat;
    return transform_mat;
}
