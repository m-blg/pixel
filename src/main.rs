#![feature(vec_into_raw_parts, thread_local, ptr_internals)]

#[macro_use]
extern crate glium;

use std::fs::create_dir_all;

use glium::glutin;
use glutin::event_loop::ControlFlow;
use glutin::event::Event;

use glam::*;

mod loading;
mod game;
mod draw;

use loading::Assets;
use draw::{RenderState, Render3dData};

fn main() {
    let event_loop = glutin::event_loop::EventLoopBuilder::with_user_event().build();

    let window_size = UVec2 {x: 1000, y: 1000};
    RenderState::init(window_size, &event_loop);
    Assets::init();
    // unsafe{
    //     RenderState::init(RenderState {
    //         window_size: window_size.as_vec2(),
    //         display: create_display(&event_loop, window_size.into()),
    //         egui_glium: egui_glium::EguiGlium::new(&(*draw::render_state).display, &event_loop),

    //         render_buffer: RenderBuffer::new(&(*draw::render_state).display, 100),
    //     });
    // }
    let rs = RenderState::get();

    //
    // let empty_texture
    // let empty_texture = vec![255_u8; (window_size.x * window_size.y * 3) as usize];
    // let raw = glium::texture::RawImage2d::from_raw_rgb(empty_texture, window_size.into());
    // dbg!(raw.format.get_size());
    // dbg!(std::mem::size_of::<u32>());
    // let texture = glium::texture::srgb_texture2d::SrgbTexture2d::new(&display, raw).unwrap();

    // unsafe {*draw::render_state = }
    game::init();
    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let mut t: f32 = 0.;

    let mut prev_frame_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {

        let frame_begin_time = std::time::Instant::now();
        let dt_dur = frame_begin_time - prev_frame_time;
        let dt = dt_dur.as_secs() as f32 + dt_dur.subsec_nanos() as f32 / 1_000_000_000.0;

        *control_flow = ControlFlow::WaitUntil(frame_begin_time + ::std::time::Duration::new(0, 1_000_000_000u32 / 60));
        // *control_flow = ControlFlow::Poll;


        // let mut redraw = || {
        // };

        match event {
            Event::MainEventsCleared => {
                let gl_window = rs.display.gl_window();
                // platform
                //     .prepare_frame(imgui.io_mut(), gl_window.window())
                //     .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => { 
                if dt_dur >= ::std::time::Duration::new(0, 1_000_000_000u32 / 60) {
                    prev_frame_time = frame_begin_time;
                    game::update(dt);
                    game::render(dt, control_flow); 
                }
            }

            Event::WindowEvent { event, .. } => {
                use glutin::event::WindowEvent;
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                rs.egui_glium.on_event(&event);

                rs.display.gl_window().window().request_redraw(); // TODO(emilk): ask egui if the events warrants a repaint instead
            }

            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } 
                    => rs.display.gl_window().window().request_redraw(),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => (),
        }

        // ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    });
}
