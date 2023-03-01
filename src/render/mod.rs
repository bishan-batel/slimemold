use std::mem::size_of;

pub mod shader;
pub mod vertex_arrays;
pub mod buffer;
pub mod texture;
pub mod color;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum GlDataType {
    Byte = gl::BYTE,
    UnsignedByte = gl::UNSIGNED_BYTE,
    Short = gl::SHORT,
    UnsignedShort = gl::UNSIGNED_SHORT,
    Int = gl::INT,
    UnsignedInt = gl::UNSIGNED_INT,
    Fixed = gl::FIXED,
    Half = gl::HALF_FLOAT,
    Float = gl::FLOAT,
    Double = gl::DOUBLE,
}

impl GlDataType {
    fn size(&self) -> usize {
        match self {
            GlDataType::Byte => size_of::<i8>(),
            GlDataType::UnsignedByte => size_of::<u8>(),
            GlDataType::Short => size_of::<i16>(),
            GlDataType::UnsignedShort => size_of::<u16>(),
            GlDataType::Int => size_of::<i32>(),
            GlDataType::UnsignedInt => size_of::<u32>(),
            GlDataType::Fixed => size_of::<i32>(),
            GlDataType::Half => 16,
            GlDataType::Float => size_of::<f32>(),
            GlDataType::Double => size_of::<f64>(),
        }
    }
}