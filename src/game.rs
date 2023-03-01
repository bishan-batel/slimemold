use std::collections::HashSet;
use std::f32::consts;
use std::mem::size_of;
use std::ptr;
use std::time::Instant;
use gl::types::{GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use image::DynamicImage::ImageRgba32F;
use image::{EncodableLayout, GenericImageView, Rgba32FImage};
use image::imageops::FilterType;
use rand::thread_rng;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseWheelDirection;
use sdl2::sys::rand;
use sdl2::video::{GLContext, GLProfile, Window};
use crate::{CELL_COUNT};

use crate::render::color::{ColorInternal, ColorRepr};
use crate::render::GlDataType;
use crate::render::shader::{ComputeProgram, Program, Shader, Uniform};
use crate::render::texture::{ImageAccess, Texture, TextureTarget};
use crate::render::vertex_arrays::{AttributeLayout, VertexArrayObject};
use crate::render::buffer::{BufferObject, BufferType, BufferUsage};

pub struct Game {
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    window: Window,
    event_pump: EventPump,
    gl_context: GLContext,
    running: bool,
    state: Option<GameState>,
    window_size: (i32, i32),
    keys_down: HashSet<Scancode>,

    cam_zoom: f32,
    cam_pos: (f32, f32),
    cam_zoom_vel: f32,
    cam_pos_vel: (f32, f32),
}


struct GameState {
    screen_vao: VertexArrayObject,
    screen_program: Program,
    diffuse_compute: ComputeProgram,
    cell_compute: ComputeProgram,
    trail_tex: Texture,
    cell_buffer: BufferObject,
}

impl Game {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sdl = sdl2::init()?;

        let video_subsystem = sdl.video()?;

        let event_pump = sdl.event_pump()?;

        // set GL versions
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);

        gl_attr.set_context_version(4, 3);

        let display = video_subsystem.desktop_display_mode(0).unwrap();

        // create window
        let window = video_subsystem
            .window("Gaming", display.w as u32, display.h as u32)
            .fullscreen()
            .opengl()
            .build()?;

        // bind context
        let gl_context = window.gl_create_context()?;
        gl::load_with(|f| video_subsystem.gl_get_proc_address(f) as *const std::os::raw::c_void);

        Ok(Self {
            sdl,
            gl_context,
            video_subsystem,
            cam_zoom: 1.,
            cam_zoom_vel: 0.,
            cam_pos: (0., 0.),
            cam_pos_vel: (0f32, 0f32),
            window_size: (window.size().0 as i32, window.size().1 as i32),
            window,
            event_pump,
            running: true,
            state: None,
            keys_down: HashSet::new(),
        })
    }

    pub fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.running = false,

                // Window Events
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => self.window_size = (width, height),
                    _ => {}
                },
                Event::MouseWheel { y, direction, .. } => {
                    let y = if let MouseWheelDirection::Flipped = direction {
                        -y
                    } else {
                        y
                    };
                    self.cam_zoom_vel = (self.cam_zoom_vel + y as f32).clamp(-10., 10.);
                }
                Event::KeyDown { scancode, .. } => if let Some(scancode) = scancode {
                    self.keys_down.insert(scancode);
                },
                Event::KeyUp { scancode, .. } => if let Some(scancode) = scancode {
                    self.keys_down.remove(&scancode);
                },
                _ => {}
            }
        }
    }

    pub unsafe fn init(&mut self) {
        gl::Viewport(0, 0, self.window_size.0 as i32, self.window_size.1 as i32);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);


        // let img = image::open("frame_00.jpg").unwrap();
        // let img = img.resize_exact(self.window_size.0 as u32, self.window_size.1 as u32, FilterType::Gaussian);
        // let mut img = img.into_rgba32f();

        let img = Rgba32FImage::new(
            self.window_size.0 as u32,
            self.window_size.1 as u32,
        );

        Texture::set_active(0);
        let tex = Texture::texture_2d();

        tex.fill_image_2d(
            ColorInternal::RGBA32F,
            ColorRepr::RGBA,
            img.width(),
            img.height(),
            GlDataType::Float,
            img.as_ref(),
        );

        // run compute
        let diffuse_compute = ComputeProgram::from_source(include_str!("shaders/diffusion.glsl")).unwrap();

        #[repr(C, packed)]
        struct Cell((f32, f32), f32);

        let mut cell_data = Vec::with_capacity(CELL_COUNT as usize);


        let mut rng = thread_rng();
        for i in 0..CELL_COUNT {
            use rand::prelude::*;

            let pos_ang = rng.gen::<f32>() * consts::TAU;
            // let dist = rng.gen::<f32>() * img.height() as f32 / 4.;
            let dist = img.height() as f32 / 4.;

            let x = pos_ang.cos() * dist + img.width() as f32 / 2.;
            let y = pos_ang.sin() * dist + img.height() as f32 / 2.;

            // let x = img.width() as f32 / 2.;
            // let y = img.height() as f32 / 2.;

            let angle: f32 = rng.gen::<f32>() * consts::TAU;
            // let angle: f32 = consts::TAU / 2.;
            cell_data.push(Cell((x, y), angle));
        }

        let cell_buffer = BufferObject::with_data(
            BufferType::ShaderStorage,
            cell_data.as_slice(),
            BufferUsage::StaticDraw,
        );

        let cell_compute = ComputeProgram::from_source(include_str!("shaders/cell.glsl")).unwrap();

        Uniform::compute(&cell_compute, "windowSize").set_vec2((img.width() as f32, img.height() as f32));
        Uniform::compute(&diffuse_compute, "windowSize").set_vec2((img.width() as f32, img.height() as f32));

        let vert = Shader::from_vertex_source(include_str!("shaders/triangle.vert")).unwrap();
        let frag = Shader::from_frag_source(include_str!("shaders/triangle.frag")).unwrap();

        let program = Program::from_shaders(&[vert, frag]).unwrap();

        #[repr(C, packed)]
        struct Vertex([f32; 2], [f32; 2]);
        const VERTICES: [Vertex; 4] = [
            Vertex([-1., -1.], [0.0, 1.0]),
            Vertex([1., -1.], [1.0, 1.0]),
            Vertex([1., 1.], [1.0, 0.0]),
            Vertex([-1., 1.], [0.0, 0.0]),
        ];

        const INDICES: [u32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        let vbo = BufferObject::create_vbo(
            &VERTICES,
            BufferUsage::StaticDraw,
        );

        let mut index_buffer = BufferObject::gen(1, BufferType::ElementArray);
        index_buffer.set_data(&INDICES, BufferUsage::StaticDraw);

        let tri_vao = VertexArrayObject::new_arrays(&vbo, Some(&index_buffer), |a| {
            a.vector(GlDataType::Float, 2); // position
            a.vector(GlDataType::Float, 2); // Tex Coord
        });


        self.state = Some(GameState {
            screen_program: program,
            screen_vao: tri_vao,
            cell_compute,
            cell_buffer,
            diffuse_compute,
            trail_tex: tex,
        });
    }

    pub fn update(&mut self, delta: f64) {
        const ACC: f32 = 0.1;
        const SPEED: f32 = 1.;
        const FRICTION: f32 = 0.94;
        const ZOOM_FRICTION: f32 = 0.94;
        let delta = delta as f32;

        let mut dir: (f32, f32) = (0., 0.);

        if self.keys_down.contains(&Scancode::A) {
            dir.0 += 1.;
        }
        if self.keys_down.contains(&Scancode::D) {
            dir.0 -= 1.;
        }
        if self.keys_down.contains(&Scancode::W) {
            dir.1 -= 1.;
        }
        if self.keys_down.contains(&Scancode::S) {
            dir.1 += 1.;
        }

        self.cam_pos_vel.0 += (dir.0 * delta * ACC).clamp(-SPEED, SPEED);
        self.cam_pos_vel.1 += (dir.1 * delta * ACC).clamp(-SPEED, SPEED);

        self.cam_pos_vel.0 *= FRICTION;
        self.cam_pos_vel.1 *= FRICTION;

        self.cam_pos.0 += self.cam_pos_vel.0;
        self.cam_pos.1 += self.cam_pos_vel.1;

        self.cam_zoom_vel *= ZOOM_FRICTION;
        self.cam_zoom = (self.cam_zoom + self.cam_zoom_vel * delta).clamp(1., 5.);

        let v = 1. / self.cam_zoom - 1.;
        self.cam_pos.0 = self.cam_pos.0.clamp(v, -v);
        self.cam_pos.1 = self.cam_pos.1.clamp(v, -v);
    }

    pub unsafe fn render(&mut self, mut delta: f64) {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        if let Some(state) = self.state.as_mut() {
            Texture::set_active(0);
            state.trail_tex.bind();

            // diffuse compute shader
            Uniform::compute(&state.diffuse_compute, "deltaTime").set_float(delta as f32);
            Texture::set_active(0);
            state.trail_tex.bind();
            state.trail_tex.bind_image_texture(ColorInternal::RGBA32F, ImageAccess::ReadWrite);
            state.diffuse_compute.execute(self.window_size.0 as u32 / 8, self.window_size.1 as u32 / 8, 1);

            Uniform::compute(&state.cell_compute, "deltaTime").set_float(delta as f32);
            state.cell_buffer.bind_base(1);
            state.cell_compute.execute(CELL_COUNT / 1024, 1, 1);


            state.screen_vao.bind();
            state.screen_program.set_used();

            Uniform::program(&state.screen_program, "camPos").set_vec2(self.cam_pos);
            Uniform::program(&state.screen_program, "zoom").set_float(self.cam_zoom);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        self.window.gl_swap_window();
    }

    #[inline]
    pub const fn is_running(&self) -> bool {
        self.running
    }
}