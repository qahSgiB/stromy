use std::{sync::Arc, time::Instant};

use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{ElementState, KeyEvent, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Fullscreen, Window, WindowId}};

use crate::game::*;



pub mod controls;
pub mod game;
pub mod lsystem;
pub mod models;
pub mod rolling_average;
pub mod wgpu_helpers;



struct AppWgpu {
    window: Arc<Window>,

    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,

    surface_config: wgpu::SurfaceConfiguration,

    start_time: Instant,
    last_time: Instant,
    
    game: Game,

    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,

    /* timing stats */
    last_surface_texture_wait: u128,
}

impl AppWgpu {
    async fn new(window: Window) -> AppWgpu {
        let window = Arc::new(window);

        let size = window.inner_size();

        /* init - instance, surface, adapter */
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        // let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        //     backends: wgpu::Backends::DX12,
        //     ..wgpu::InstanceDescriptor::default()
        // });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        wgpu_helpers::print_adapter_info(&adapter);

        /* msaa feature */
        let feature_tasff = adapter.features().features_wgpu.contains(wgpu::FeaturesWGPU::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES);
        println!("Feature \"TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES\" is {}", if feature_tasff { "AVAILABLE" } else { "NOT AVAILABLE" });

        let mut required_features = wgpu::Features::empty();
        if feature_tasff {
            required_features.features_wgpu |= wgpu::FeaturesWGPU::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES;
        }

        /* init - device, queue */
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features,
            required_limits: wgpu::Limits::defaults(),
            ..Default::default()
        }).await.unwrap();

        /* surface config */
        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = *surface_capabilities.formats
            .iter()
            .find(|format| format.is_srgb())
            .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        /* msaa */
        let formats = [surface_format, wgpu::TextureFormat::Depth32Float];
        let msaa_sample_counts = wgpu_helpers::get_avaiable_msaa_sample_counts(&adapter, formats.iter().copied());
        
        print!("    available sample counts: [1");
        for msaa_sample_count in msaa_sample_counts.iter().copied() {
            print!(", {msaa_sample_count}");
        }
        println!("]\n");

        /* game */
        let game = Game::new(&device, &queue, surface_format, size, &msaa_sample_counts[..]);

        /* egui */
        let egui_context = egui::Context::default();

        let egui_state = egui_winit::State::new(
            egui_context,
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            Some(2048),
        );

        let egui_renderer = egui_wgpu::Renderer::new(
            &device,
            surface_format,
            egui_wgpu::RendererOptions {
                msaa_samples: 1,
                depth_stencil_format: None,
                dithering: true, // TODO
                predictable_texture_filtering: false,
            },
        );

        /* finish */

        let start_time = Instant::now();
        let last_time = start_time;

        let last_surface_texture_wait = 0;

        AppWgpu {
            window,
            surface,
            device,
            queue,
            surface_config,
            start_time,
            last_time,
            game,
            egui_state,
            egui_renderer,
            last_surface_texture_wait,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.surface_config.width = size.width;
            self.surface_config.height = size.height;

            self.surface.configure(&self.device, &self.surface_config);

            self.game.resize(&self.device, size);
        }
    }

    fn key_event(&mut self, key: KeyCode, state: ElementState) {
        self.game.key_event(key, state);
    }

    fn render(&mut self) {
        /* timing */
        let now = Instant::now();
        let t = (now - self.start_time).as_millis();
        let dt = (now - self.last_time).as_millis();

        self.last_time = now;

        /* update egui */
        let egui_input = self.egui_state.take_egui_input(&self.window);

        let egui_output = self.egui_state.egui_ctx().run(egui_input, |ctx| {
            ctx.style_mut(|style| style.visuals.window_shadow = egui::Shadow::NONE); // TODO: do this properly
            self.game.ui(ctx);
        });

        let egui_tris = self.egui_state.egui_ctx().tessellate(egui_output.shapes, self.egui_state.egui_ctx().pixels_per_point());

        for (texture_id, texture_delta) in &egui_output.textures_delta.set {
            self.egui_renderer.update_texture(&self.device, &self.queue, *texture_id, texture_delta);
        }

        for texture_id in &egui_output.textures_delta.free {
            self.egui_renderer.free_texture(texture_id);
        }

        self.egui_state.handle_platform_output(&self.window, egui_output.platform_output);

        /* update game */
        self.game.update(&self.queue, t, dt, self.last_surface_texture_wait);

        /* surface texture */
        let surface_texture_start = Instant::now();

        let surface_texture = self.surface.get_current_texture().unwrap(); // this blocks, limits fps
        if surface_texture.suboptimal {
            println!("surface suboptimal");
        }

        let surface_texture_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let surface_texture_end = Instant::now();
        self.last_surface_texture_wait = (surface_texture_end - surface_texture_start).as_millis();

        // TODO: try to render as as soon as we have texture (we are here)
        //       e.g. don't do unnecessary thing from here until we submit the command encoder
        //       this is not valid idea ?

        /* render */
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        self.game.render(&mut encoder, &surface_texture_view);

        /* render egui */
        let egui_screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.surface_config.width, self.surface_config.height],
            pixels_per_point: self.egui_state.egui_ctx().pixels_per_point(), // TODO
        };

        self.egui_renderer.update_buffers(&self.device, &self.queue, &mut encoder, &egui_tris, &egui_screen_descriptor);

        {
            let render_pass_egui = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            self.egui_renderer.render(&mut render_pass_egui.forget_lifetime(), &egui_tris, &egui_screen_descriptor);
        }

        /* render finish */
        self.queue.submit(std::iter::once(encoder.finish()));

        surface_texture.present();
    }
}


#[derive(Default)]
struct AppRunner {
    app: Option<AppWgpu>,
}

impl ApplicationHandler for AppRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // let monitor = event_loop.primary_monitor().unwrap();
        // let video_mode = monitor.video_modes().next().unwrap();

        let window_attributes = Window::default_attributes()
            .with_title("stromy")
            // .with_fullscreen(Some(Fullscreen::Borderless(None)));
            // .with_fullscreen(Some(Fullscreen::Exclusive(video_mode)));
            .with_inner_size(PhysicalSize::new(1020, 780));

        let window = event_loop.create_window(window_attributes).unwrap();
        let app = pollster::block_on(AppWgpu::new(window));

        self.app = Some(app);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        /* close event */
        match event {
            // close button | ESC key
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput { event: KeyEvent { physical_key: PhysicalKey::Code(KeyCode::Escape), .. }, .. } => {
                event_loop.exit();
                return;
            }
            _ => {}
        };

        /* app event */
        let app = if let Some(app) = self.app.as_mut() { app } else { return };

        // repaint not used since we draw continously
        // TODO: consider ignoring `consumed` on resize and redraw
        // TODO: `AppWgpu` method
        let egui_winit::EventResponse { consumed, .. } = app.egui_state.on_window_event(&app.window, &event);
        if consumed {
            return;
        }

        match event {
            WindowEvent::Resized(size) => {
                app.resize(size)
            },
            WindowEvent::RedrawRequested => {
                app.render();
                app.window.request_redraw(); // TODO: how to render continously
            },
            WindowEvent::KeyboardInput { event: KeyEvent { physical_key: PhysicalKey::Code(key), state, .. }, .. } => {
                app.key_event(key, state);
            },
            _ => {},
        }
    }
}



fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = AppRunner::default();
    event_loop.run_app(&mut app).unwrap();
}
