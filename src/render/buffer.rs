use std::mem;
use std::mem::size_of;
use gl::types::{GLenum, GLintptr, GLsizei, GLsizeiptr, GLuint, GLvoid};

pub struct BufferObject {
    id: GLuint,
    kind: BufferType,
    size: GLsizeiptr,
}

impl BufferObject {
    pub fn create_vbo<T>(data: &[T], usage: BufferUsage) -> Self where T: Sized {
        Self::with_data(BufferType::ArrayBuffer, data, usage)
    }

    pub fn with_data<T>(kind: BufferType, data: &[T], usage: BufferUsage) -> Self where T: Sized {
        let mut buff = Self::gen(1, kind);
        buff.set_data(data, usage);
        buff
    }

    pub fn with_capacity<T>(kind: BufferType, size: usize, usage: BufferUsage) -> Self where T: Sized {
        let mut buff = Self::gen(1, kind);

        buff.bind();

        unsafe {
            gl::BufferData(
                buff.kind as GLenum,
                (size * size_of::<T>()) as GLsizeiptr,
                std::ptr::null(),
                usage as GLenum,
            );
        }
        buff.unbind();

        buff
    }

    pub fn gen(n: usize, kind: BufferType) -> Self {
        let mut bo = 0;
        unsafe {
            gl::GenBuffers(n as GLsizei, &mut bo);
        };

        Self {
            id: bo,
            size: 0,
            kind,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.kind as GLenum, self.id);
        }
    }

    pub fn set_data<T>(&mut self, data: &[T], usage: BufferUsage) where T: Sized {
        self.size = (data.len() * size_of::<T>()) as GLsizeiptr;

        self.bind();
        unsafe {
            gl::BufferData(
                self.kind as GLenum,
                self.size,
                data.as_ptr() as *const _,
                usage as GLenum,
            );
        }
        self.unbind();
    }

    pub fn copy_all_to(&self, dest: &mut BufferObject) {
        unsafe {
            self.copy_to(dest, 0, 0, self.size as usize);
        }
    }

    pub fn copy_to(&self, dest: &mut BufferObject, read_offset: usize, write_offset: usize, length: usize) {
        unsafe {
            gl::CopyBufferSubData(
                self.id,
                dest.id,
                read_offset as GLintptr,
                write_offset as GLintptr,
                length as GLsizeiptr,
            );
        }
    }

    pub fn bind_base(&self, layout: u32) {
        unsafe {
            gl::BindBufferBase(self.kind as GLenum, layout, self.id)
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.kind as GLenum, 0);
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BufferType {
    ArrayBuffer = gl::ARRAY_BUFFER,
    ElementArray = gl::ELEMENT_ARRAY_BUFFER,
    ShaderStorage = gl::SHADER_STORAGE_BUFFER,
    // TODO
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BufferUsage {
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,
    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,
    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
}