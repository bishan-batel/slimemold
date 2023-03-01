use std::ffi::{CStr, CString};
use gl::types::{GLchar, GLenum, GLint, GLuint};

#[repr(u32)]
pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
    Compute = gl::COMPUTE_SHADER,
    Geometry = gl::GEOMETRY_SHADER,
}

unsafe fn shader_from_source(source: &str, kind: ShaderType) -> Result<GLuint, String> {
    let id = gl::CreateShader(kind as GLenum);

    let source = CString::new(source).unwrap();

    gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
    gl::CompileShader(id);

    let mut success = 1;

    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    // checks if compilation was successful
    if success == 0 {
        let mut len = 0;
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

        let buffer = vec![b' '; len as usize];
        let error = CString::from_vec_unchecked(buffer);

        gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut GLchar,
        );

        Err(error.to_string_lossy().into_owned())
    } else {
        Ok(id)
    }
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &str, kind: ShaderType) -> Result<Shader, String> {
        let id = unsafe { shader_from_source(source, kind)? };
        Ok(Shader { id })
    }

    pub fn from_vertex_source(source: &str) -> Result<Shader, String> {
        Self::from_source(source, ShaderType::Vertex)
    }

    pub fn from_frag_source(source: &str) -> Result<Shader, String> {
        Self::from_source(source, ShaderType::Fragment)
    }

    #[inline]
    pub const fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id)
        }
    }
}

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        unsafe {
            let id = gl::CreateProgram();

            // attach each shader
            for shader in shaders {
                gl::AttachShader(id, shader.id());
            }

            // link program
            gl::LinkProgram(id);

            // detach all shaders
            for shader in shaders {
                gl::DetachShader(id, shader.id());
            }

            // check if succeeded
            let mut success = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut len = 0;

                // get cstr error
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let error = CString::from_vec_unchecked(vec![b' '; len as usize]);

                // convert to rust string
                Err(error.to_string_lossy().into_owned())
            } else {
                Ok(Program { id })
            }
        }
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id)
        }
    }

    #[inline]
    pub const fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ComputeProgram {
    program: Program,
}

impl ComputeProgram {
    pub fn from_source(src: &str) -> Result<Self, String> {
        let shader = Shader::from_source(src, ShaderType::Compute)?;
        let program = Program::from_shaders(&[shader])?;

        Ok(ComputeProgram {
            program
        })
    }

    pub fn execute(&self, x: u32, y: u32, z: u32) {
        self.program.set_used();
        unsafe {
            gl::DispatchCompute(x, y, z);
            gl::MemoryBarrier(gl::ALL_BARRIER_BITS);
        }
    }

    pub fn set_used(&self) {
        self.program.set_used();
    }

    pub const fn program(&self) -> &Program {
        &self.program
    }
}

pub struct Uniform {
    location: GLint,
}

impl Uniform {
    pub fn compute(compute: &ComputeProgram, name: &str) -> Uniform {
        Uniform::program(compute.program(), name)
    }
    pub fn program(program: &Program, name: &str) -> Uniform {
        program.set_used();

        let cstr = CString::new(name).expect(format!("Invalid uniform name {name}").as_str());

        unsafe {
            Uniform {
                location: gl::GetUniformLocation(program.id, cstr.as_ptr())
            }
        }
    }

    pub fn set_vec2(&self, v: (f32, f32)) {
        unsafe {
            gl::Uniform2f(self.location, v.0, v.1);
        }
    }

    pub fn set_float(&self, n: f32) {
        unsafe {
            gl::Uniform1f(self.location, n);
        }
    }

    pub const fn location(&self) -> GLint {
        self.location
    }
}