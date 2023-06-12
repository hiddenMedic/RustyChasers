use crate::*;

#[derive(Debug, Clone)]
pub struct ChaserBrain{
    pub(crate) nn: nn::Network
}

impl Brain for ChaserBrain{
    fn random(rng: &mut dyn RngCore, sim_conf: &SimulationConfig, chaser_conf: &IndividualConfig) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(sim_conf, chaser_conf))
        }
    }

    fn as_chromosome(&self) -> Chromosome {
        Chromosome::new(self.nn.weights())
    }

    fn from_chromosome(chromosome: Chromosome, sim_conf: &SimulationConfig, chaser_conf: &IndividualConfig) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(sim_conf, chaser_conf), chromosome)
        }
    }

    fn input_size(sim_conf: &SimulationConfig, chaser_conf: &IndividualConfig) -> usize {
        match chaser_conf.training_model {
            Model::POSITIONAL => {
                sim_conf.nhervors * 4 + 1
            }
            Model::CELLULAR => {
                chaser_conf.eye_cells + 1
            }
            Model::CLOSEST => {
                4 + 1
            }
        }
    }

    //edit topology here
    fn topology(sim_conf: &SimulationConfig, chaser_conf: &IndividualConfig) -> Vec<nn::LayerTopology> {
        let in_size = Self::input_size(sim_conf, chaser_conf);

        vec![
            nn::LayerTopology {
                neurons: in_size,
            },
            nn::LayerTopology {
                neurons: 32,
            },
            nn::LayerTopology {
                neurons: 32,
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}
