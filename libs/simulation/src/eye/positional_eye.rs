use crate::*;

#[derive(Debug, Clone)]
pub struct PositionalEye{
    fov_range: f32,
    fov_angle: f32,
}

impl Eye for PositionalEye{
    fn new(fov_range: f32, fov_angle: f32) -> Self where Self:Sized {
        assert!(fov_angle > 0.0);
        assert!(fov_range > 0.0);
        
        Self { fov_range, fov_angle}
    }

    fn process_vision_see_chasers(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, chasers: &[Chaser]) -> Vec<f32> {
        let mut cells = vec![0.0; chasers.len() * 4]; //(x, y, can see)

        for i in 0..chasers.len() {
            let chaser = &chasers[i];
            let vec = chaser.position - position;
            let dist = vec.norm();
            if dist > self.fov_range { //out of range
                cells[i] = 0.0;
                cells[i + 1] = 0.0;
                cells[i + 2] = 0.0;
                cells[i + 3] = 0.0;
                continue;
            }
            let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 { //out of angle
                cells[i] = 0.0;
                cells[i + 1] = 0.0;
                cells[i + 2] = 0.0;
                cells[i + 3] = 0.0;
                continue;
            }
            
            let relx = chaser.position.x - position.x;
            let rely = chaser.position.y - position.y;
            let relangle = chaser.rotation.angle() - rotation.angle();
            cells[i] = relx;
            cells[i + 1] = rely;
            cells[i + 2] = relangle;
            cells[i + 3] = 1.0;
        }

        return cells;
    }

    fn process_vision_see_plants(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, plants: &[Plant]) -> Vec<f32> {
        let mut cells = vec![0.0, 0.0, 0.0];
        //closest relative coordinates, number of plants seen

        let mut min_dist: f32 = f32::MAX;
        let mut best_x: f32 = 0.0;
        let mut best_y: f32 = 0.0;
        let mut seen: f32 = 0.0;

        for plant in plants {
            let vec = plant.position - position;
            let dist = vec.norm();
            if dist > self.fov_range {
                continue;
            }
            let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }
            
            seen += 1.0;
            if dist < min_dist {
                min_dist = dist;
                best_x = plant.position.x - position.x;
                best_y = plant.position.y - position.y;
            }
        }

        cells[0] = best_x;
        cells[1] = best_y;
        cells[2] = seen;

        return cells;
    }

    fn process_vision_see_hervors(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, hervors: &[Hervor]) -> Vec<f32> {
        let mut cells = vec![0.0; hervors.len() * 4]; //(x, y, can see)

        for i in 0..hervors.len() {
            let hervor = &hervors[i];
            let vec = hervor.position - position;
            let dist = vec.norm();
            if dist > self.fov_range { //out of range
                cells[i] = 0.0;
                cells[i + 1] = 0.0;
                cells[i + 2] = 0.0;
                cells[i + 3] = 0.0;
                continue;
            }
            let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 { //out of angle
                cells[i] = 0.0;
                cells[i + 1] = 0.0;
                cells[i + 2] = 0.0;
                cells[i + 3] = 0.0;
                continue;
            }
            
            let relx = hervor.position.x - position.x;
            let rely = hervor.position.y - position.y;
            let relangle = hervor.rotation.angle() - rotation.angle();
            cells[i] = relx;
            cells[i + 1] = rely;
            cells[i + 2] = relangle;
            cells[i + 3] = 1.0;
        }

        return cells;
    }
}