use gl::types::GLuint;
use glam::{Mat4, Vec2, Vec4, Vec3};

pub enum UniformValue {
    Float(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Float4(f32, f32, f32, f32),
    Int(i32),
    Int2(i32, i32),
    Int3(i32, i32, i32),
    Int4(i32, i32, i32, i32),
    U8_1(u8),
    U8_2(u8, u8),
    U8_3(u8, u8, u8),
    U8_4(u8, u8, u8, u8),
    Mat4(Mat4),
}

impl From<f32> for UniformValue {
    fn from(value: f32) -> Self {
        UniformValue::Float(value)
    }
}

impl From<(f32, f32)> for UniformValue {
    fn from(value: (f32, f32)) -> Self {
        UniformValue::Float2(value.0, value.1)
    }
}

impl From<(f32, f32, f32)> for UniformValue {
    fn from(value: (f32, f32, f32)) -> Self {
        UniformValue::Float3(value.0, value.1, value.2)
    }
}

impl From<(f32, f32, f32, f32)> for UniformValue {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        UniformValue::Float4(value.0, value.1, value.2, value.3)
    }
}

impl From<i32> for UniformValue {
    fn from(value: i32) -> Self {
        UniformValue::Int(value)
    }
}

impl From<(i32, i32)> for UniformValue {
    fn from(value: (i32, i32)) -> Self {
        UniformValue::Int2(value.0, value.1)
    }
}

impl From<(i32, i32, i32)> for UniformValue {
    fn from(value: (i32, i32, i32)) -> Self {
        UniformValue::Int3(value.0, value.1, value.2)
    }
}

impl From<(i32, i32, i32, i32)> for UniformValue {
    fn from(value: (i32, i32, i32, i32)) -> Self {
        UniformValue::Int4(value.0, value.1, value.2, value.3)
    }
}

impl From<u8> for UniformValue {
    fn from(value: u8) -> Self {
        UniformValue::U8_1(value)
    }
}

impl From<(u8, u8)> for UniformValue {
    fn from(value: (u8, u8)) -> Self {
        UniformValue::U8_2(value.0, value.1)
    }
}

impl From<(u8, u8, u8)> for UniformValue {
    fn from(value: (u8, u8, u8)) -> Self {
        UniformValue::U8_3(value.0, value.1, value.2)
    }
}

impl From<(u8, u8, u8, u8)> for UniformValue {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        UniformValue::U8_4(value.0, value.1, value.2, value.3)
    }
}

impl From<Mat4> for UniformValue {
    fn from(value: Mat4) -> Self {
        UniformValue::Mat4(value)
    }
}

impl From<Vec2> for UniformValue {
    fn from(value: Vec2) -> Self {
        UniformValue::Float2(value.x, value.y)
    }
}
impl From<Vec3> for UniformValue {
    fn from(value: Vec3) -> Self {
        UniformValue::Float3(value.x, value.y, value.z)
    }
}
impl From<Vec4> for UniformValue {
    fn from(value: Vec4) -> Self {
        UniformValue::Float4(value.x, value.y, value.z, value.w)
    }
}

impl UniformValue {
    pub fn set_uniform(&self, location: i32) {
        unsafe {
            match self {
                UniformValue::Float(value) => {
                    gl::Uniform1f(location, *value);
                },
                UniformValue::Float2(value1, value2) => {
                    gl::Uniform2f(location, *value1, *value2);
                },
                UniformValue::Float3(value1,value2, value3) => {
                    gl::Uniform3f(location, *value1, *value2, *value3);
                },
                UniformValue::Float4(value1, value2, value3, value4) => {
                    gl::Uniform4f(location, *value1, *value2, *value3, *value4);
                },
                UniformValue::Int(value) => {
                    gl::Uniform1i(location, *value);
                },
                UniformValue::Int2(value1, value2) => {
                    gl::Uniform2i(location, *value1, *value2);
                },
                UniformValue::Int3(value1, value2, value3) => {
                    gl::Uniform3i(location, *value1, *value2, *value3);
                },
                UniformValue::Int4(value1, value2, value3, value4) => {
                    gl::Uniform4i(location, *value1, *value2, *value3, *value4);
                },
                UniformValue::U8_1(value) => {
                    gl::Uniform1ui(location, *value as GLuint);
                },
                UniformValue::U8_2(value1, value2) => {
                    gl::Uniform2ui(location, *value1 as GLuint, *value2 as GLuint);
                },
                UniformValue::U8_3(value1, value2, value3) => {
                    gl::Uniform3ui(location, *value1 as GLuint, *value2 as GLuint, *value3 as GLuint);
                },
                UniformValue::U8_4(value1, value2, value3, value4) => {
                    gl::Uniform4ui(location, *value1 as GLuint, *value2 as GLuint, *value3 as GLuint, *value4 as GLuint);
                },
                UniformValue::Mat4(mat) => {
                    let ptr: *const f32 = std::mem::transmute(mat);
                    gl::UniformMatrix4fv(location, 1, gl::FALSE, ptr);
                }
            }
        }
    }
}
