use crate::*;

#[derive(Clone)]
pub struct HervorIndividual{
    fitness: f32,
    chromosome: Chromosome,
}

impl Individual for HervorIndividual {
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

impl HervorIndividual{
    pub(crate) fn calc_fitness(hervor: &Hervor, nplants: usize) -> f32 {
        hervor.satiation as f32 + if hervor.dead {0.0} else {40.0} + if hervor.satiation == nplants {30.0} else {0.0}
    }

    pub(crate) fn from_hervor(hervor: &Hervor, nplants: usize) -> Self {
        Self{
            fitness: HervorIndividual::calc_fitness(hervor, nplants),
            chromosome: hervor.as_chromosome(),
        }
    }

    pub(crate) fn into_hervor(self, sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig, rng: &mut dyn RngCore) -> Hervor {
        Hervor::from_chromosome(sim_conf, hervor_conf, self.chromosome, rng)
    }
}