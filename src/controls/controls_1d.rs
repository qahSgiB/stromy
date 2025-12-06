use super::direction::Direction;



fn f32_max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

fn f32_min(a: f32, b: f32) -> f32 {
    if a > b { b } else { a }
}



pub struct Controls1DBuiler {
    /* config */
    go_acc: f32,
    friction_acc: f32,
    max_vel: f32,
    clamp_min: Option<f32>,
    clamp_max: Option<f32>,

    /* state */
    start_pos: Option<f32>,
}

impl Controls1DBuiler {
    fn new(go_acc: f32, friction_acc: f32, max_vel: f32) -> Controls1DBuiler {
        Controls1DBuiler {
            go_acc,
            friction_acc,
            max_vel,
            clamp_min: None,
            clamp_max: None,
            start_pos: None,
        }
    }

    pub fn with_clamp(&mut self, clamp_min: f32, clamp_max: f32) -> &mut Controls1DBuiler {
        self.with_clamp_min(clamp_min).with_clamp_max(clamp_max)
    }

    pub fn with_clamp_min(&mut self, clamp_min: f32) -> &mut Controls1DBuiler {
        self.clamp_min = Some(clamp_min);
        self
    }

    pub fn with_clamp_max(&mut self, clamp_max: f32) -> &mut Controls1DBuiler {
        self.clamp_max = Some(clamp_max);
        self
    }

    pub fn with_start_pos(&mut self, start_pos: f32) -> &mut Controls1DBuiler {
        self.start_pos = Some(start_pos);
        self
    }

    pub fn build(&self) -> Controls1D {
        Controls1D {
            go_acc: self.go_acc,
            friction_acc: self.friction_acc,
            max_vel: self.max_vel,
            clamp_min: self.clamp_min,
            clamp_max: self.clamp_max,
            direction: None,
            pos: self.start_pos.unwrap_or(0.0),
            vel: 0.0,
        }
    }
}


pub struct Controls1D {
    /* config */
    go_acc: f32,
    friction_acc: f32,
    max_vel: f32,
    clamp_min: Option<f32>,
    clamp_max: Option<f32>,

    /* state */
    direction: Option<Direction>,
    pos: f32,
    vel: f32,
}

impl Controls1D {
    pub fn builder(go_acc: f32, friction_acc: f32, max_vel: f32) -> Controls1DBuiler {
        Controls1DBuiler::new(go_acc, friction_acc, max_vel)
    }

    pub fn get_pos(&self) -> f32 {
        self.pos
    }

    pub fn get_vel(&self) -> f32 {
        self.vel
    }

    pub fn flip_acc(&mut self) {
        self.vel *= -1.0;
        self.go_acc *= -1.0;
    }

    pub fn start(&mut self, direction: Direction) {
        self.direction = Some(direction);
    }

    pub fn end(&mut self, direction: Direction) {
        if let Some(d) = self.direction && d == direction {
            self.direction = None;
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        /* basic update (infinite) */
        match self.direction {
            Some(direction) => {
                let acc = self.go_acc * direction.get_multiplier();
                self.vel = f32::clamp(self.vel + acc * dt, -self.max_vel, self.max_vel);
            },
            None => {
                if self.vel > 0.0 {
                    self.vel = f32_max(self.vel - self.friction_acc * dt, 0.0);
                } else if self.vel < 0.0 {
                    self.vel = f32_min(self.vel + self.friction_acc * dt, 0.0);
                }
            },
        }

        self.pos += self.vel * dt;

        /* clamping */
        if let Some(clamp_max) = self.clamp_max && self.pos >= clamp_max && self.vel > 0.0 {
            self.pos = clamp_max;
            self.vel = 0.0;
            true
        } else if let Some(clamp_min) = self.clamp_min && self.pos < clamp_min && self.vel < 0.0 {
            self.pos = clamp_min;
            self.vel = 0.0;
            true
        } else {
            self.vel != 0.0
        }
    }
}