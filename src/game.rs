use std::f32;

use nalgebra as na;
use nalgebra::RealField;

use rand::distr::Distribution;

use winit::dpi::PhysicalSize;
use winit::event::ElementState;
use winit::keyboard::KeyCode;

use crate::controls::*;
use crate::lsystem;
use crate::lsystem::geometry::Segment;
use crate::models::cube;
use crate::rolling_average::RollingAvegare;
use crate::wgpu_helpers;
use crate::wgpu_helpers::pipo30::{BindGroupLayoutManager, Instanced3dPipeline, InstancedQcylPipeline, SurfaceManager};



type V3 = na::Vector3<f32>;
type V4 = na::Vector4<f32>;
type M4 = na::Matrix4<f32>;



fn segments_center(segments: &[lsystem::geometry::Segment]) -> V3 {
    let (min,max) = segments
        .iter()
        .flat_map(|segment| {
            [segment.pos, segment.pos + segment.x]
        })
        .fold((V3::from_element(f32::INFINITY), V3::from_element(f32::NEG_INFINITY)), |(min, max), vertex| {
            (min.inf(&vertex), max.sup(&vertex))
        });

    (min + max) / 2.0
}



struct GameTimingStats {
    surface_texture_wait: RollingAvegare<u32, 60>,
    dt: RollingAvegare<u32, 60>,
}

pub struct Game {
    surface_manager: SurfaceManager,
    pipeline_3d: Instanced3dPipeline,
    pipeline_qcyl: InstancedQcylPipeline,

    /* view */
    view_buffer: wgpu::Buffer,
    view_bindgroup: wgpu::BindGroup,

    /* projection */
    projection: M4,
    projection_needs_update: bool,

    projection_buffer: wgpu::Buffer,
    projection_bindgroup: wgpu::BindGroup,

    /* tree (instanced qcyl) */
    tree_resolution: usize,
    tree_resolution_needs_update: bool,
    tree_smooth_normals: bool,
    tree_smooth_normals_needs_update: bool,

    tree_model_buffer: wgpu::Buffer,
    tree_model_it_buffer: wgpu::Buffer,
    tree_radiuses_buffer: wgpu::Buffer,
    tree_color_buffer: wgpu::Buffer,
    tree_resolution_buffer: wgpu::Buffer,
    tree_smooth_normals_buffer: wgpu::Buffer,
    tree_bindgroup: wgpu::BindGroup,

    /* leaves (instanced qcyl) */
    leaves_resolution: usize,

    leaves_model_buffer: wgpu::Buffer,
    leaves_model_it_buffer: wgpu::Buffer,
    leaves_radiuses_buffer: wgpu::Buffer,
    leaves_color_buffer: wgpu::Buffer,
    leaves_resolution_buffer: wgpu::Buffer,
    leaves_smooth_normals_buffer: wgpu::Buffer,
    leaves_bindgroup: wgpu::BindGroup,

    /* orbit cube (instanced 3d) */
    orbit_cube_model_buffer: wgpu::Buffer,
    orbit_cube_model_it_buffer: wgpu::Buffer,
    orbit_cube_color_buffer: wgpu::Buffer,
    orbit_cube_bindgroup: wgpu::BindGroup,

    /* cube vertices */
    cube_vertices_buffer: wgpu::Buffer,
    cube_normals_buffer: wgpu::Buffer,

    /* controls */
    controls: Controls,

    /* timing stats */
    timing_stats: GameTimingStats,
}

impl Game {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, surface_format: wgpu::TextureFormat, surface_size: PhysicalSize<u32>, msaa_sample_counts: &[u32]) -> Game {
        let segments = Game::init_l_system();

        let (tree_models, tree_radiuses, center) = Game::init_tree(&segments);

        let (leaves_models, leaves_radiuses) = Game::init_leaves(&segments);

        let controls = Game::init_controls(center);

        let bind_group_layout_manager = BindGroupLayoutManager::new(device);

        let surface_manager = Game::init_surface_manager(device, surface_size, surface_format, msaa_sample_counts);

        let (pipeline_3d, pipeline_qcyl) = Game::init_pipelines(device, &bind_group_layout_manager, &surface_manager);

        let (
            projection,
            view_buffer,
            projection_buffer,
            view_bindgroup,
            projection_bindgroup,
        ) = Game::init_view_projection(device, &bind_group_layout_manager, surface_size, &controls);

