use crate::*;
//use std::f32::consts::*;

//const FOV_RANGE: f32 = 0.25;
//const FOV_ANGLE: f32 = PI + FRAC_PI_4;
//const CELLS: usize = 9;

pub trait Eye: Send + Sync {
    fn new(fov_range: f32, fov_angle: f32) -> Self where Self:Sized;
    fn process_vision_see_chasers(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, chasers: &[Chaser]) -> Vec<f32>;
    fn process_vision_see_plants(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, plants: &[Plant]) -> Vec<f32>;
    fn process_vision_see_hervors(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, hervors: &[Hervor]) -> Vec<f32>;
}

use core::fmt::Debug;
impl Debug for dyn Eye{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Eye{{}}")
    }
}