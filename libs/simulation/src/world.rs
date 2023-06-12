use crate::{*, hervor::Hervor, chaser::Chaser};

#[derive(Debug)]
pub struct World{
    pub(crate) hervors: Vec<Hervor>,
    pub(crate) chasers: Vec<Chaser>,
    pub(crate) plants: Vec<Plant>,
    pub(crate) kill_count: usize //number of killed hervors
}
impl World {
    pub fn random(rng: &mut dyn RngCore, sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig, chaser_conf: &IndividualConfig) -> Self {
        let hervors = (0..sim_conf.nhervors)
            .map(|_| Hervor::random(sim_conf, hervor_conf, rng))
            .collect();

        let chasers = (0..sim_conf.nchasers)
        .map(|_| Chaser::random(sim_conf, chaser_conf, rng))
        .collect();

        let plants = (0..sim_conf.nplants)
            .map(|_| Plant::random(rng))
            .collect();
        
        //animals and plants can overlap :(, use e.g. Poisson disk sampling ( https://en.wikipedia.org/wiki/Supersampling)
        Self { hervors, chasers, plants , kill_count: 0}
    }

    pub fn hervors(&self) -> &[Hervor] {
        &self.hervors
    }

    pub fn chasers(&self) -> &[Chaser] {
        &self.chasers
    }

    pub fn plants(&self) -> &[Plant] {
        &self.plants
    }
}

impl World{
    pub fn extra_clone(&self, sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig, chaser_conf: &IndividualConfig) -> Self {
        let herv: Vec<Hervor> = self.hervors.iter().map(|x| x.extra_clone(sim_conf, hervor_conf)).collect();
        let chas: Vec<Chaser> = self.chasers.iter().map(|x| x.extra_clone(sim_conf, chaser_conf)).collect();

        Self{
            hervors: herv,
            chasers: chas,
            plants: self.plants.clone(),
            kill_count: self.kill_count
        }
    }
}