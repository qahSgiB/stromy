use std::ops::Range;

use crate::wgpu_helpers::pipo30::SurfaceManager;
use crate::wgpu_helpers::pipo30::BindGroupLayoutManager;



// TODO: names
const BYTES_F: u64 = 4;
const BYTES_F3: u64 = BYTES_F * 3;
const BYTES_F4: u64 = BYTES_F * 4;
const BYTES_M4: u64 = BYTES_F * 16;



pub struct Instanced3dPipeline {
    pipeline: wgpu::RenderPipeline,
}

impl Instanced3dPipeline {
    pub fn new(device: &wgpu::Device, bind_group_layout_manager: &BindGroupLayoutManager, surface_manager: &SurfaceManager) -> Instanced3dPipeline {
        /* shader */
        let shader_source = std::fs::read_to_string("assets/shaders/instanced_3d.wgsl").unwrap();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        /* pipeline */
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                bind_group_layout_manager.get_view(),
                bind_group_layout_manager.get_projection(),
                bind_group_layout_manager.get_color(),
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    // vertex position
                    wgpu::VertexBufferLayout {
                        array_stride: BYTES_F3,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 }
                        ]
                    },
                    // vertex normal
                    wgpu::VertexBufferLayout {
                        array_stride: BYTES_F3,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 1, format: wgpu::VertexFormat::Float32x3 }
                        ]
                    },
                    // model
                    wgpu::VertexBufferLayout {
                        array_stride: BYTES_M4,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: BYTES_F4 * 0, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 1, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 2, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 3, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                        ]
                    },
                    // model_it
                    wgpu::VertexBufferLayout {
                        array_stride: BYTES_M4,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: BYTES_F4 * 0, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 1, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 2, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 3, shader_location: 9, format: wgpu::VertexFormat::Float32x4 },
                        ]
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format: surface_manager.get_surface_format(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })
                ],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: surface_manager.get_depth_format(),
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: surface_manager.get_sample_count(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Instanced3dPipeline {
            pipeline,
        }
    }

    pub fn use_pipeline<'a, 'encoder>(&self, render_pass: &'a mut wgpu::RenderPass<'encoder>) -> Instanced3dRenderPass<'a, 'encoder> {
        render_pass.set_pipeline(&self.pipeline);

        Instanced3dRenderPass { render_pass }
    }
}


pub struct Instanced3dRenderPass<'a, 'encoder> {
    render_pass: &'a mut wgpu::RenderPass<'encoder>
}

impl<'a, 'encoder> Instanced3dRenderPass<'a, 'encoder> {
    pub fn set_projection_bind_group(&mut self, projection: &wgpu::BindGroup) {
        self.render_pass.set_bind_group(0, projection, &[]);
    }

    pub fn set_view_bind_group(&mut self, view: &wgpu::BindGroup) {
        self.render_pass.set_bind_group(1, view, &[]);
    }

    pub fn set_color_bind_group(&mut self, color: &wgpu::BindGroup) {
        self.render_pass.set_bind_group(2, color, &[]);
    }

    pub fn set_object_vertex_buffer(&mut self, vertices: wgpu::BufferSlice, normals: wgpu::BufferSlice) {
        self.render_pass.set_vertex_buffer(0, vertices);
        self.render_pass.set_vertex_buffer(1, normals);
    }

    pub fn set_model_vertex_buffer(&mut self, model: wgpu::BufferSlice, model_it: wgpu::BufferSlice) {
        self.render_pass.set_vertex_buffer(2, model);
        self.render_pass.set_vertex_buffer(3, model_it);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass.draw(vertices, instances);
    }
}