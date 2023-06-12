//use nn::LayerTopology;

use crate::*;

#[derive(Debug)]
pub struct Hervor{
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Box<dyn Eye>,
    pub(crate) satiation: usize,
    pub(crate) dead: bool,
    pub(crate) brain: HervorBrain
}
impl Hervor {
    pub fn new(eye: Box<dyn Eye>, brain: HervorBrain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
            dead: false
        }
    }

    pub fn dead(&self) -> bool {
        return self.dead;
    }

    pub fn random(sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig, rng: &mut dyn rand::RngCore) -> Self {
        match hervor_conf.training_model {
            Model::POSITIONAL => {
                let peye = PositionalEye::new(hervor_conf.fov_range, hervor_conf.fov_angle);
                let brain = HervorBrain::random(rng, sim_conf, hervor_conf);
                Self::new(Box::new(peye), brain, rng)
            }
            Model::CELLULAR => {
                let ceye = CellularEye::new(hervor_conf.fov_range, hervor_conf.fov_angle);
                let brain = HervorBrain::random(rng, sim_conf, hervor_conf);
                Self::new(Box::new(ceye), brain, rng)
            }
            Model::CLOSEST => {
                let paeye = ClosestEye::new(hervor_conf.fov_range, hervor_conf.fov_angle);
                let brain = HervorBrain::random(rng, sim_conf, hervor_conf);
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

    pub(crate) fn from_chromosome(sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig, chromosome: Chromosome, rng: &mut dyn RngCore) -> Self {
        match hervor_conf.training_model {
            Model::POSITIONAL => {
                let peye = PositionalEye::new(hervor_conf.fov_range, hervor_conf.fov_angle);
                let brain = Brain::from_chromosome(chromosome, sim_conf, hervor_conf);
                Self::new(Box::new(peye), brain, rng)
            }
            Model::CELLULAR => {
                let ceye = CellularEye::new(hervor_conf.fov_range, hervor_conf.fov_angle);
                let brain = Brain::from_chromosome(chromosome, sim_conf, hervor_conf);
                Self::new(Box::new(ceye), brain, rng)
            }
            Model::CLOSEST => {
                let paeye = ClosestEye::new(hervor_conf.fov_range, hervor_conf.fov_angle);
                let brain = HervorBrain::from_chromosome(chromosome, sim_conf, hervor_conf);
                Self::new(Box::new(paeye), brain, rng)
            }
        }
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.brain.as_chromosome()
    }
}

impl Hervor{
    pub fn extra_clone(&self, sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig,) -> Self{
        Hervor{
            position: self.position,
            rotation: self.rotation,
            speed: self.speed,
            eye: match hervor_conf.training_model {
                Model::POSITIONAL => {
                    Box::new(PositionalEye::new(hervor_conf.fov_range, hervor_conf.fov_angle))
                },
                Model::CELLULAR => {
                    Box::new(CellularEye::new(hervor_conf.fov_range, hervor_conf.fov_angle))
                },
                Model::CLOSEST => {
                    Box::new(ClosestEye::new(hervor_conf.fov_range, hervor_conf.fov_angle))
                },
            },
            dead: self.dead,
            satiation: self.satiation,
            brain: self.brain.clone(),
        }
    }
}