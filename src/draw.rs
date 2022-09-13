
use std::marker::PhantomData;
use std::mem::MaybeUninit;

use glium::framebuffer::DepthRenderBuffer;
use glium::glutin::{self, event_loop};
use glium::{VertexBuffer, IndexBuffer, index::PrimitiveType, Display, Surface, Program, uniforms::Uniforms, draw_parameters::DrawParameters, glutin::event_loop::{EventLoop, ControlFlow}};
use glam::*;

use crate::loading::*;

pub struct Mesh {
    pub pos: Vec<Vec3>,
    pub nor: Vec<Vec3>,
    pub ind: Vec<u32>,
}

pub struct MeshRenderData<'a> {
    pub mesh: &'a Mesh,
    pub vps_vbo: VertexBuffer<MeshRenderDataVertexPos>, 
    pub nor_vbo: VertexBuffer<MeshRenderDataVertexNor>, 
    pub ibo: IndexBuffer<u32>,
}

impl<'a> MeshRenderData<'a> {
    pub fn new(display: &Display, mesh: &'a Mesh) -> Self {
        // let data: Vec<MeshRenderDataVertexPos> = unsafe {std::mem::transmute(mesh.vps)};
        let data = unsafe {
            std::slice::from_raw_parts(mesh.pos.as_ptr() as *const MeshRenderDataVertexPos, mesh.pos.len())
        };
        MeshRenderData {
            mesh: mesh,
            vps_vbo: VertexBuffer::new(display, &data).unwrap(),
            nor_vbo: VertexBuffer::new(display, &vec![MeshRenderDataVertexNor{normal: Vec3::ZERO.into()}; mesh.pos.len()]).unwrap(),
            ibo: IndexBuffer::new(display, PrimitiveType::TrianglesList, &mesh.ind).unwrap(),
        }
    }

    pub fn render<S: Surface, U: Uniforms>(self: &Self, surface: &mut S, shader: &Program, uniforms: &U, draw_parameters: &DrawParameters) {
        surface.draw(&self.vps_vbo, &self.ibo, &shader, uniforms,
                        draw_parameters).unwrap();
    }
}


#[derive(Copy, Clone, Debug)]
pub struct MeshRenderDataVertexPos {
    pub position: [f32; 3],
}
implement_vertex!(MeshRenderDataVertexPos, position);
#[derive(Copy, Clone, Debug)]
pub struct MeshRenderDataVertexNor {
    pub normal: [f32; 3],
}
implement_vertex!(MeshRenderDataVertexNor, normal);


pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn id() -> Transform {
        Transform {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn model(self: &Self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

// pub struct GameObject<'a> {
//     pub transform: Transform,
//     pub render_data: &'a MeshRenderData<'a>,
// }

pub struct ShaderData<'a, U: Uniforms> {
    pub program: &'a Program,
    pub uniforms: U,
    pub draw_parameters: DrawParameters<'a>,
}

// pub fn render_plain<S: Surface>(surface: &S) {
//     data.render(surface);
// }

#[derive(Debug)]
pub struct Render3dData {
    pub pos: Vec<MeshRenderDataVertexPos>,
    pub nor: Vec<MeshRenderDataVertexNor>,
    pub ind: Vec<u32>,
    // render_buffer_uv: Vec<MeshRenderDataVertexNor>,

    pub pos_vbo: VertexBuffer<MeshRenderDataVertexPos>, 
    pub nor_vbo: VertexBuffer<MeshRenderDataVertexNor>, 
    pub ibo: IndexBuffer<u32>,
}

impl Render3dData {
    pub fn new(display: &Display, cap: usize) -> Self {
        // let data: Vec<MeshRenderDataVertexPos> = unsafe {std::mem::transmute(mesh.vps)};
        Render3dData {
            pos: Vec::with_capacity(cap),
            nor: Vec::with_capacity(cap),
            ind: Vec::with_capacity(cap),

            pos_vbo: VertexBuffer::empty_dynamic(display, cap).unwrap(),
            nor_vbo: VertexBuffer::empty_dynamic(display, cap).unwrap(),
            ibo: IndexBuffer::empty_dynamic(display, PrimitiveType::TrianglesList, cap).unwrap(),
        }
    }

    pub fn render<S: Surface, U: Uniforms>(&self, surface: &mut S, shader: &Program, uniforms: &U, draw_parameters: &DrawParameters) {
        surface.draw((&self.pos_vbo, &self.nor_vbo), &self.ibo, &shader, uniforms,
                        draw_parameters).unwrap();
    }

    pub fn send(&mut self) {
        if self.pos.len() == 0 { return; }

        if self.pos.len() != self.pos_vbo.len() {
            self.pos_vbo = VertexBuffer::dynamic(&RenderState::get().display, &self.pos).unwrap();
        } else {
            self.pos_vbo.write(&self.pos);
        }

        if self.ind.len() != self.ibo.len() {
            self.ibo = IndexBuffer::dynamic(&RenderState::get().display, PrimitiveType::TrianglesList,  &self.ind).unwrap();
        } else {
            self.ibo.write(&self.ind);
        }
    }
}


pub struct Render3dPixelationData {
    pub render3d_data: Render3dData,
    pub pixel_texture: glium::texture::srgb_texture2d::SrgbTexture2d,
    pub pixel_depth: DepthRenderBuffer,
    pub quad_vbo: VertexBuffer<QuadVertex>,
    pub quad_ibo: IndexBuffer<u32>,
}


// granularity is important for readability
// generics are too constraint

pub fn gl_vbo_update<T>(vbo: &mut VertexBuffer<T>, data: &[T]) 
where T: Copy + glium::Vertex 
{
        if data.len() != vbo.len() {
            *vbo = VertexBuffer::dynamic(&RenderState::get().display, &data).unwrap();
        } else {
            vbo.write(&data);
        }
}

pub fn gl_ibo_update<T>(ibo: &mut IndexBuffer<T>, data: &[T]) 
where T: glium::index::Index 
{
        if data.len() != ibo.len() {
            *ibo = IndexBuffer::dynamic(&RenderState::get().display, PrimitiveType::TrianglesList,  &data).unwrap();
        } else {
            ibo.write(&data);
        }
}
// thread_local! {
//     static transforms: Vec<Transform> = Vec::new();
//     static mesh_ids: Vec<usize> = Vec::new();
//     // static mesh_render_data: Vec<MeshRenderData<'static>> = Default::default();

//     // static render_buffer: RenderBuffer = Default::default();
// }

pub fn render3d<S: Surface, U: Uniforms>(
    target: &mut S,
    game_objects: &[crate::game::GameObject], 
    render_data: &mut Render3dData,
    shader_data: &ShaderData<U>) 
{
    
    // writing batch
    Vec::clear(&mut render_data.pos);
    Vec::clear(&mut render_data.ind);
    for go in game_objects.iter() {
        let mvp = go.transform.model(); // TODO(mb): upgrade to full mvp matrix
        render_data.pos.extend(go.mesh.pos.iter()
            .map(|&p: &Vec3| MeshRenderDataVertexPos {position: (mvp * p.extend(1.)).xyz().into()} ) );
        
        let last_ind = if render_data.ind.is_empty() {-1} else {render_data.ind[render_data.ind.len()-1] as i32};
        render_data.ind.extend(go.mesh.ind.iter()
            .map(|&i: &u32| (last_ind + 1 + i as i32) as u32 ) );
    }
    // println!("{:?}", render_buffer.pos);
    // println!("{:?}", render_buffer.ind);

    Render3dData::send(render_data);

    Render3dData::render(render_data, target, 
        &shader_data.program, 
        &shader_data.uniforms, 
        &shader_data.draw_parameters);

}

pub fn render3d_pixelation<S: Surface, U: Uniforms>(
    target: &mut S,
    game_objects: &[crate::game::GameObject], 
    render_data: &mut Render3dPixelationData,
    shader_data: &ShaderData<U>) 
{
    
    let mut fb = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
        &RenderState::get().display, 
        &render_data.pixel_texture, 
        &render_data.pixel_depth).unwrap();

    fb.clear_color_and_depth((0., 0., 0., 0.), 1.);

    render3d(&mut fb, game_objects, &mut render_data.render3d_data, shader_data);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };
    
    use glium::uniforms::*;
    let behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };
    let uniforms = uniform! {
        tex: glium::uniforms::Sampler(&render_data.pixel_texture, behavior),
    };

    target.draw(&render_data.quad_vbo, &render_data.quad_ibo, &Assets::get().shaders[0], 
                &uniforms,
                &params).unwrap();

}



