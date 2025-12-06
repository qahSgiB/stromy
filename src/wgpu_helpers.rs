use std::ops::BitAnd;


pub use create_dumb::*;



pub mod pipo30;
pub mod create_dumb;



pub fn print_adapter_info(adapter: &wgpu::Adapter) {
    let adapter_info = adapter.get_info();
    let adapter_type_str = match adapter_info.device_type {
        wgpu::DeviceType::Cpu => "CPU",
        wgpu::DeviceType::DiscreteGpu => "Discrete GPU",
        wgpu::DeviceType::IntegratedGpu => "Integrated GPU",
        wgpu::DeviceType::VirtualGpu => "Virtual GPU",
        wgpu::DeviceType::Other => "Other type",
    };
    let adapter_backend_str = match adapter_info.backend {
        wgpu::Backend::Noop => "Noop",
        wgpu::Backend::Vulkan => "Vulkan",
        wgpu::Backend::Metal => "Metal",
        wgpu::Backend::Dx12 => "Direct3D 12",
        wgpu::Backend::Gl => "OpenGL",
        wgpu::Backend::BrowserWebGpu => "Browser WebGPU",
    };

    println!("");
    println!("Using adapter {} ({})", adapter_info.name, adapter_type_str);
    println!("    driver: {} ({})", adapter_info.driver, adapter_info.driver_info);
    println!("    backend: {}", adapter_backend_str);
    println!("");
}

/// Returns all msaa sample counts available on specified `adapter` for specified list of `texture_formats`.
/// Returned list is in ascending order.
/// Sample count of 1 is not included in the list.
pub fn get_avaiable_msaa_sample_counts(adapter: &wgpu::Adapter, texture_formats: impl Iterator<Item = wgpu::TextureFormat>) -> Vec<u32> {
    let all_texture_formats_flags = texture_formats
        .map(|format| { adapter.get_texture_format_features(format).flags })
        .reduce(BitAnd::bitand)
        .unwrap();

    
    let msaa_counts = [
        (wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X2, 2),
        (wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X4, 4),
        (wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X8, 8),
        (wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X16, 16),
    ];

    msaa_counts
        .iter()
        .filter_map(|&(msaa_feature, msaa_count)| all_texture_formats_flags.contains(msaa_feature).then_some(msaa_count))
        .collect()
}