
use std::ops::RangeInclusive;

use egui::Ui;
use glium::glutin::event_loop::ControlFlow;
use glium::{
    VertexBuffer, IndexBuffer, index::PrimitiveType, Display, 
    Surface, Program, uniforms::{EmptyUniforms, Uniforms}, draw_parameters::DrawParameters,};
use glium::glutin::*;
use glam::*;

use crate::loading::*;
use crate::draw::*;

include!("../assets/shaders.rs");
include!("../assets/cube.rs");


pub struct GameObject {
    pub name: &'static str,
    pub transform: Transform,
    pub mesh: &'static Mesh,
}

pub struct GameState {
    pub game_objects: Vec<GameObject>,
    pub t: f32,
    pub is_pixelated: bool,
}
impl GameState {
    pub fn init(value: Self) {
        unsafe {
            game_state = Some(value);
        }
    }

    pub fn get() -> &'static mut Self {
        unsafe { game_state.as_mut().unwrap() }
    }
}

static mut game_state: Option<GameState> = None;

pub fn init() {

    Assets::get().meshes.push(Mesh {
        pos: vec![
            Vec3 {x: -1.0,  y: -1.0, z: 0.0},
            Vec3 {x:  1.0,  y: -1.0, z: 0.0},
            Vec3 {x:  1.0,  y: 1.0 , z: 0.0}, 
            Vec3 {x:  -1.0, y: 1.0 , z: 0.0},
        ],
        nor: Vec::new(),
        ind: vec![0_u32, 1, 2, 0, 2, 3]
    });
    Assets::get().meshes.push(cube_mesh());

    GameState::init(GameState {
        game_objects: vec![
            GameObject {
                name: "test",
                transform: Transform::id(),
                mesh: &Assets::get().meshes[0],
            },
            GameObject {
                name: "cube",
                transform: Transform::id(),
                mesh: &Assets::get().meshes[1],
            },
        ],
        t: 0.,
        is_pixelated: false,
    });
    GameState::get().game_objects[0].transform.scale = Vec3 {x: 0.0, y: 0.1, z: 0.1};
    GameState::get().game_objects[1].transform.scale = Vec3 {x: 0.1, y: 0.1, z: 0.1};

    let display = &RenderState::get().display;


    // #[derive(Copy, Clone)]
    // struct Vertex {
    //     position: [f32; 2],
    //     uv: [f32; 2],
    // }

    // implement_vertex!(Vertex, position, uv);

    // let vertex1 = Vertex { position: [-0.5, -0.5], uv: [0.0, 0.0] };
    // let vertex2 = Vertex { position: [ 0.0,  0.5], uv: [1.0, 0.0] };
    // let vertex3 = Vertex { position: [ 0.5, -0.25], uv: [0.0, 1.0] };
    // let shape = vec![vertex1, vertex2, vertex3];

    // let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    // let shape_wf_ibo = glium::IndexBuffer::new(display, glium::index::PrimitiveType::LineLoop, &[0_u32, 1, 2]).unwrap();

    // let vertex1 = Vertex { position: [-1.0, -1.0], uv: [0.0, 0.0] };
    // let vertex2 = Vertex { position: [ 1.0,  -1.0], uv: [1.0, 0.0] };
    // let vertex3 = Vertex { position: [ 1.0, 1.0], uv: [1.0, 1.0] };
    // let vertex4 = Vertex { position: [ -1.0, 1.0], uv: [0.0, 1.0] };
    // let quad = vec![vertex1, vertex2, vertex3, vertex4];
    // let quad_indices = [0_u32, 1, 2, 0, 2, 3];

    // let quad_shape_vbo = glium::VertexBuffer::new(display, &quad).unwrap();
    // let quad_ibo = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &quad_indices).unwrap();


    let quad_program = glium::Program::from_source(display, QUAD_VSH_SRC, QUAD_FSH_SRC, None).unwrap();
    Assets::get().shaders.push(quad_program);

    let triangle_program = glium::Program::from_source(display, TRIANGLE_VSH_SRC, TRIANGLE_FSH_SRC, None).unwrap();
    Assets::get().shaders.push(triangle_program);

    let triangle_wf_program = glium::Program::from_source(display, WIREFRAME_VSH_SRC, WIREFRAME_FSH_SRC, None).unwrap();
    Assets::get().shaders.push(triangle_wf_program);

    let diffuse_program = glium::Program::from_source(display, DIFFUSE_VSH_SRC, DIFFUSE_FSH_SRC, None).unwrap();
    Assets::get().shaders.push(diffuse_program);
}

pub fn update(dt: f32) {
    let gs = GameState::get();

    let cube_rot = &mut gs.game_objects[1].transform.rotation;
    *cube_rot = Quat::from_axis_angle(Vec3::Y, dt) * (*cube_rot);
}


