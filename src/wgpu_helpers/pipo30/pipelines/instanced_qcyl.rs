use std::ops::Range;

use crate::wgpu_helpers::pipo30::BindGroupLayoutManager;
use crate::wgpu_helpers::pipo30::SurfaceManager;



// TODO: names
const BYTES_U: u64 = 4; // u32
const BYTES_F: u64 = 4;
const BYTES_F3: u64 = BYTES_F * 3;
const BYTES_F4: u64 = BYTES_F * 4;
const BYTES_M4: u64 = BYTES_F * 16;



pub struct InstancedQcylPipeline {
    pipeline: wgpu::RenderPipeline,
}

impl InstancedQcylPipeline {
    pub fn new(device: &wgpu::Device, bind_group_layout_manager: &BindGroupLayoutManager, surface_manager: &SurfaceManager) -> InstancedQcylPipeline {
        /* shader */
        let shader_source = std::fs::read_to_string("assets/shaders/instanced_qcyl.wgsl").unwrap();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        /* pipeline */
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                bind_group_layout_manager.get_projection(),
                bind_group_layout_manager.get_view(),
                bind_group_layout_manager.get_qcyl(),
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
                    // radius
                    wgpu::VertexBufferLayout {
                        array_stride: BYTES_F,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32 }
                        ]
                    },
                    // model
                    wgpu::VertexBufferLayout {
                        array_stride: BYTES_M4,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: BYTES_F4 * 0, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 1, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 2, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 3, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                        ]
                    },
                    // model_it
                    wgpu::VertexBufferLayout {
                        array_stride: BYTES_M4,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: BYTES_F4 * 0, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 1, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 2, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: BYTES_F4 * 3, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
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

        /* render textures */

        InstancedQcylPipeline {
            pipeline,
        }
    }

    pub fn use_pipeline<'a, 'encoder>(&self, render_pass: &'a mut wgpu::RenderPass<'encoder>) -> InstancedQcylRenderPass<'a, 'encoder> {
        render_pass.set_pipeline(&self.pipeline);

        InstancedQcylRenderPass { render_pass }
    }
}


pub struct InstancedQcylRenderPass<'a, 'encoder> {
    // TODO: no idea about the lifetimes, but currently this seems to be working
    render_pass: &'a mut wgpu::RenderPass<'encoder>
}

impl<'a, 'encoder> InstancedQcylRenderPass<'a, 'encoder> {
    pub fn set_projection_bind_group(&mut self, projection: &wgpu::BindGroup) {
        self.render_pass.set_bind_group(0, projection, &[]);
    }

    pub fn set_view_bind_group(&mut self, view: &wgpu::BindGroup) {
        self.render_pass.set_bind_group(1, view, &[]);
    }

    pub fn set_qcyl_bind_group(&mut self, qcyl: &wgpu::BindGroup) {
        self.render_pass.set_bind_group(2, qcyl, &[]);
    }

    pub fn set_radiuses_vertex_buffer(&mut self, radiuses: wgpu::BufferSlice) {
        self.render_pass.set_vertex_buffer(0, radiuses);
    }

    pub fn set_model_vertex_buffer(&mut self, model: wgpu::BufferSlice, model_it: wgpu::BufferSlice) {
        self.render_pass.set_vertex_buffer(1, model);
        self.render_pass.set_vertex_buffer(2, model_it);
    }

    pub fn draw(&mut self, resolution: u32, instances: Range<u32>) {
        self.render_pass.draw(0..(12 * resolution), instances);
    }
}