use std::f32::consts::PI;
use crate::math::vec2::Vec2;
use crate::blueprints::blu_common::Blueprint;
use crate::components::com_draw2d::Draw2d;
use crate::components::com_move::Move;
use crate::components::com_controll_ball::ControlBall;
use crate::components::com_collide::Collide;

pub fn get_ball(x: f32, y: f32) -> Blueprint {
    let angle = PI + rand::random::<f32>() * PI;
    Blueprint {
        translation: Some(Vec2::new(x, y)),
        rotation: None,
        scale: None,
        using: vec![
            Box::new(Draw2d::new(Some(20), Some(20), Some([0, 255, 0, 255]))),
            Box::new(Move::new(Some(Vec2::new(1.0, 1.0)), Some(600.0))),
            Box::new(ControlBall::new(Some(angle))),
            Box::new(Collide::new(Some(Vec2::new(20.0, 20.0)))),
        ],
        children: None,
    }
}

