use std::num::NonZero;

use wgpu::util::DeviceExt;

use bytemuck::NoUninit;



pub fn create_dumb_bind_group_layout_entry(binding: u32, visibility: wgpu::ShaderStages, size: u64) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: Some(NonZero::new(size).unwrap()),
        },
        count: None,
    }
}

pub fn create_dumb_bindgroup(device: &wgpu::Device, label: Option<&str>, layout: &wgpu::BindGroupLayout, buffer: &wgpu::Buffer) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer,
                    offset: 0,
                    size: None,
                })
            }
        ]
    })
}

pub fn create_dumb_texture(
    device: &wgpu::Device,
    label: Option<&str>,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    sample_count: u32,
) -> (wgpu::Texture, wgpu::TextureView) {
    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::wgt::TextureDescriptor {
        label,
        size,
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let texture_view = texture.create_view(&wgpu::wgt::TextureViewDescriptor::default());

    (texture, texture_view)
}

pub fn create_dumb_buffer_init<A: NoUninit>(device: &wgpu::Device, label: Option<&str>, usage: wgpu::BufferUsages, data: &[A]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label,
        usage,
        contents: bytemuck::cast_slice(data),
    })
}