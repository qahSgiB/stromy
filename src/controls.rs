use nalgebra as na;

use controls_1d::Controls1D;
pub use direction::Direction;



mod controls_1d;
mod direction;



type V2 = na::Vector2<f32>;
type V3 = na::Vector3<f32>;
type M4 = na::Matrix4<f32>;



fn create_view_matrix(forward: V3, right: V3, up: V3, eye: V3) -> M4 {
    let rotate = M4::new(
        right.x   , right.y   , right.z   , 0.0,
        up.x      , up.y      , up.z      , 0.0,
        -forward.x, -forward.y, -forward.z, 0.0,
        0.0       , 0.0       , 0.0       , 1.0,
    );
    let translate = M4::new_translation(&-eye);

    rotate * translate
}



/// Represents controls camera orientation and orbit center.
/// Returned by `Controls::update`.
#[derive(Debug, Clone, Copy)]
pub struct ControlsState {
    pub view: M4,
    pub center: V3,
}

pub struct ControlsConfig {
    pub rotation_go_acc: f32,
    pub rotation_friction_acc: f32,
    pub rotation_max_vel: f32,
    pub rotation_vertical_max_pos: f32,
    pub rotation_drag_vel: f32,

    pub start_pos: V3,
    pub position_go_acc: f32,
    pub position_friction_acc: f32,
    pub position_max_vel: f32,

    pub orbit_radius: f32,
    pub orbit_radius_go_acc: f32,
    pub orbit_radius_friction_acc: f32,
    pub orbit_radius_max_vel: f32,
    pub orbit_radius_min: f32,
    pub orbit_radius_max: f32,
    pub orbit_radius_scroll_vel: f32,
}

pub struct Controls {
    rotation_horizontal: Controls1D,
    rotation_vertical: Controls1D,
    rotation_drag_vel: f32,

    position_horizontal: Controls1D,
    position_vertical: Controls1D,
    position_forward: Controls1D,
    position: V3,

    orbit_mode: bool,
    orbit_mode_switch_pending: bool,
    orbit_radius: Controls1D,
    orbit_radius_scroll_vel: f32,
}

impl Controls {
    pub fn new(config: ControlsConfig) -> Controls {
        Controls {
            rotation_horizontal:
                Controls1D::builder(config.rotation_go_acc, config.rotation_friction_acc, config.rotation_max_vel)
                    .build(),
            rotation_vertical:
                Controls1D::builder(config.rotation_go_acc, config.rotation_friction_acc, config.rotation_max_vel)
                    .with_clamp(-config.rotation_vertical_max_pos, config.rotation_vertical_max_pos )
                    .build(),
            rotation_drag_vel: config.rotation_drag_vel,
            position_horizontal:
                Controls1D::builder(config.position_go_acc, config.position_friction_acc, config.position_max_vel)
                    .with_start_pos(config.start_pos.x)
                    .build(),
            position_vertical:
                Controls1D::builder(config.position_go_acc, config.position_friction_acc, config.position_max_vel)
                    .with_start_pos(config.start_pos.y)
                    .build(),
            position_forward:
                Controls1D::builder(config.position_go_acc, config.position_friction_acc, config.position_max_vel)
                    .with_start_pos(config.start_pos.z)
                    .build(),
            position: config.start_pos,
            orbit_mode: true,
            orbit_mode_switch_pending: false,
            orbit_radius:
                Controls1D::builder(config.orbit_radius_go_acc, config.orbit_radius_friction_acc, config.orbit_radius_max_vel)
                    .with_clamp(config.orbit_radius_min, config.orbit_radius_max)
                    .with_start_pos(config.orbit_radius)
                    .build(),
            orbit_radius_scroll_vel: config.orbit_radius_scroll_vel,
        }
    }

    pub fn orbit_mode_is_orbiting(&self) -> bool {
        self.orbit_mode
    }

    pub fn rotation_horizontal_start(&mut self, direction: Direction) {
        self.rotation_horizontal.start(direction);
    }

    pub fn rotation_horizontal_end(&mut self, direction: Direction) {
        self.rotation_horizontal.end(direction);
    }

    pub fn rotation_vertical_start(&mut self, direction: Direction) {
        self.rotation_vertical.start(direction);
    }

    pub fn rotation_vertical_end(&mut self, direction: Direction) {
        self.rotation_vertical.end(direction);
    }

