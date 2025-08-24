

use wgpu::{util::StagingBelt, PipelineCompilationOptions};
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text}; 
use winit::window::Window;

pub struct State<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    pub triangle_pipeline: wgpu::RenderPipeline,
    pub line_pipeline: wgpu::RenderPipeline,

    pub draw_data: Vec<crate::vertex::VertexDraw>,

    pub glyph_brush: GlyphBrush<()>, 
    pub staging_belt: StagingBelt, 
}

impl<'a> State<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find adapter");

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            ..Default::default()
        }).await.unwrap();

        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shape Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into())
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let triangle_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_main"), compilation_options: PipelineCompilationOptions::default(), buffers: &[crate::vertex::Vertex::desc()] },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(format.into())],
                compilation_options: PipelineCompilationOptions::default()
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_main"), compilation_options: PipelineCompilationOptions::default(), buffers: &[crate::vertex::Vertex::desc()] },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(format.into())],
                compilation_options: PipelineCompilationOptions::default()
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("Roboto-Medium.ttf")).unwrap();

        let glyph_brush = GlyphBrushBuilder::using_font(font)
            .build(&device, format);

        let staging_belt = StagingBelt::new(1024);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            triangle_pipeline,
            line_pipeline,
            draw_data: vec![],
            glyph_brush,
            staging_belt
        }
    }

     pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 100.0, g: 50.0, b: 0.0, a: 1.0 }), store: wgpu::StoreOp::Store },
                    depth_slice: None
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            if !self.draw_data.is_empty() {
                for draw in &self.draw_data {
                    match draw.topology {
                        wgpu::PrimitiveTopology::LineList => draw.draw(&mut _rpass, &self.line_pipeline),
                        _ => draw.draw(&mut _rpass, &self.triangle_pipeline),
                    }
                }
            }
        };

        self.glyph_brush.draw_queued(&self.device, &mut self.staging_belt, &mut encoder, &view, self.config.width, self.config.height).unwrap();
        self.staging_belt.finish();

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        self.staging_belt.recall();
        Ok(())
    }

    pub fn submit_draw(&mut self, vertices: &[crate::vertex::Vertex], topology: wgpu::PrimitiveTopology) {
        let draw = match topology {
            wgpu::PrimitiveTopology::LineList => crate::vertex::VertexDraw::new_lines(&self.device, vertices),
            _ => crate::vertex::VertexDraw::new_tris(&self.device, vertices),
        };
        self.draw_data.push(draw);
    }

    pub fn queue_text(&mut self, text: &str) {
        self.glyph_brush.queue(Section {
            screen_position: (30.0, 50.0),
            text: vec![
                Text::new(text)
                    .with_scale(40.0)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Section::default()
        });
    }
}