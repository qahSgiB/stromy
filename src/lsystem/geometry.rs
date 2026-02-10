use std::f32;

use nalgebra as na;

use super::LSystemError;
use super::lsystem::{StringToken, string_iter_march_to_next_symbol, string_iter_take_symbol};



type V3 = na::Vector3<f32>;
type M4 = na::Matrix4<f32>;



#[derive(Debug, Clone, Copy)]
pub enum Action {
    Push,
    Pop,
    /// params: length, width
    Forward,
    /// params: angle
    Pitch,
    /// params: angle
    Yaw,
    /// params: angle
    Roll,
}


#[derive(Debug, Clone, Copy)]
struct Turtle {
    pos: V3,
    dir: V3,
    right: V3,
    up: V3,
}

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub pos: V3,
    pub x: V3,
    pub y: V3,
    pub z: V3,
    pub top_radius: f32,
}



pub fn lsystem_string_to_segments(string: &[StringToken], actions: &[Option<Action>]) -> Result<Vec<Segment>, LSystemError> {
    let mut turtle = Turtle {
        pos: V3::new(0.0, 0.0, 0.0),
        dir: V3::new(0.0, 1.0, 0.0),
        right: V3::new(1.0, 0.0, 0.0),
        up: V3::new(0.0, 0.0, -1.0),
    };

    let mut turtle_stack = Vec::new();

    let mut segments = Vec::new();

    let mut string_iter = string.iter().copied();

    let mut next_symbol = string_iter_take_symbol(&mut string_iter)?;

    while let Some(symbol) = next_symbol {
        // check if symbol corresponds to some action
        let action = match actions.get(symbol).copied().flatten() {
            Some(action) => action,
            None => {
                next_symbol = string_iter_march_to_next_symbol(&mut string_iter, |_| {});
                continue;
            },
        };

        // process action
        match action {
            Action::Push => {
                turtle_stack.push(turtle);
            },
            Action::Pop => {
                turtle = turtle_stack.pop().ok_or(LSystemError::TooMuchPop)?;
            },
            Action::Forward => {
                let step_length = string_iter_get_parameter(&mut string_iter)?;
                let width = string_iter_get_parameter(&mut string_iter)?;
                let top_radius = string_iter_get_parameter(&mut string_iter)?;

                let dir = turtle.dir * step_length;

                segments.push(Segment {
                    pos: turtle.pos,
                    x: dir,
                    y: turtle.right * width,
                    z: turtle.up * width,
                    top_radius: top_radius / width,
                });

                turtle.pos += dir;
            },
            Action::Pitch => {
                let angle = string_iter_get_parameter(&mut string_iter)?;

                let rot = na::Rotation3::new(turtle.right * angle * 2.0 * f32::consts::PI);

                turtle.dir = rot * turtle.dir;
                turtle.up = rot * turtle.up;
            },
            Action::Yaw => {
                let angle = string_iter_get_parameter(&mut string_iter)?;

                let rot = na::Rotation3::new(turtle.up * angle * 2.0 * f32::consts::PI);

                turtle.dir = rot * turtle.dir;
                turtle.right = rot * turtle.right;
            },
            Action::Roll => {
                let angle = string_iter_get_parameter(&mut string_iter)?;

                let rot = na::Rotation3::new(turtle.dir * angle * 2.0 * f32::consts::PI);

                turtle.right = rot * turtle.right;
                turtle.up = rot * turtle.up;
            },
        }

        next_symbol = string_iter_take_symbol(&mut string_iter)?;
    };

    Ok(segments)
}

fn string_iter_get_parameter(string_iter: &mut impl Iterator<Item = StringToken>) -> Result<f32, LSystemError> {
    match string_iter.next() {
        Some(StringToken::Value(value)) => Ok(value),
        _ => Err(LSystemError::ActionParameterExpected),
    }
}


pub fn segments_to_models_and_radiuses(segments: &[Segment]) -> (Vec<M4>, Vec<f32>) {
    segments
        .iter()
        .map(|segment| {
            let half_x = segment.x * 0.5;
            let center = segment.pos + half_x;

            // rotation and translation in one
            let model = M4::new(
                half_x.x, segment.y.x, segment.z.x, center.x,
                half_x.y, segment.y.y, segment.z.y, center.y,
                half_x.z, segment.y.z, segment.z.z, center.z,
                0.0     , 0.0        , 0.0        , 1.0     ,
            );

            let radius = segment.top_radius;

            (model, radius)
        })
        .unzip()
}