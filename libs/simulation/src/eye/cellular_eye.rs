use crate::*;

const DEFAULT_CELLS: usize = 9;

#[derive(Debug, Clone)]
pub struct CellularEye{
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye for CellularEye{
    fn new(fov_range: f32, fov_angle: f32) -> Self where Self:Sized {
        assert!(fov_angle > 0.0);
        assert!(fov_range > 0.0);
        
        Self { fov_range, fov_angle, cells:DEFAULT_CELLS}
    }

    fn process_vision_see_chasers(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, chasers: &[Chaser]) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for chaser in chasers {
            let vec = chaser.position - position;
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

            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }

        return cells;
    }

    fn process_vision_see_plants(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, plants: &[Plant]) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

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

            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }

        return cells;
    }

    fn process_vision_see_hervors(&self, position: &na::Point2<f32>, rotation: &na::Rotation2<f32>, hervors: &[Hervor]) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for hervor in hervors {
            let vec = hervor.position - position;
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

            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }

        return cells;
    }
}

/*
impl CellularEye {
    fn new_with_cells(fov_range: f32, fov_angle: f32, cells: usize) -> Self where Self:Sized {
        assert!(cells > 0);

        let mut s: Self = Self::new(fov_range, fov_angle); s.cells = cells;
        return s;
    }

    fn cells(&self) -> usize {
        return self.cells;
    }
}
*/