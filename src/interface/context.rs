use macroquad::{camera::Camera2D, math::Vec2};

pub const CAM_SPEED: f32 = 10.;

pub struct Context {
    pub grid_size: Vec2,
    pub camera: Camera2D,
}