        let (
            tree_resolution,
            tree_smooth_normals,
            tree_model_buffer,
            tree_model_it_buffer,
            tree_radiuses_buffer,
            tree_color_buffer,
            tree_resolution_buffer,
            tree_smooth_normals_buffer,
            tree_bindgroup,
        ) = Game::init_tree_wgpu(device, &bind_group_layout_manager, &tree_models, &tree_radiuses);

        let (
            leaves_resolution,
            leaves_smooth_normals,
            leaves_model_buffer,
            leaves_model_it_buffer,
            leaves_radiuses_buffer,
            leaves_color_buffer,
            leaves_resolution_buffer,
            leaves_smooth_normals_buffer,
            leaves_bindgroup,
        ) = Game::init_leaves_wgpu(device, &bind_group_layout_manager, &leaves_models, &leaves_radiuses);

        let (
            orbit_cube_model_buffer,
            orbit_cube_model_it_buffer,
            orbit_cube_color_buffer,
            orbit_cube_bindgroup,
        ) = Game::init_orbit_cube_wgpu(device, &bind_group_layout_manager);

        let (cube_vertices_buffer, cube_normals_buffer) = Game::init_cube_vertices_wgpu(device);

        // TODO: init fn
        let timing_stats = GameTimingStats {
            surface_texture_wait: RollingAvegare::new(),
            dt: RollingAvegare::new(),
        };

