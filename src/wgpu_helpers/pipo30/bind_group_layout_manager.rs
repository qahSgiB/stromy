use crate::wgpu_helpers;



// TODO: names
const BYTES_F: u64 = 4;
const BYTES_F3: u64 = BYTES_F * 3;
const BYTES_F4: u64 = BYTES_F * 4;
const BYTES_M4F32: u64 = BYTES_F * 16;
const BYTES_U32: u64 = 4;



pub struct BindGroupLayoutManager {
    bind_group_layout_m4f32: wgpu::BindGroupLayout, // view, projection
    bind_group_layout_color: wgpu::BindGroupLayout, // color
    bind_group_layout_qcyl: wgpu::BindGroupLayout, // qcyl: color, resolution, smooth_normals
}

impl BindGroupLayoutManager {
    pub fn new(device: &wgpu::Device) -> BindGroupLayoutManager {
        let bind_group_layout_m4f32 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu_helpers::create_dumb_bind_group_layout_entry(0, wgpu::ShaderStages::VERTEX, BYTES_M4F32)]
        });

        let bind_group_layout_color = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu_helpers::create_dumb_bind_group_layout_entry(0, wgpu::ShaderStages::FRAGMENT, BYTES_F4)],
        });

        let bind_group_layout_qcyl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                // color
                wgpu_helpers::create_dumb_bind_group_layout_entry(0, wgpu::ShaderStages::FRAGMENT, BYTES_F4),
                // resolution
                wgpu_helpers::create_dumb_bind_group_layout_entry(1, wgpu::ShaderStages::VERTEX, BYTES_U32),
                // smooth_normals
                wgpu_helpers::create_dumb_bind_group_layout_entry(2, wgpu::ShaderStages::VERTEX, BYTES_U32),
            ],
        });

        BindGroupLayoutManager {
            bind_group_layout_m4f32,
            bind_group_layout_color,
            bind_group_layout_qcyl,
        }
    }

    pub fn get_view(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout_m4f32
    }

    pub fn get_projection(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout_m4f32
    }

    pub fn get_color(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout_color
    }

    pub fn get_qcyl(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout_qcyl
    }

    pub fn create_view_bind_group(&self, device: &wgpu::Device, label: Option<&str>, view: &wgpu::Buffer) -> wgpu::BindGroup {
        wgpu_helpers::create_dumb_bindgroup(device, label, self.get_view(), view)
    }

    pub fn create_projection_bind_group(&self, device: &wgpu::Device, label: Option<&str>, projection: &wgpu::Buffer) -> wgpu::BindGroup {
        wgpu_helpers::create_dumb_bindgroup(device, label, self.get_projection(), projection)
    }

    pub fn create_color_bind_group(&self, device: &wgpu::Device, label: Option<&str>, color: &wgpu::Buffer) -> wgpu::BindGroup {
        wgpu_helpers::create_dumb_bindgroup(device, label, self.get_color(), color)
    }

    pub fn create_qcyl_bind_group(&self, device: &wgpu::Device, label: Option<&str>, color: &wgpu::Buffer, resolution: &wgpu::Buffer, smooth_normals: &wgpu::Buffer) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: self.get_qcyl(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: color,
                        offset: 0,
                        size: None,
                    })
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: resolution,
                        offset: 0,
                        size: None,
                    })
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: smooth_normals,
                        offset: 0,
                        size: None,
                    })
                },
            ]
        })
    }
}