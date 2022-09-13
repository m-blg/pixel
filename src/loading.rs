
use glium::{VertexBuffer, IndexBuffer, index::PrimitiveType, Display, Surface, Program, uniforms::Uniforms, draw_parameters::DrawParameters};
use glam::*;
use crate::draw::*;

#[derive(Default)]
pub struct Assets 
{
    pub meshes: Vec<Mesh>,
    pub shaders: Vec<Program>,
}


impl Assets {
    // const unsafe fn uninit() -> Self {
    //     const_zero::const_zero!(RenderState)
    // }
    
    pub fn init() {
        unsafe {
            assets = Some(Default::default());
        }
    }

    pub fn get() -> &'static mut Self {
        unsafe { assets.as_mut().unwrap() }
    }
}

static mut assets: Option<Assets> = None;

// TODO(mb): mesh loading