        Game {
            surface_manager,
            pipeline_3d,
            pipeline_qcyl,
            view_buffer,
            view_bindgroup,
            projection,
            projection_needs_update: false,
            projection_buffer,
            projection_bindgroup,
            tree_resolution,
            tree_resolution_needs_update: false,
            tree_smooth_normals,
            tree_smooth_normals_needs_update: false,
            tree_model_buffer,
            tree_model_it_buffer,
            tree_color_buffer,
            tree_resolution_buffer,
            tree_smooth_normals_buffer,
            tree_bindgroup,
            tree_radiuses_buffer,
            leaves_resolution,
            leaves_model_buffer,
            leaves_model_it_buffer,
            leaves_radiuses_buffer,
            leaves_color_buffer,
            leaves_resolution_buffer,
            leaves_smooth_normals_buffer,
            leaves_bindgroup,
            orbit_cube_model_buffer,
            orbit_cube_model_it_buffer,
            orbit_cube_color_buffer,
            orbit_cube_bindgroup,
            cube_vertices_buffer,
            cube_normals_buffer,
            controls,
            timing_stats,
        }
    }

    fn init_l_system() -> Vec<Segment> {
        let binary_tree = lsystem::lsystem::loader::load_from_file("assets/lsystems/thesis_rand_tree_300.txt").unwrap();
        let segments = binary_tree.expand_to_geometry(7).unwrap();

        println!("segments count: {}\n", segments.len());

        segments
    }

    fn init_tree(segments: &[Segment]) -> (Vec<M4>, Vec<f32>, V3) {
        let models = lsystem::geometry::segments_to_models(&segments);

        let center = segments_center(&segments);
        let center = V3::new(0.0, center.y, 0.0);

        let radiuses = segments.iter().map(|segment| segment.top_radius).collect::<Vec<_>>();

        (models, radiuses, center)
    }

    fn init_leaves(segments: &[Segment]) -> (Vec<M4>, Vec<f32>) {
        let offset_dist = rand::distr::Uniform::new(0.0, 1.0).unwrap(); // TODO: unwrap_unchecked
        let angle_dist = rand::distr::Uniform::new(0.0, 2.0 * f32::pi()).unwrap(); // TODO: unwrap_unchecked

        // let width_mult_dist = rand_ ::Uniform::new(0.6, 1.4).unwrap(); // TODO: unwrap_unchecked
        let width_mult_dist = rand_distr::Normal::new(1.0, 0.175).unwrap(); // TODO: unwrap_unchecked

        let count_var = 0.4; // 95%

        let models = segments.into_iter()
            .filter(|segment| segment.y.norm() < 0.03)
            .flat_map(|segment| {
                let branch_length = segment.x.norm();
                let branch_width = segment.y.norm();

                let width = branch_width * 0.75;
                let length = width * 35.0;

                let spacing = width * 3.5;
                let count = branch_length / spacing;

                let count_dist = rand_distr::Normal::new(count, count_var / 2.0).unwrap(); // TODO: unwrap_unchecked

                let mut rng = rand::rng(); // TODO: move rng outwards
                let count = count_dist.sample(&mut rng).round() as usize;

                (0..count).map(move |_| {
                    let mut rng = rand::rng(); // TODO: move rng outwards

                    let offset = offset_dist.sample(&mut rng);
                    let angle = angle_dist.sample(&mut rng);

                    let c = f32::cos(angle);
                    let s = f32::sin(angle);
    
                    let x = segment.y * c + segment.z * s;
                    let y = -segment.y * s + segment.z * c;
                    let z = segment.x;

                    let width_mult = width_mult_dist.sample(&mut rng).max(0.2);
    
                    let half_x = x.normalize() * length * width_mult * 0.5;
                    let y = y.normalize() * width * width_mult;
                    let z = z.normalize() * width * width_mult;
    
                    let center = segment.pos + segment.x * offset + half_x;
    
                    M4::new(
                        half_x.x, y.x, z.x, center.x,
                        half_x.y, y.y, z.y, center.y,
                        half_x.z, y.z, z.z, center.z,
                        0.0     , 0.0, 0.0, 1.0     ,
                    )
                })
            })
            .collect::<Vec<_>>();

        let radiuses = vec![0.4; models.len()];

        println!("leaves count: {}", models.len());

        (models, radiuses)
    }

    fn init_controls(center: V3) -> Controls {
        let rotation_max_vel = f32::pi() * 2.0 / 3.0;
        let rotation_go_acc = rotation_max_vel * 3.0;
        let rotation_friction_acc = rotation_max_vel * 4.0;
        let rotation_vertical_max_pos = f32::pi() / 2.0 * 5.0 / 6.0;

        let start_pos = center;
        let position_max_vel = 6.0;
        let position_go_acc = position_max_vel * 9.0;
        let position_friction_acc = position_max_vel * 12.0;

        let orbit_radius = 4.0;
        let orbit_radius_max_vel = 4.5;
        let orbit_radius_go_acc = orbit_radius_max_vel * 9.0;
        let orbit_radius_friction_acc = orbit_radius_max_vel * 12.0;
        let orbit_radius_min = 0.2;
        let orbit_radius_max = 6.0;

        Controls::new(ControlsConfig {
            rotation_go_acc,
            rotation_friction_acc,
            rotation_max_vel,
            rotation_vertical_max_pos,
            start_pos,
            position_go_acc,
            position_friction_acc,
            position_max_vel,
            orbit_radius,
            orbit_radius_go_acc,
            orbit_radius_friction_acc,
            orbit_radius_max_vel,
            orbit_radius_min,
            orbit_radius_max,
        })
    }

    fn init_surface_manager(device: &wgpu::Device, surface_size: PhysicalSize<u32>, surface_format: wgpu::TextureFormat, msaa_sample_counts: &[u32]) -> SurfaceManager {
        let msaa_sample_count = msaa_sample_counts.last().map_or(1, |&msaa_sample_count| msaa_sample_count); // max msaa

        SurfaceManager::new(device, surface_size, surface_format, msaa_sample_count)
    }

    fn init_pipelines(device: &wgpu::Device, bind_group_layout_manager: &BindGroupLayoutManager, surface_manager: &SurfaceManager) -> (Instanced3dPipeline, InstancedQcylPipeline) {
        let pipeline_3d = Instanced3dPipeline::new(device, bind_group_layout_manager, surface_manager);
        let pipeline_qcyl = InstancedQcylPipeline::new(device, bind_group_layout_manager, surface_manager);

        (pipeline_3d, pipeline_qcyl)
    }

    fn init_view_projection(device: &wgpu::Device, bind_group_layout_manager: &BindGroupLayoutManager, surface_size: PhysicalSize<u32>, controls: &Controls) -> (M4, wgpu::Buffer, wgpu::Buffer, wgpu::BindGroup, wgpu::BindGroup) {
        let view = controls.get_view_matrix();

        let aspect = (surface_size.width as f32) / (surface_size.height as f32);
        let fov = f32::pi() / 2.0;
        let projection = M4::new_perspective(aspect, fov, 0.01, 1000.0);

        let view_buffer = wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, &[view]);
        let projection_buffer = wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, &[projection]);

        let view_bindgroup = bind_group_layout_manager.create_view_bind_group(device, None, &view_buffer);
        let projection_bindgroup = bind_group_layout_manager.create_projection_bind_group(device, None, &projection_buffer);

        (projection, view_buffer, projection_buffer, view_bindgroup, projection_bindgroup)
    }

    fn init_tree_wgpu(device: &wgpu::Device, bind_group_layout_manager: &BindGroupLayoutManager, tree_models: &[M4], tree_radiuses: &[f32]) -> (usize, bool, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::BindGroup) {
        let color = V4::new(0.65, 0.2, 0.2, 1.0);
        let resolution = 6;
        let smooth_normals = false;

        let models_it = tree_models.iter().map(|cm| cm.try_inverse().unwrap().transpose()).collect::<Vec<_>>();

        let model_buffer = wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::VERTEX, &tree_models[..]);
        let model_it_buffer =  wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::VERTEX, &models_it[..]);

        let radiuses_buffer = wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::VERTEX, &tree_radiuses[..]);

        let color_buffer =  wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::UNIFORM, &[color]);
        let resolution_buffer =  wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, &[resolution]);
        let smooth_normals_buffer =  wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, &[usize::from(smooth_normals)]);

        let qcyl_bindgroup = bind_group_layout_manager.create_qcyl_bind_group(device, None, &color_buffer, &resolution_buffer, &smooth_normals_buffer);

        (resolution, smooth_normals, model_buffer, model_it_buffer, radiuses_buffer, color_buffer, resolution_buffer, smooth_normals_buffer, qcyl_bindgroup)
    }

    // TODO: use same buffers for tree and leaves ?
    fn init_leaves_wgpu(device: &wgpu::Device, bind_group_layout_manager: &BindGroupLayoutManager, leaves_models: &[M4], leaves_radiuses: &[f32]) -> (usize, bool, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::BindGroup) {
        let color = V4::new(0.15, 0.65, 0.2, 1.0);
        let resolution = 6;
        let smooth_normals = true;

        // TODO: dedup `init_tree_wgpu`
        let models_it = leaves_models.iter().map(|cm| cm.try_inverse().unwrap().transpose()).collect::<Vec<_>>();

        let model_buffer = wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::VERTEX, &leaves_models[..]);
        let model_it_buffer =  wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::VERTEX, &models_it[..]);

        let radiuses_buffer = wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::VERTEX, &leaves_radiuses[..]);

        let color_buffer =  wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::UNIFORM, &[color]);
        let resolution_buffer =  wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, &[resolution]);
        let smooth_normals_buffer =  wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, &[usize::from(smooth_normals)]);

        let qcyl_bindgroup = bind_group_layout_manager.create_qcyl_bind_group(device, None, &color_buffer, &resolution_buffer, &smooth_normals_buffer);

        (resolution, smooth_normals, model_buffer, model_it_buffer, radiuses_buffer, color_buffer, resolution_buffer, smooth_normals_buffer, qcyl_bindgroup)
    }

    fn init_orbit_cube_wgpu(device: &wgpu::Device, bind_group_layout_manager: &BindGroupLayoutManager) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::BindGroup) {
        let orbit_cube_color = V4::new(2.0, 0.0, 0.0, 1.0); // TODO: 2.0 color

        let orbit_cube_model = M4::new_scaling(0.04);
        let orbit_cube_model_it = orbit_cube_model.try_inverse().unwrap().transpose();

        let orbit_cube_model_buffer = wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, &[orbit_cube_model]);
        let orbit_cube_model_it_buffer =  wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, &[orbit_cube_model_it]);

        let orbit_cube_color_buffer =  wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::UNIFORM, &[orbit_cube_color]);

        let orbit_cube_bindgroup = bind_group_layout_manager.create_color_bind_group(device, None, &orbit_cube_color_buffer);

        (orbit_cube_model_buffer, orbit_cube_model_it_buffer, orbit_cube_color_buffer, orbit_cube_bindgroup)
    }

    fn init_cube_vertices_wgpu(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
        let cube_vertices_buffer = wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::VERTEX, &cube::CUBE_VERTICES);
        let cube_normals_buffer = wgpu_helpers::create_dumb_buffer_init(device, None, wgpu::BufferUsages::VERTEX, &cube::CUBE_NORMALS);

        (cube_vertices_buffer, cube_normals_buffer)
    }

    pub fn resize(&mut self, device: &wgpu::Device, surface_size: PhysicalSize<u32>) {
        /* projection */
        let aspect = (surface_size.width as f32) / (surface_size.height as f32);
        let fov = f32::pi() / 2.0;
        self.projection = M4::new_perspective(aspect, fov, 0.01, 1000.0);

        self.projection_needs_update = true;

        /* pipeline */
        self.surface_manager.resize(device, surface_size);
    }

    pub fn key_event(&mut self, key: KeyCode, state: ElementState) {
        match (state, key) {
            (ElementState::Pressed, KeyCode::KeyA) => self.controls.rotation_horizontal_start(Direction::Positive),
            (ElementState::Released, KeyCode::KeyA) => self.controls.rotation_horizontal_end(Direction::Positive),
            (ElementState::Pressed, KeyCode::KeyD) => self.controls.rotation_horizontal_start(Direction::Negative),
            (ElementState::Released, KeyCode::KeyD) => self.controls.rotation_horizontal_end(Direction::Negative),
            (ElementState::Pressed, KeyCode::KeyS) => self.controls.rotation_vertical_start(Direction::Positive),
            (ElementState::Released, KeyCode::KeyS) => self.controls.rotation_vertical_end(Direction::Positive),
            (ElementState::Pressed, KeyCode::KeyW) => self.controls.rotation_vertical_start(Direction::Negative),
            (ElementState::Released, KeyCode::KeyW) => self.controls.rotation_vertical_end(Direction::Negative),
            (ElementState::Pressed, KeyCode::KeyJ) => self.controls.position_horizontal_start(Direction::Negative),
            (ElementState::Released, KeyCode::KeyJ) => self.controls.position_horizontal_end(Direction::Negative),
            (ElementState::Pressed, KeyCode::KeyL) => self.controls.position_horizontal_start(Direction::Positive),
            (ElementState::Released, KeyCode::KeyL) => self.controls.position_horizontal_end(Direction::Positive),
            (ElementState::Pressed, KeyCode::KeyK) => self.controls.position_vertical_start(Direction::Negative),
            (ElementState::Released, KeyCode::KeyK) => self.controls.position_vertical_end(Direction::Negative),
            (ElementState::Pressed, KeyCode::KeyI) => self.controls.position_vertical_start(Direction::Positive),
            (ElementState::Released, KeyCode::KeyI) => self.controls.position_vertical_end(Direction::Positive),
            (ElementState::Pressed, KeyCode::KeyH) => self.controls.position_forward_start(Direction::Negative),
            (ElementState::Released, KeyCode::KeyH) => self.controls.position_forward_end(Direction::Negative),
            (ElementState::Pressed, KeyCode::KeyY) => self.controls.position_forward_start(Direction::Positive),
            (ElementState::Released, KeyCode::KeyY) => self.controls.position_forward_end(Direction::Positive),
            (ElementState::Pressed, KeyCode::KeyO) => self.controls.orbit_radius_start(Direction::Negative),
            (ElementState::Released, KeyCode::KeyO) => self.controls.orbit_radius_end(Direction::Negative),
            (ElementState::Pressed, KeyCode::KeyU) => self.controls.orbit_radius_start(Direction::Positive),
            (ElementState::Released, KeyCode::KeyU) => self.controls.orbit_radius_end(Direction::Positive),
            (ElementState::Released, KeyCode::KeyP) => self.controls.switch_orbit_mode(),
            _ => {},
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("abc").show(ctx, |ui| {
            let avg_dt = self.timing_stats.dt.get_average_f32();

            ui.label(format!("FPS : {:.0} ms", 1000.0 / avg_dt));
            ui.label(format!("frame delta : {:.0} ms", avg_dt));
            ui.label(format!("surface wait : {:.0} ms", self.timing_stats.surface_texture_wait.get_average_f32()));

            self.tree_smooth_normals_needs_update |= ui
                .checkbox(&mut self.tree_smooth_normals, "smooth normals")
                .changed();

            self.tree_resolution_needs_update |= ui
                .add(
                    egui::Slider::new(&mut self.tree_resolution, 4..=32)
                    .text("resolution")
                    .step_by(2.0)
                )
                .changed();
        });
    }

    pub fn update(&mut self, queue: &wgpu::Queue, t: u128, dt: u128, surface_texture_wait: u128) {
        let _t = t as f32;
        let dt_f = dt as f32;
        let dt_s = dt_f / 1000.0;

        let controls_update = self.controls.update(dt_s);

        if let Some(ControlsState { view, center }) = controls_update {
            queue.write_buffer(&self.view_buffer, 0, bytemuck::cast_slice(&[view]));

            let orbit_cube_model = M4::new_translation(&center) * M4::new_scaling(0.04);
            let orbit_cube_model_it = orbit_cube_model.try_inverse().unwrap().transpose();
    
            queue.write_buffer(&self.orbit_cube_model_buffer, 0, bytemuck::cast_slice(&[orbit_cube_model]));
            queue.write_buffer(&self.orbit_cube_model_it_buffer, 0, bytemuck::cast_slice(&[orbit_cube_model_it]));
        }

        if self.projection_needs_update {
            queue.write_buffer(&self.projection_buffer, 0, bytemuck::cast_slice(&[self.projection]));
            self.projection_needs_update = false;
        }

        if self.tree_resolution_needs_update {
            queue.write_buffer(&self.tree_resolution_buffer, 0, bytemuck::cast_slice(&[self.tree_resolution]));
            self.tree_resolution_needs_update = false;
        }

        if self.tree_smooth_normals_needs_update {
            queue.write_buffer(&self.tree_smooth_normals_buffer, 0, bytemuck::cast_slice(&[u32::from(self.tree_smooth_normals)]));
            self.tree_smooth_normals_needs_update = false;
        }

        /* timing stats */
        self.timing_stats.surface_texture_wait.add(surface_texture_wait as u32);
        self.timing_stats.dt.add(dt as u32);
    }

    pub fn render(&mut self, encoder: &mut wgpu::CommandEncoder, surface_texture_view: &wgpu::TextureView) {
        let clear_color = wgpu::Color { r: 0.65, g: 0.7, b: 1.0, a: 1.0 };
        let mut render_pass = self.surface_manager.begin_render_pass(encoder, surface_texture_view, clear_color);

        let mut render_pass_qcyl = self.pipeline_qcyl.use_pipeline(&mut render_pass);

        render_pass_qcyl.set_projection_bind_group(&self.projection_bindgroup);
        render_pass_qcyl.set_view_bind_group(&self.view_bindgroup);

        /* draw - tree */
        render_pass_qcyl.set_qcyl_bind_group(&self.tree_bindgroup);

        render_pass_qcyl.set_radiuses_vertex_buffer(self.tree_radiuses_buffer.slice(..));
        render_pass_qcyl.set_model_vertex_buffer(self.tree_model_buffer.slice(..), self.tree_model_it_buffer.slice(..));

        let count = self.tree_model_buffer.size() / 64;

        render_pass_qcyl.draw(self.tree_resolution as u32, 0..(count as u32));

        /* draw - tree */
        render_pass_qcyl.set_qcyl_bind_group(&self.leaves_bindgroup);

        render_pass_qcyl.set_radiuses_vertex_buffer(self.leaves_radiuses_buffer.slice(..));
        render_pass_qcyl.set_model_vertex_buffer(self.leaves_model_buffer.slice(..), self.leaves_model_it_buffer.slice(..));

        let count = self.leaves_model_buffer.size() / 64;

        render_pass_qcyl.draw(self.leaves_resolution as u32, 0..(count as u32));

        /* draw - orbit cube */
        if self.controls.orbit_mode_is_orbiting() || true {
            let cube_vertex_count = cube::CUBE_VERTICES.len();

            let mut render_pass_3d = self.pipeline_3d.use_pipeline(&mut render_pass);

            render_pass_3d.set_color_bind_group(&self.orbit_cube_bindgroup);

            render_pass_3d.set_object_vertex_buffer(self.cube_vertices_buffer.slice(..), self.cube_normals_buffer.slice(..));
            render_pass_3d.set_model_vertex_buffer(self.orbit_cube_model_buffer.slice(..), self.orbit_cube_model_it_buffer.slice(..));

            render_pass.draw(0..(cube_vertex_count as u32), 0..1);
        }
    }
}