pub fn render(dt: f32, control_flow: &mut ControlFlow) {
    let rs = RenderState::get();
    let gs = GameState::get();
    let egui_glium = &mut rs.egui_glium;
    let display = &rs.display;

    let mut target = display.draw();


    // let color = egui::Rgba::from_rgb(0.1, 0.3, 0.2);
    // target.clear_color(color[0], color[1], color[2], color[3]);

    // draw things behind egui here
    // target.clear_color(70./256., 102./256., 101./256., 1.0);
    target.clear_color_and_depth((70./256., 102./256., 101./256., 1.0), 1.0);
    // use glium::uniforms::*;
    // let behavior = glium::uniforms::SamplerBehavior {
    //     minify_filter: MinifySamplerFilter::Nearest,
    //     magnify_filter: MagnifySamplerFilter::Nearest,
    //     ..Default::default()
    // };
    // let uniforms = uniform! {
    //     t: *t,
    //     tex: glium::uniforms::Sampler(&texture, behavior),
    // };

    // let mut fb = glium::framebuffer::SimpleFrameBuffer::new(display, &rs.pixel_texture).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    if !gs.is_pixelated {
        render3d(&mut target, gs.game_objects.as_slice(), &mut rs.render3d_pixelation_data.render3d_data, &ShaderData {
            program: &Assets::get().shaders[3], 
            uniforms: EmptyUniforms, 
            draw_parameters: params.clone(),
        });
        // rs.render_buffer.render(&mut target, &Assets::get().shaders[3], 
        // &EmptyUniforms, &params);
    } else {
        render3d_pixelation(&mut target, gs.game_objects.as_slice(), &mut rs.render3d_pixelation_data, &ShaderData {
            program: &Assets::get().shaders[3], 
            uniforms: EmptyUniforms, 
            draw_parameters: params.clone(),
        });
        // rs.render_buffer.render(&mut fb, &Assets::get().shaders[3], 
        // &EmptyUniforms, &params);


    } 



    let repaint_after = egui_glium.run(&display, |egui_ctx| {
        egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
            ui.heading("Help me!");
            ui.label(format!("{}", 1./dt));
            if ui.button("Quit").clicked() {
                *control_flow = ControlFlow::Exit;
            }
            ui.add(egui::Checkbox::new(&mut gs.is_pixelated, "Pixel?"));
            ui.add(egui::Slider::new(&mut gs.t, 0.0..=1.0));

            ui.add(egui::Label::new("Game Objects: "));
            for go in gs.game_objects.iter_mut() {
                egui::CollapsingHeader::new(go.name)
                    .show(ui, |ui| {
                        gui_transform(ui, &mut go.transform, -1.0..=1.0);
                    });
            }
        });
    });

    egui_glium.paint(&display, &mut target);

    // draw things on top of egui here

    target.finish().unwrap();

}

// pub fn _render(dt: f32, control_flow: &ControlFlow) {
//     let egui_glium = RenderState::get().egui_glium;
//     let display = RenderState::get().display;

//     let mut quit = false;
//     let t = &mut GameState::get().t;

//     *t += dt;
//     let repaint_after = egui_glium.run(&display, |egui_ctx| {
//         egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
//             ui.heading("Hello World!");
//             ui.label(format!("{}", 1./dt));
//             if ui.button("Quit").clicked() {
//                 *control_flow = ControlFlow::Exit;
//             }
//             ui.add(egui::Slider::new(t, 0.0..=1.0));
//         });
//     });


//     {
//         use glium::Surface as _;

//         let mut fb = glium::framebuffer::SimpleFrameBuffer::new(display, &texture).unwrap();

//         fb.clear_color(0., 0., 0., 0.);


//         fb.draw(&vertex_buffer, &indices, &triangle_program, &uniform!{t: *t},
//                     &Default::default()).unwrap();
//         fb.draw(&vertex_buffer, &shape_wf_ibo, &triangle_wf_program, &uniform!{t: *t},
//                     &Default::default()).unwrap();
//         // fb.finish().unwrap();






//         let mut target = display.draw();


//         // let color = egui::Rgba::from_rgb(0.1, 0.3, 0.2);
//         // target.clear_color(color[0], color[1], color[2], color[3]);

//         // draw things behind egui here
//         target.clear_color(70./256., 102./256., 101./256., 1.0);

//         // t += 100. * dt;

//         use glium::uniforms::*;
//         let behavior = glium::uniforms::SamplerBehavior {
//             minify_filter: MinifySamplerFilter::Nearest,
//             magnify_filter: MagnifySamplerFilter::Nearest,
//             ..Default::default()
//         };
//         let uniforms = uniform! {
//             t: *t,
//             tex: glium::uniforms::Sampler(&texture, behavior),
//         };

//         target.draw(&quad_shape_vbo, &quad_ibo, &guad_program, 
//                     &uniforms,
//                     &params).unwrap();


//         egui_glium.paint(&display, &mut target);

//         // draw things on top of egui here

//         target.finish().unwrap();
//     }
// }

fn gui_vec3(ui: &mut Ui, v: &mut Vec3, range: RangeInclusive<f32>)  {
    ui.add(egui::Slider::new(&mut v.x, range.clone()));
    ui.add(egui::Slider::new(&mut v.y, range.clone()));
    ui.add(egui::Slider::new(&mut v.z, range.clone()));
}
fn gui_vec4(ui: &mut Ui, v: &mut Vec4, range: RangeInclusive<f32>)  {
    ui.add(egui::Slider::new(&mut v.x, range.clone()));
    ui.add(egui::Slider::new(&mut v.y, range.clone()));
    ui.add(egui::Slider::new(&mut v.z, range.clone()));
    ui.add(egui::Slider::new(&mut v.w, range.clone()));
}
fn gui_quat(ui: &mut Ui, q: &mut Quat, range: RangeInclusive<f32>)  {
    ui.add(egui::Slider::new(&mut q.x, range.clone()));
    ui.add(egui::Slider::new(&mut q.y, range.clone()));
    ui.add(egui::Slider::new(&mut q.z, range.clone()));
    ui.add(egui::Slider::new(&mut q.w, range.clone()));
}

fn gui_transform(ui: &mut Ui, t: &mut Transform, range: RangeInclusive<f32>) {
    ui.add(egui::Label::new("position"));
    gui_vec3(ui, &mut t.position, range.clone());
    ui.add(egui::Label::new("rotation"));
    gui_quat(ui, &mut t.rotation, range.clone());
    ui.add(egui::Label::new("scale"));
    gui_vec3(ui, &mut t.scale, range.clone());
}
