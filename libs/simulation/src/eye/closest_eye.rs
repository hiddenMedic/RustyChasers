use crate::*;

#[derive(Debug, Clone)]
pub struct ClosestEye{
    fov_range: f32,
    fov_angle: f32,
}

impl Eye for ClosestEye{
    fn new(fov_range: f32, fov_angle: f32) -> Self where Self:Sized {
        assert!(fov_angle > 0.0);
        assert!(fov_range > 0.0);
        
        Self { fov_range, fov_angle}
    }

    fn process_vision_see_chasers(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, chasers: &[Chaser]) -> Vec<f32> {
        let mut cells = vec![0.0; 4]; //for closest chaser (their rotation, angle to them, distance, count)

        let mut closest_angle: f32 = 0f32;
        let mut closest_dist: f32 = f32::MAX;
        let mut closest_h_angle: f32 = 0f32;
        let mut nseen: usize = 0;

        for i in 0..chasers.len() {
            let chaser = &chasers[i];
            let vec = chaser.position - position;
            let dist = vec.norm();
            if dist > self.fov_range { //out of range
                continue;
            }
            let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 { //out of angle
                continue;
            }

            nseen += 1;
            if dist < closest_dist {
                closest_dist = dist;
                closest_angle = angle;
                closest_h_angle = chaser.rotation.angle() - rotation.angle();
            }
        }

        if nseen == 0 {
            closest_dist = 0f32;
        }
        cells[0] = closest_h_angle;
        cells[1] = closest_dist;
        cells[2] = closest_angle;
        cells[3] = nseen as f32;

        return cells;
    }

    fn process_vision_see_plants(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, plants: &[Plant]) -> Vec<f32> {
        let mut cells = vec![0.0; 3]; //for closest plant (angle to them, distance, count)

        let mut closest_angle: f32 = 0f32;
        let mut closest_dist: f32 = f32::MAX;
        let mut nseen: usize = 0;

        for i in 0..plants.len() {
            let plant = &plants[i];
            let vec = plant.position - position;
            let dist = vec.norm();
            if dist > self.fov_range { //out of range
                continue;
            }
            let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 { //out of angle
                continue;
            }

            nseen += 1;
            if dist < closest_dist {
                closest_dist = dist;
                closest_angle = angle;
            }
        }

        if nseen == 0 {
            closest_dist = 0f32;
        }
        cells[0] = closest_dist;
        cells[1] = closest_angle;
        cells[2] = nseen as f32;

        return cells;
    }

    fn process_vision_see_hervors(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, hervors: &[Hervor]) -> Vec<f32> {
        let mut cells = vec![0.0; 4]; //for closest hervor (their rotation, angle to them, distance, count)

        let mut closest_angle: f32 = 0f32;
        let mut closest_dist: f32 = f32::MAX;
        let mut closest_h_angle: f32 = 0f32;
        let mut nseen: usize = 0;

        for i in 0..hervors.len() {
            let hervor = &hervors[i];
            let vec = hervor.position - position;
            let dist = vec.norm();
            if dist > self.fov_range { //out of range
                continue;
            }
            let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 { //out of angle
                continue;
            }

            nseen += 1;
            if dist < closest_dist {
                closest_dist = dist;
                closest_angle = angle;
                closest_h_angle = hervor.rotation.angle() - rotation.angle();
            }
        }

        if nseen == 0 {
            closest_dist = 0f32;
        }
        cells[0] = closest_h_angle;
        cells[1] = closest_dist;
        cells[2] = closest_angle;
        cells[3] = nseen as f32;

        return cells;
    }
}