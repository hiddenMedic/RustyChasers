use crate::*;

#[derive(Debug, Clone)]
pub struct HervorBrain {
    pub(crate) nn: nn::Network
}

impl Brain for HervorBrain{
    fn random(rng: &mut dyn RngCore, sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(sim_conf, hervor_conf))
        }
    }

    fn as_chromosome(&self) -> Chromosome {
        Chromosome::new(self.nn.weights())
    }

    fn from_chromosome(chromosome: Chromosome, sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(sim_conf, hervor_conf), chromosome)
        }
    }

    fn input_size(sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig) -> usize {
        match hervor_conf.training_model {
            Model::POSITIONAL => {
                sim_conf.nchasers * 4 + 3 + 1
            }
            Model::CELLULAR => {
                hervor_conf.eye_cells + 1
            }
            Model::CLOSEST => {
                4 + 3 + 1
            }
        }
    }

    //edit topology here
    fn topology(sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig) -> Vec<nn::LayerTopology> {
        let in_size = Self::input_size(sim_conf, hervor_conf);

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