pub struct RenderState 
{

    pub window_size: Vec2,
    // static mut display: *mut Display = std::ptr::null_mut();
    pub display: Display,
    pub egui_glium: egui_glium::EguiGlium,


    pub render3d_pixelation_data: Render3dPixelationData,

    // _marker: std::marker::PhantomData<RenderBuffer>
}

impl RenderState {
    // const unsafe fn uninit() -> Self {
    //     const_zero::const_zero!(RenderState)
    // }
    
    // pub fn init(value: Self) {
    //     unsafe {
    //         render_state = std::alloc::alloc(std::alloc::Layout::new::<Self>()) as *mut Self;
    //         *render_state = value;
    //     }
    // }

    pub fn init(window_size: UVec2, event_loop: &EventLoop<()>) {
        unsafe {
            let display = create_display(&event_loop, window_size.into());
            let egui_glium = egui_glium::EguiGlium::new(&display, &event_loop);
            let render_buffer = Render3dData::new(&display, 100);

            let pixel_texture_size = window_size / 10;
            let pixel_texture = glium::texture::srgb_texture2d::SrgbTexture2d::empty(&display, pixel_texture_size.x, pixel_texture_size.y).unwrap();
            let pixel_depth = glium::framebuffer::DepthRenderBuffer::new(&display, 
                glium::texture::DepthFormat::I24, pixel_texture_size.x, pixel_texture_size.y).unwrap();

            let vertex1 = QuadVertex { position: [-1.0, -1.0], uv: [0.0, 0.0] };
            let vertex2 = QuadVertex { position: [ 1.0,  -1.0], uv: [1.0, 0.0] };
            let vertex3 = QuadVertex { position: [ 1.0, 1.0], uv: [1.0, 1.0] };
            let vertex4 = QuadVertex { position: [ -1.0, 1.0], uv: [0.0, 1.0] };
            let quad = vec![vertex1, vertex2, vertex3, vertex4];
            let quad_indices = [0_u32, 1, 2, 0, 2, 3];

            let quad_vbo = glium::VertexBuffer::new(&display, &quad).unwrap();
            let quad_ibo = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &quad_indices).unwrap();

            // render_state.write(val)._marker = PhantomData;

            render_state = Some(RenderState {
                window_size: window_size.as_vec2(),
                display,
                egui_glium,
                render3d_pixelation_data: Render3dPixelationData {
                    render3d_data: render_buffer,
                    pixel_texture,
                    pixel_depth,
                    quad_vbo,
                    quad_ibo,
                }
            });
        }
    }

    // pub fn init(window_size: UVec2, event_loop: &EventLoop<()>) {
    //     unsafe {

    //         render_state.write(val)._marker = PhantomData;
    //         render_state = std::ptr::Unique::new(std::alloc::alloc(std::alloc::Layout::new::<Self>()) as *mut Self).unwrap();

    //         let display = create_display(&event_loop, window_size.into());
    //         render_state.as_mut().window_size = window_size.as_vec2();
    //         render_state.as_mut().egui_glium = egui_glium::EguiGlium::new(&render_state.as_mut().display, &event_loop);
    //         render_state.as_mut().render_buffer = RenderBuffer::new(&render_state.as_mut().display, 100);
    //         render_state.as_mut().display = display;

    //     }
    // }

    pub fn get() -> &'static mut Self {
        unsafe { render_state.as_mut().unwrap() }
    }
}

pub static mut render_state: Option<RenderState> = None;
// pub static mut render_state: std::mem::MaybeUninit<RenderState> = MaybeUninit::<RenderState>::uninit();
// pub static mut render_state: std::ptr::Unique<RenderState> = unsafe {
//     std::ptr::Unique::new_unchecked(std::ptr::NonNull::dangling().as_ptr())  
// };

fn create_display(event_loop: &glutin::event_loop::EventLoop<()>, window_size: (u32, u32)) -> glium::Display {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: window_size.0,
            height: window_size.1,
        })
        .with_title("egui_glium example");

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}

#[derive(Copy, Clone)]
pub struct QuadVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

implement_vertex!(QuadVertex, position, uv);