    pub fn position_horizontal_start(&mut self, direction: Direction) {
        self.position_horizontal.start(direction);
    }

    pub fn position_horizontal_end(&mut self, direction: Direction) {
        self.position_horizontal.end(direction);
    }

    pub fn position_vertical_start(&mut self, direction: Direction) {
        self.position_vertical.start(direction);
    }

    pub fn position_vertical_end(&mut self, direction: Direction) {
        self.position_vertical.end(direction);
    }

    pub fn position_forward_start(&mut self, direction: Direction) {
        self.position_forward.start(direction);
    }

    pub fn position_forward_end(&mut self, direction: Direction) {
        self.position_forward.end(direction);
    }

    pub fn orbit_radius_start(&mut self, direction: Direction) {
        self.orbit_radius.start(direction);
    }

    pub fn orbit_radius_end(&mut self, direction: Direction) {
        self.orbit_radius.end(direction);
    }

    pub fn switch_orbit_mode(&mut self) {
        self.orbit_mode_switch_pending = !self.orbit_mode_switch_pending;
    }

    pub fn drag_start(&mut self) {
        self.rotation_horizontal.stop();
        self.rotation_vertical.stop();
    }

    pub fn drag(&mut self, delta: V2) {
        self.rotation_horizontal.manual_move(delta.x * self.rotation_drag_vel);
        self.rotation_vertical.manual_move(delta.y * self.rotation_drag_vel);
    }

    pub fn scroll(&mut self, delta: f32) {
        self.orbit_radius.manual_move(delta * self.orbit_radius_scroll_vel);
    }

    fn get_camera_coords(&self) -> (V3, V3, V3) {
        let rotation_horizontal = self.rotation_horizontal.get_pos();
        let rotation_vertical = self.rotation_vertical.get_pos();

        let sa = na::ComplexField::sin(rotation_horizontal);
        let ca = na::ComplexField::cos(rotation_horizontal);
        let sb = na::ComplexField::sin(rotation_vertical);
        let cb = na::ComplexField::cos(rotation_vertical);

        let forward = V3::new(sa * cb, sb, -ca * cb);
        let right = V3::new(ca, 0.0, sa);
        let up = V3::new(-sa * sb, cb, ca * sb);

        (forward, right, up)
    }

    pub fn get_view_matrix(&self) -> M4 {
        let (forward, right, up) = self.get_camera_coords();
        let eye = if self.orbit_mode { self.position - forward * self.orbit_radius.get_pos() } else { self.position };

        create_view_matrix(forward, right, up, eye)
    }

    pub fn update(&mut self, dt: f32) -> Option<ControlsState> {
        /* update orientation */
        let rv_needs_update = self.rotation_vertical.update(dt);
        let rh_needs_update = self.rotation_horizontal.update(dt);

        let (forward, right, up) = self.get_camera_coords();

        /* update orbit radius */
        let or_needs_update = self.orbit_radius.update(dt);

        let orbit_radius = self.orbit_radius.get_pos();

        /* orbit mode switch */
        if self.orbit_mode_switch_pending {
            let orbit_switch_direction = if self.orbit_mode { -1.0 } else { 1.0 };
            self.position += orbit_switch_direction * forward * orbit_radius;

            self.rotation_vertical.flip_acc();
            self.rotation_horizontal.flip_acc();

            self.orbit_mode = !self.orbit_mode;
            self.orbit_mode_switch_pending = false;
        }

        /* update position */
        let ph_needs_update = self.position_horizontal.update(dt);
        if ph_needs_update {
            self.position += dt * right * self.position_horizontal.get_vel();
        }

        let pv_needs_update = self.position_vertical.update(dt);
        if pv_needs_update {
            self.position += dt * up * self.position_vertical.get_vel();
        }

        let pf_needs_update = self.position_forward.update(dt);
        if pf_needs_update {
            self.position += dt * forward * self.position_forward.get_vel();
        }

        /* calculate camera */
        let (eye, center) = if self.orbit_mode {
            (self.position - forward * orbit_radius, self.position)
        } else {
            (self.position, self.position + forward * orbit_radius)
        };

        (rv_needs_update || rh_needs_update || or_needs_update || ph_needs_update || pv_needs_update || pf_needs_update).then(|| {
            /* compute controls state */
            let view = create_view_matrix(forward, right, up, eye);

            ControlsState {
                view,
                center,
            }
        })
    }
}