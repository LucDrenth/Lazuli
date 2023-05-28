pub enum UniformValue {
    Float(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Float4(f32, f32, f32, f32),
    Int(i32),
    Int2(i32, i32),
    Int3(i32, i32, i32),
    Int4(i32, i32, i32, i32),
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
            }
        }
    }
}
