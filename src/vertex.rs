use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use std::borrow::Cow;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::vertex_attr_array![0 => Float32x2][0],
                wgpu::vertex_attr_array![1 => Float32x3][0]
            ]
            
        }
    }
}

pub struct VertexDraw {
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    pub topology: wgpu::PrimitiveTopology,
}

impl VertexDraw {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], topology: wgpu::PrimitiveTopology)-> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertex_buffer,
            num_vertices: vertices.len() as u32,
            topology
        }
    }

    pub fn new_lines(device: &wgpu::Device, vertices: &[Vertex]) -> Self {
        Self::new(device, vertices, wgpu::PrimitiveTopology::LineList)
    }

    pub fn new_tris(device: &wgpu::Device, vertices: &[Vertex]) -> Self {
        Self::new(device, vertices, wgpu::PrimitiveTopology::TriangleList)
    }

    pub fn draw<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, pipeline: &wgpu::RenderPipeline) {
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_pipeline(pipeline);
        rpass.draw(0..self.num_vertices, 0..1);
    }

}