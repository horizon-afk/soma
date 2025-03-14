use image::DynamicImage;
use wgpu::{
    util::DeviceExt, BindGroupDescriptor, BindGroupLayoutDescriptor, Device, PipelineLayout,
    PipelineLayoutDescriptor, PrimitiveTopology, RenderPipeline, TextureFormat,
};

use crate::texture::{self, TextureBundle};

pub struct GraphicsBundle<U> {
    pipeline: wgpu::RenderPipeline,
    // texture: texture::Texture,
    // texture_bind_group: wgpu::BindGroup,
    texture_bundle: TextureBundle,
    uniform_bind_group: wgpu::BindGroup,
    pub uniforms: U,
    uniform_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl<U> GraphicsBundle<U>
where
    U: Default + bytemuck::Pod + bytemuck::Zeroable + Copy,
{
    pub fn new(
        img: DynamicImage,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        topology: PrimitiveTopology,
        format: TextureFormat,
    ) -> Self {
        let texture = texture::RenderTexture::from_image(device, queue, &img, None)
            .expect("Could not load texture");
        let uniforms = U::default();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            layout: &uniform_bind_group_layout,
            label: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&crate::vertex::QUAD),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&crate::vertex::QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let texture_bundle = TextureBundle::new(texture, device);
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &texture_bundle.bind_group_layout,
                &uniform_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = make_pipeline(device, format, &layout, topology);

        Self {
            pipeline,
            // texture_bind_group: bind_group,
            texture_bundle,
            uniform_bind_group,
            // texture,
            uniforms,
            uniform_buffer,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn update_buffer(&self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    pub fn with_uniforms(self, uniforms: U) -> Self {
        Self { uniforms, ..self }
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.texture_bundle.bind_group, &[]);
        pass.set_bind_group(1, &self.uniform_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // pass.set_vertex_buffer(1, self.uniform_buffer.slice(..));
        // pass.draw(0..4, 0..1);
        pass.draw_indexed(0..4, 0, 0..1);
    }
}

fn make_pipeline(
    device: &Device,
    format: TextureFormat,
    // config: &SurfaceConfiguration,
    layout: &PipelineLayout,
    topology: PrimitiveTopology,
) -> RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/gui.wgsl").into()),
    });
    let vertex = wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        buffers: &[
            crate::vertex::Vertex::desc(),
            // crate::instance::InstanceRaw::desc(),
        ],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
    };
    let fragment = wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        targets: &[Some(wgpu::ColorTargetState {
            format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(layout),
        vertex,
        fragment: Some(fragment),
        primitive: wgpu::PrimitiveState {
            topology,
            strip_index_format: topology.is_strip().then_some(wgpu::IndexFormat::Uint16),
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
            unclipped_depth: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
