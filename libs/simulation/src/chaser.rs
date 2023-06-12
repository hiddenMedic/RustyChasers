//use nn::LayerTopology;

use crate::*;

#[derive(Debug)]
pub struct Chaser {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Box<dyn Eye>,
    pub(crate) killed: usize,
    pub(crate) brain: ChaserBrain
}
impl Chaser {
    pub fn new(eye: Box<dyn Eye>, brain: ChaserBrain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            killed: 0, 
        }
    }

    pub fn random(sim_conf: &SimulationConfig, chaser_conf: &IndividualConfig, rng: &mut dyn rand::RngCore) -> Self {
        match chaser_conf.training_model {
            Model::POSITIONAL => {
                let peye = PositionalEye::new(chaser_conf.fov_range, chaser_conf.fov_angle);
                let brain = Brain::random(rng, sim_conf, chaser_conf);
                Self::new(Box::new(peye), brain, rng)
            }
            Model::CELLULAR => {
                let ceye = CellularEye::new(chaser_conf.fov_range, chaser_conf.fov_angle);
                let brain = ChaserBrain::random(rng, sim_conf, chaser_conf);
                Self::new(Box::new(ceye), brain, rng)
            }
            Model::CLOSEST => {
                let paeye = ClosestEye::new(chaser_conf.fov_range, chaser_conf.fov_angle);
                let brain = ChaserBrain::random(rng, sim_conf, chaser_conf);
                Self::new(Box::new(paeye), brain, rng)
            }
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        return self.position;
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        return self.rotation;
    }

    pub(crate) fn from_chromosome(sim_conf: &SimulationConfig, chaser_conf: &IndividualConfig, chromosome: Chromosome, rng: &mut dyn RngCore) -> Self {
        match chaser_conf.training_model {
            Model::POSITIONAL => {
                let peye = PositionalEye::new(chaser_conf.fov_range, chaser_conf.fov_angle);
                let brain = Brain::from_chromosome(chromosome, sim_conf, chaser_conf);
                Self::new(Box::new(peye), brain, rng)
            }
            Model::CELLULAR => {
                let ceye = CellularEye::new(chaser_conf.fov_range, chaser_conf.fov_angle);
                let brain = Brain::from_chromosome(chromosome, sim_conf, chaser_conf);
                Self::new(Box::new(ceye), brain, rng)
            }
            Model::CLOSEST => {
                let paeye = ClosestEye::new(chaser_conf.fov_range, chaser_conf.fov_angle);
                let brain = Brain::from_chromosome(chromosome, sim_conf, chaser_conf);
                Self::new(Box::new(paeye), brain, rng)
            }
        }
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.brain.as_chromosome()
    }
}

impl Chaser{
    pub fn extra_clone(&self, sim_conf: &SimulationConfig, chaser_conf: &IndividualConfig,) -> Self{
        Chaser{
            position: self.position,
            rotation: self.rotation,
            speed: self.speed,
            eye: match chaser_conf.training_model {
                Model::POSITIONAL => {
                    Box::new(PositionalEye::new(chaser_conf.fov_range, chaser_conf.fov_angle))
                },
                Model::CELLULAR => {
                    Box::new(CellularEye::new(chaser_conf.fov_range, chaser_conf.fov_angle))
                },
                Model::CLOSEST => {
                    Box::new(ClosestEye::new(chaser_conf.fov_range, chaser_conf.fov_angle))
                },
            },
            killed: self.killed,
            brain: self.brain.clone(),
        }
    }
}