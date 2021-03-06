#[macro_use]
extern crate log;
extern crate env_logger;
extern crate cgmath;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate image;

use gfx::Device;
use glutin::GlContext;
use gfx::traits::FactoryExt;

mod obj;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
        // normal: [gfx::format::I8Norm; 4] = "a_Normal",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    constant Globals {
        mx_vp: [[f32; 4]; 4] = "u_ViewProj",
    }

    constant WireframeParam {
        width: f32 = "u_WireWidth",
        color: [f32; 4] = "u_WireColor",
    }

    pipeline wireframe_pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        cb_locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        cb_globals: gfx::ConstantBuffer<Globals> = "u_Globals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

// const VIEWPORT: [i32; 4] = [0, 0, 600, 800];

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const TEX_QUAD: [Vertex; 4] = [
    Vertex { // bottom right
        pos: [0.5, -0.5],
        uv: [1.0, 1.0],
    },
    Vertex { // bottom left
        pos: [-0.5, -0.5],
        uv: [0.0, 1.0],
    },
    Vertex { // top left
        pos: [-0.5, 0.5],
        uv: [0.0, 0.0],
    },
    Vertex { // top right
        pos: [0.5, 0.5],
        uv: [1.0, 0.0],
    },
];

// const TEX_INDICES: [u16; 6] = [0, 3, 1, 1, 3, 2];
const TEX_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

const LOCALS: Locals = Locals {
        transform: [[1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]],
};

pub fn main() {
    // initialize logger
    env_logger::init().unwrap();

    // initialize window
    let builder = glutin::WindowBuilder::new()
        .with_title("rbomberman".to_string())
        .with_dimensions(800, 600);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let mut events_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);

    // create encoder
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    // load textures
    let sampler = factory.create_sampler_linear();
    let texture = gfx_load_texture(&mut factory);
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TEX_QUAD, &TEX_INDICES[..]);
    let data = pipe::Data {
        vbuf: vertex_buffer,
        tex: (texture, sampler),
        transform: LOCALS.transform,
        // line_width: LOCALS.line_width,
        // line_color: LOCALS.line_color,
        out: main_color,
    };

    // load shaders
    // let shader_set = factory.create_shader_set_geometry(
    //     include_bytes!("shaders/wireframe_vert.glsl"),
    //     include_bytes!("shaders/wireframe_geom.glsl"),
    //     include_bytes!("shaders/wireframe_frag.glsl"),
    //     ).unwrap();

    // create pso
    // let pso = factory.create_pipeline_state(
    //     &shader_set, gfx::Primitive::TriangleList, gfx::state::Rasterizer::new_fill(),
    //     pipe::new()
    //     ).unwrap();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/basic_vert.glsl"),
        include_bytes!("shaders/basic_frag.glsl"),
        pipe::new()
        ).unwrap();

    // event loop
    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        }, ..
                    } => running = false,
                    glutin::WindowEvent::Resized(w, h) => {
                        debug!("resizing window: {}x{}", w, h);
                        window.resize(w, h);
                    },
                    glutin::WindowEvent::MouseMoved{ position, .. } => {
                        debug!("mouse moved: ({},{})", position.0, position.1)
                    },
                    _ => {}
                }
            }
        });

        encoder.clear(&data.out, BLACK);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

fn gfx_load_texture<F, R>(factory: &mut F) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
where F: gfx::Factory<R>,
      R: gfx::Resources
{
    use gfx::format::Rgba8;
    let img = image::open("resources/bemo.jpg").unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<ColorFormat>(kind, &[&img]).unwrap();
    view
}
