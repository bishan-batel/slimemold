use gl::types::{GLboolean, GLenum, GLint, GLsizei, GLuint, GLvoid};
use crate::render::GlDataType;
use crate::render::buffer::BufferObject;

struct AttributeDescription {
    kind: GlDataType,
    number: usize,
    normalized: bool,
}


pub struct AttributeLayout {
    attributes: Vec<AttributeDescription>,
    stride: usize,
}

impl AttributeLayout {
    pub fn number_normalized(&mut self, kind: GlDataType) -> &mut Self {
        self.attributes.push(AttributeDescription {
            kind,
            number: 1,
            normalized: true,
        });
        self.stride += kind.size();
        self
    }

    pub fn vector_normalized(&mut self, kind: GlDataType, size: usize) -> &mut Self {
        self.attributes.push(AttributeDescription {
            kind,
            number: size,
            normalized: true,
        });
        self.stride += kind.size();
        self
    }

    pub fn number(&mut self, kind: GlDataType) -> &mut Self {
        self.attributes.push(AttributeDescription {
            kind,
            number: 1,
            normalized: false,
        });
        self.stride += kind.size();
        self
    }

    pub fn vector(&mut self, kind: GlDataType, number: usize) -> &mut Self {
        self.attributes.push(AttributeDescription {
            kind,
            number,
            normalized: false,
        });
        self.stride += kind.size() * number;
        self
    }
}

pub struct VertexArrayObject {
    id: GLuint,
}

impl VertexArrayObject {
    pub fn new_arrays(vbo: &BufferObject, indices: Option<&BufferObject>, gen_layout: fn(&mut AttributeLayout)) -> Self {
        let mut id = 0;

        let mut layout = AttributeLayout {
            attributes: vec![],
            stride: 0,
        };
        gen_layout(&mut layout);
        let layout = layout;

        unsafe {
            gl::GenVertexArrays(1, &mut id);

            let vao = VertexArrayObject {
                id
            };

            vao.bind();
            vbo.bind();
            if let Some(indices) = indices {
                indices.bind();
            }

            let mut offset = 0;
            for (i, attribute) in layout.attributes.iter().enumerate() {
                gl::EnableVertexAttribArray(i as GLuint);
                gl::VertexAttribPointer(
                    i as GLuint,
                    attribute.number as GLint,
                    attribute.kind as GLenum,
                    attribute.normalized as GLboolean,
                    layout.stride as GLsizei,
                    offset as *const GLvoid,
                );
                offset += attribute.kind.size() * attribute.number;
            }

            vao.unbind();
            vbo.unbind();
            if let Some(indices) = indices {
                indices.unbind();
            }
            vao
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        VertexArrayObject::bind_null();
    }

    pub fn bind_null() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
