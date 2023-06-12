use crate::*;

#[derive(Clone)]
pub struct ChaserIndividual{
    fitness: f32,
    chromosome: Chromosome,
}

impl Individual for ChaserIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn create_fit(chromosome: Chromosome, fitness: f32) -> Self {
        Self {
            fitness,
            chromosome,
        }
    }
    
    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl ChaserIndividual{
    pub(crate) fn calc_fitness(kill_count: usize) -> f32 {
        kill_count as f32 * 10.0
    }

    pub(crate) fn from_chaser(chaser: &Chaser, kill_count: usize) -> Self {
        Self{
            fitness: ChaserIndividual::calc_fitness(kill_count),
            chromosome: chaser.as_chromosome(),
        }
    }

    pub(crate) fn into_chaser(self, sim_conf: &SimulationConfig, chaser_conf: &IndividualConfig, rng: &mut dyn RngCore) -> Chaser {
        Chaser::from_chromosome(sim_conf, chaser_conf, self.chromosome, rng)
    }
}