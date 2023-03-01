use gl::types::{GLenum, GLint, GLsizei, GLuint};
use crate::render::color::{ColorInternal, ColorRepr};
use crate::render::GlDataType;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureTarget {
    Texture1d = gl::TEXTURE_1D,
    Texture2d = gl::TEXTURE_2D,
    Texture3d = gl::TEXTURE_3D,
    Texture1dArray = gl::TEXTURE_1D_ARRAY,
    Texture2dArray = gl::TEXTURE_2D_ARRAY,
    TextureRectangle = gl::TEXTURE_RECTANGLE,
    TextureCubeMap = gl::TEXTURE_CUBE_MAP,
    TextureCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
    TextureBuffer = gl::TEXTURE_BUFFER,
    Texture2dMultisample = gl::TEXTURE_2D_MULTISAMPLE,
    Texture2dMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ImageAccess {
    Read = gl::READ_ONLY,
    Write = gl::WRITE_ONLY,
    ReadWrite = gl::READ_WRITE,
}

pub struct Texture {
    id: GLuint,
    target: TextureTarget,
}

impl Texture {
    pub fn fill_image_2d<T>(&self, internal_format: ColorInternal, format: ColorRepr, width: u32, height: u32, pixel_data_type: GlDataType, data: &[T]) {
        unsafe {
            self.bind();
            gl::TexImage2D(
                self.target as GLenum,
                0,
                internal_format as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                format as GLenum,
                pixel_data_type as GLenum,
                data.as_ptr() as *const _,
            );
        }
    }

    pub fn generate_mipmaps(&self) {
        unsafe {
            gl::GenerateMipmap(self.target as GLenum)
        }
    }

    pub fn gen(n: usize, target: TextureTarget) -> Self {
        unsafe {
            let mut id = 0;
            gl::GenTextures(n as GLsizei, &mut id);
            let tex = Self {
                id,
                target,
            };
            tex
        }
    }

    pub fn texture_2d() -> Self {
        let tex = Texture::gen(1, TextureTarget::Texture2d);

        tex.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        }

        tex
    }

    pub fn bind_image_texture(&self, format: ColorInternal, access: ImageAccess) {
        unsafe {
            gl::BindImageTexture(
                0,
                self.id,
                0,
                gl::FALSE,
                0,
                access as GLenum,
                format as GLenum,
                // gl::RGBA32F,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(self.target as GLenum, self.id);
        }
    }

    pub fn set_active(tex: usize) {
        unsafe {
            gl::ActiveTexture(match tex {
                0 => gl::TEXTURE0,
                1 => gl::TEXTURE1,
                2 => gl::TEXTURE2,
                3 => gl::TEXTURE3,
                4 => gl::TEXTURE4,
                5 => gl::TEXTURE5,
                6 => gl::TEXTURE6,
                7 => gl::TEXTURE7,
                8 => gl::TEXTURE8,
                9 => gl::TEXTURE9,
                10 => gl::TEXTURE10,
                11 => gl::TEXTURE11,
                12 => gl::TEXTURE12,
                13 => gl::TEXTURE13,
                14 => gl::TEXTURE14,
                15 => gl::TEXTURE15,
                16 => gl::TEXTURE16,
                17 => gl::TEXTURE17,
                18 => gl::TEXTURE18,
                19 => gl::TEXTURE19,
                20 => gl::TEXTURE20,
                21 => gl::TEXTURE21,
                22 => gl::TEXTURE22,
                23 => gl::TEXTURE23,
                24 => gl::TEXTURE24,
                25 => gl::TEXTURE25,
                26 => gl::TEXTURE26,
                27 => gl::TEXTURE27,
                28 => gl::TEXTURE28,
                29 => gl::TEXTURE29,
                30 => gl::TEXTURE30,
                31 => gl::TEXTURE31,
                _ => panic!("Invalid texture active {}", tex)
            });
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}