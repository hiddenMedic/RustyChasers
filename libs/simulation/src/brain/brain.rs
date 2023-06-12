use crate::*;

pub trait Brain {
    fn random(rng: &mut dyn RngCore, sim_conf: &SimulationConfig, in_conf: &IndividualConfig) -> Self where Self: Sized;
    fn as_chromosome(&self) -> Chromosome;
    fn from_chromosome(chromosome: Chromosome, sim_conf: &SimulationConfig, in_conf: &IndividualConfig) -> Self where Self: Sized;
    fn topology(sim_conf: &SimulationConfig, in_conf: &IndividualConfig) -> Vec<nn::LayerTopology> where Self: Sized;
    fn input_size(sim_conf: &SimulationConfig, in_conf: &IndividualConfig) -> usize;
}
