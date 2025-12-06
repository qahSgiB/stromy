use winit::dpi::PhysicalSize;

use crate::wgpu_helpers;



struct MSAATexture {
    sample_count: u32,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}

pub struct SurfaceManager {
    /* render color texture (multisampled) */
    surface_format: wgpu::TextureFormat,
    msaa_texture: Option<MSAATexture>,

    /* render depth texture */
    depth_format: wgpu::TextureFormat,
    depth_texture: wgpu::Texture,
    depth_texture_view: wgpu::TextureView,
}

impl SurfaceManager {
    pub fn new(device: &wgpu::Device, surface_size: PhysicalSize<u32>, surface_format: wgpu::TextureFormat, msaa_sample_count: u32) -> SurfaceManager {
        let msaa_texture = (msaa_sample_count != 1).then(|| {
            let (color_texture, color_texture_view) = wgpu_helpers::create_dumb_texture(device, None, surface_size.width, surface_size.height, surface_format, msaa_sample_count);

            MSAATexture {
                sample_count: msaa_sample_count,
                texture: color_texture,
                view: color_texture_view,
            }
        });

        let depth_format = wgpu::TextureFormat::Depth32Float;
        let (depth_texture, depth_texture_view) = wgpu_helpers::create_dumb_texture(device, None, surface_size.width, surface_size.height, depth_format, msaa_sample_count);

        SurfaceManager {
            surface_format,
            msaa_texture,
            depth_format,
            depth_texture,
            depth_texture_view,
        }
    }

    pub fn get_sample_count(&self) -> u32 {
        self.msaa_texture.as_ref().map_or(1, |msaa_texture| msaa_texture.sample_count)
    }

    pub fn get_surface_format(&self) -> wgpu::TextureFormat {
        self.surface_format
    }

    pub fn get_depth_format(&self) -> wgpu::TextureFormat {
        self.depth_format
    }

    pub fn resize(&mut self, device: &wgpu::Device, surface_size: PhysicalSize<u32>) {
        let sample_count = match self.msaa_texture.as_mut() {
            Some(msaa_texture) => {
                (msaa_texture.texture, msaa_texture.view) = wgpu_helpers::create_dumb_texture(device, None, surface_size.width, surface_size.height, self.surface_format, msaa_texture.sample_count);

                msaa_texture.sample_count
            },
            None => 1,
        };

        (self.depth_texture, self.depth_texture_view) = wgpu_helpers::create_dumb_texture(device, None, surface_size.width, surface_size.height, wgpu::TextureFormat::Depth32Float, sample_count);
    }

    pub fn begin_render_pass<'encoder>(&self, encoder: &'encoder mut wgpu::CommandEncoder, surface_texture_view: &wgpu::TextureView, clear_color: wgpu::Color) -> wgpu::RenderPass<'encoder> {
        let color_attachment = if let Some(msaa_texture) = self.msaa_texture.as_ref() {
            wgpu::RenderPassColorAttachment {
                view: &msaa_texture.view,
                resolve_target: Some(surface_texture_view),
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: wgpu::StoreOp::Store
                },
                depth_slice: None,
            }
        } else {
            wgpu::RenderPassColorAttachment {
                view: &surface_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: wgpu::StoreOp::Store
                },
                depth_slice: None,
            }
        };
        
        let depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: &self.depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: Some(depth_stencil_attachment),
            ..Default::default()
        })
    }
}