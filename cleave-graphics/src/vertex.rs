use glam::Vec2;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: Vec2,
    pub tex_coords: Vec2,
}

pub const QUAD: [Vertex; 4] = [
    Vertex::from_x_y(-1.0, -1.0, 0.0, 1.0), // Bottom left
    Vertex::from_x_y(1.0, -1.0, 1.0, 1.0),  // Bottom right
    Vertex::from_x_y(-1.0, 1.0, 0.0, 0.0),  // Top left
    Vertex::from_x_y(1.0, 1.0, 1.0, 0.0),   // Top right
];
pub const QUAD_INDICES: [u16; 4] = [0, 1, 2, 3];

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    pub const fn new(pos: Vec2, tex_coords: Vec2) -> Self {
        Vertex { pos, tex_coords }
    }

    pub const fn from_x_y(x: f32, y: f32, tx: f32, ty: f32) -> Self {
        Vertex {
            pos: Vec2::new(x, y),
            tex_coords: Vec2::new(tx, ty),
        }
    }

    pub const fn x_y(&self) -> (f32, f32) {
        (self.pos.x, self.pos.y)
    }

    pub const fn xy(self) -> Vec2 {
        self.pos
    }
}
