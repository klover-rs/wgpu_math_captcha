use std::sync::mpsc::Receiver;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId, WindowAttributes},
};

use wgpu::util::DeviceExt;
use crate::{shape::{clear_offsets, Shape}, wgpu::State};

pub struct App<'a> {
    window: Option<&'a Window>,
    state: Option<State<'a>>,
    puzzle: Vec<(Shape, i32)>,
    rx: Option<Receiver<String>>,
    solved: bool,
}

impl<'a> ApplicationHandler<()> for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window: &'static Window = Box::leak(Box::new(
                event_loop.create_window(
                    WindowAttributes::default().with_title("Captcha Wgpu")
                ).unwrap()
            ));

            // Init WGPU with &'static Window
            let state = pollster::block_on(State::new(window));

            self.window = Some(window);
            self.state = Some(state);
        }
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ) {
        if let (Some(window), Some(state)) = (&self.window, &mut self.state) {
            if window.id() == window_id {
                match event {
                    WindowEvent::Focused(is_focused) => {
                        
                    }
                    WindowEvent::CloseRequested => event_loop.exit(),
                    WindowEvent::Resized(new_size) => state.resize(new_size.cast()),
                    WindowEvent::RedrawRequested => {
clear_offsets();

    if let Ok(msg) = self.rx.as_ref().unwrap().try_recv() {
        if msg == "SOLVED" {
            println!("🎉 Window got notified: captcha solved!");
            self.solved = true;
            // trigger another redraw so text appears
            window.request_redraw();
        }
    }

    if !self.solved {
        let mut tri_vertices = Vec::new();
        let mut line_vertices = Vec::new();

        for (shape, count) in &self.puzzle {
            let verts = shape.to_vertices(*count);
            match shape {
                Shape::Line => line_vertices.extend(verts),
                _ => tri_vertices.extend(verts),
            }
        }

        state.draw_data.clear();

        if !tri_vertices.is_empty() {
            state.submit_draw(&tri_vertices, wgpu::PrimitiveTopology::TriangleList);
        }
        if !line_vertices.is_empty() {
            state.submit_draw(&line_vertices, wgpu::PrimitiveTopology::LineList);
        }
    } else {
        // once solved, clear shapes
        state.draw_data.clear();
        state.queue_text("✅ Captcha Solved");
    }


    // 🔥 only one render call here
    state.render().unwrap();
                            
                           

                        

                        
                        
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn run(puzzle: Vec<(Shape, i32)>, rx: Receiver<String>) {
    let event_loop = EventLoop::new().unwrap();
    let mut app = crate::window::App {
        window: None,
        state: None,
        puzzle,
        rx: Some(rx),
        solved: false
    };
    event_loop.run_app(&mut app).unwrap();
}