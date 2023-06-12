pub use self::{plant::*, world::*, eye::*, brain::*, hervor::*, chaser::*,
    positional_eye::*,
    cellular_eye::*,
    closest_eye::*,
    brain::brain::*,
    eye::eye::*,
    config::*,
    hervor_brain::*,
    chaser_brain::*,
    lib_statistics::*,
    lib_individual::*,
    individual::{ chaser_individual::*, hervor_individual::*}
};

use std::{io::{BufWriter, Write}};
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod individual;
mod plant;
mod world;
mod hervor;
mod chaser;
mod config;
mod brain;
mod eye;

use serde_json;
pub use nalgebra as na;

use lib_statistics;
use lib_individual;
use lib_neural_network as nn;
pub use lib_genetic_algorithm as ga;
//use nn::Network;

pub use rand;
use rand::Rng;
use rand::RngCore;
use rayon::prelude::*;

use std::{f32::consts::PI, fs};
//const SPEED_MIN: f32 = 0.001;
//const SPEED_MAX: f32 = 0.005;
//const SPEED_ACCEL: f32 = 0.2;
//const ROTATION_ACCEL: f32 = PI / 32.0;
//const GENERATION_LENGTH: usize = 2500;
const EAT_RANGE:f32 = 0.02;

//utility function for benchmarking
pub fn time_function<F, T>(f: F) -> (T, Duration)
    where F: FnOnce() -> T {
    let start_time = Instant::now();
    let res = f();
    let elapsed = start_time.elapsed();

    (res, elapsed)
}

pub struct Simulation {
    worlds: Vec<World>,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
    generation: usize,
    sim_config: SimulationConfig,
    hervor_config: IndividualConfig,
    chaser_config: IndividualConfig,
}
impl Simulation {
    pub fn random(rng: &mut dyn rand::RngCore, sim_conf: SimulationConfig, hervor_conf: IndividualConfig, chaser_conf: IndividualConfig) -> Self {
        let mut worlds: Vec<World> = Vec::with_capacity(sim_conf.nworlds);
        
        for _ in 0..(sim_conf.nworlds){
            worlds.push(World::random(rng, &sim_conf, &hervor_conf, &chaser_conf));
        }
        
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(sim_conf.mutation_probability, sim_conf.mutation_magnitude)
        );

        Self {
            worlds, ga, age: 0, generation: 0, sim_config: sim_conf, hervor_config: hervor_conf, chaser_config: chaser_conf
        }
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn worlds(&self) -> &Vec<World> {
        &self.worlds
    }

    fn move_hervors_in_world(world: &mut World){
        for hervor in &mut world.hervors {
            if hervor.dead {
                continue;
            }

            hervor.position -= hervor.rotation * na::Vector2::new(0.0, hervor.speed);
            
            let offset:na::Vector2<f32> = hervor.rotation * na::Vector2::new(0.0, 0.03); //unforch hardcoded visual offset for border

            hervor.position.x = na::wrap(hervor.position.x + offset.x, 0.0, 1.0) - offset.x;
            hervor.position.y = na::wrap(hervor.position.y + offset.y, 0.0, 1.0) - offset.y;
        }
    }
    fn move_hervors(&mut self, world_index: usize){
        Self::move_hervors_in_world(&mut self.worlds[world_index]);
    }

    fn move_chasers_in_world(world: &mut World){
        for chaser in &mut world.chasers {
            chaser.position -= chaser.rotation * na::Vector2::new(0.0, chaser.speed);
            
            let offset:na::Vector2<f32> = chaser.rotation * na::Vector2::new(0.0, 0.03); //unforch hardcoded visual offset for border

            chaser.position.x = na::wrap(chaser.position.x + offset.x, 0.0, 1.0) - offset.x;
            chaser.position.y = na::wrap(chaser.position.y + offset.y, 0.0, 1.0) - offset.y;
        }
    }
    fn move_chasers(&mut self, world_index: usize){
        Self::move_chasers_in_world(&mut self.worlds[world_index]);
    }
    
    //improve with hit testing (https://en.wikipedia.org/wiki/Hit-testing)
    //should be taking rng as a parameter, but annoying with threading
    fn proc_eating_plants_in_world(world: &mut World, respawn_plants: bool){
        let mut rng = rand::thread_rng();

        for hervor in &mut world.hervors {
            for plant in &mut world.plants {
                if plant.eaten {
                    continue;
                }

                let dist = na::distance(&hervor.position, &plant.position);
                if dist <= EAT_RANGE {
                    hervor.satiation += 1;
                    if respawn_plants {
                        plant.position = rng.gen();
                    }
                    plant.eaten = true;
                }
            }
        }
    }
    fn proc_eating_plants(&mut self, world_index: usize){
        Self::proc_eating_plants_in_world(&mut self.worlds[world_index], self.sim_config.respawn_plants);
    }

    fn proc_eating_hervors_in_world(world: &mut World){
        for chaser in &mut world.chasers {
            for hervor in &mut world.hervors {
                let dist = na::distance(&hervor.position, &chaser.position);

                if dist <= EAT_RANGE && !hervor.dead{ //this is bad detection
                    chaser.killed += 1;
                    world.kill_count += 1;
                    hervor.dead = true;
                }
            }
        }
    }
    fn proc_eating_hervors(&mut self, world_index: usize){
        Self::proc_eating_hervors_in_world(&mut self.worlds[world_index]);
    }

    fn proc_hervor_brains_in_world(world: &mut World, age: usize, hervor_config: &IndividualConfig){
        for hervor in &mut world.hervors {
            let mut vision:Vec<f32> = hervor.eye.process_vision_see_plants(&hervor.position, &hervor.rotation, &world.plants);
            vision.append(&mut hervor.eye.process_vision_see_chasers(&hervor.position, &hervor.rotation, &world.chasers));
            vision.push(age as f32); //time
            let response = hervor.brain.nn.propagate(vision);
            
            //relavite values
            let speed = response[0].clamp(-hervor_config.speed_accel, hervor_config.speed_accel);
            let rotation = response[1].clamp(-hervor_config.rotation_accel, hervor_config.rotation_accel);

            hervor.speed = (hervor.speed + speed).clamp(hervor_config.speed_min, hervor_config.speed_max);
            hervor.rotation = na::Rotation2::new(hervor.rotation.angle() + rotation);
        }
    }
    fn proc_hervor_brains(&mut self, world_index: usize){
        Self::proc_hervor_brains_in_world(&mut self.worlds[world_index], self.age, &self.hervor_config)
    }

    fn proc_chaser_brains_in_world(world: &mut World, age: usize, chaser_conf: &IndividualConfig){
        for chaser in &mut world.chasers {
            let mut vision = chaser.eye.process_vision_see_hervors(&chaser.position, &chaser.rotation, &world.hervors);
            vision.push(age as f32);
            let response = chaser.brain.nn.propagate(vision);
            
            //relavite values
            let speed = response[0].clamp(-chaser_conf.speed_accel, chaser_conf.speed_accel);
            let rotation = response[1].clamp(-chaser_conf.rotation_accel, chaser_conf.rotation_accel);

            chaser.speed = (chaser.speed + speed).clamp(chaser_conf.speed_min, chaser_conf.speed_max);
            chaser.rotation = na::Rotation2::new(chaser.rotation.angle() + rotation);
        }
    }
    fn proc_chaser_brains(&mut self, world_index: usize){
        Self::proc_chaser_brains_in_world(&mut self.worlds[world_index], self.age, &self.chaser_config);
    }

    fn proc_world(world: &mut World, age: usize, sim_conf: &SimulationConfig, hervor_conf: &IndividualConfig, chaser_conf: &IndividualConfig){
        Self::proc_eating_plants_in_world(world, sim_conf.respawn_plants);
        Self::proc_eating_hervors_in_world(world);
        Self::proc_hervor_brains_in_world(world, age, &hervor_conf);
        Self::proc_chaser_brains_in_world(world, age, &chaser_conf);
        Self::move_hervors_in_world(world);
        Self::move_chasers_in_world(world);
    }

    //redundant function
    fn parallel_step_manual(world_slice: Arc<Mutex<&mut [World]>>, age: usize, sim_conf: SimulationConfig, hervor_conf: IndividualConfig, chaser_conf: IndividualConfig){
        let mut val = world_slice.lock();
        let sauce = val.as_deref_mut().unwrap().iter_mut();
        for x in sauce {
            Self::proc_world(x, age, &sim_conf, &hervor_conf, &chaser_conf);
        }
    }

    //redundant function
    pub fn run_parallel_step_manual(&mut self){
        let half = self.worlds.len() / 2;
        //let worlds_len = self.worlds.len();
        //let mut first_half: Vec<World> = (0..half).map(|i| self.worlds[i].extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();
        //let mut second_half: Vec<World> = (half..(self.worlds.len())).map(|i| self.worlds[i].extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();
        let (first_half, second_half) = self.worlds.split_at_mut(half);
        let first_half = Arc::new(Mutex::new(first_half));
        let first_half_thread = first_half.clone();
        let second_half = Arc::new(Mutex::new(second_half));
        let second_half_thread = second_half.clone();

        let age_copy = self.age.clone();
        let cha_conf_copy = self.chaser_config.clone();
        let herv_conf_copy = self.hervor_config.clone();
        let sim_conf_copy = self.sim_config.clone();

        std::thread::scope(|s| {
            let first_handle = s.spawn(move || 
                Self::parallel_step_manual(first_half_thread, age_copy, sim_conf_copy, herv_conf_copy, cha_conf_copy));
            
            let age_copy = self.age.clone();
            let cha_conf_copy = self.chaser_config.clone();
            let herv_conf_copy = self.hervor_config.clone();
            let sim_conf_copy = self.sim_config.clone();

            let second_handle = s.spawn(move || 
                Self::parallel_step_manual(second_half_thread, age_copy, sim_conf_copy, herv_conf_copy, cha_conf_copy));

            first_handle.join().expect("Thread error");
            second_handle.join().expect("Thread error");
        });
    }

    //redundant function
    pub fn run_parallel_step_rayon_manual(&mut self) {
        let wlen = self.worlds.len();
        let age_copy = self.age.clone();
        let cha_conf_copy = self.chaser_config.clone();
        let herv_conf_copy = self.hervor_config.clone();
        let sim_conf_copy = self.sim_config.clone();
        let slice = &mut self.worlds[..];
        let worlds_for_thread = Arc::new(Mutex::new(slice));

        //this shouldnt be parallel since it locks the whole worlds
        (0..wlen).into_par_iter().for_each(|x| Self::proc_world(&mut worlds_for_thread.lock().as_deref_mut().unwrap()[x], age_copy, &sim_conf_copy, &herv_conf_copy, &cha_conf_copy));
    }

    fn run_sequential_worlds(&mut self){
        for i in 0..self.worlds.len(){ //faster than calling proc_world
            self.proc_eating_plants(i); //no rng pass :(
            self.proc_eating_hervors(i);
            self.proc_hervor_brains(i);
            self.proc_chaser_brains(i);
            self.move_hervors(i);
            self.move_chasers(i);
        }
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<(Statistics, Statistics)>{
        if self.sim_config.parallelized {
            //self.run_parallel_step_manual(rng);
            //self.run_parallel_step_rayon_manual(rng);

            self.worlds.par_iter_mut().for_each(|world| Self::proc_world(world, self.age, &self.sim_config, &self.hervor_config, &self.chaser_config));
        }else{
            self.run_sequential_worlds();
        }
    
        self.age += 1;

        if self.age > self.sim_config.generation_length {
            self.generation += 1;
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    pub fn next_gen(&mut self, rng: &mut dyn RngCore) -> (Statistics, Statistics) {
        loop {
            if let Some(summary) = self.step(rng){
                return summary;
            }
        }
    }

    pub fn step_bench(&mut self, rng: &mut dyn RngCore, benchmark: usize) -> (Option<((Statistics, Statistics), [Duration; 4])>, [Duration; 5]){
        let fstart_time = Instant::now();

        let mut elapsed_return = [Duration::ZERO; 5];

        if benchmark == 2{
            let world_backup: Vec<World> = self.worlds.iter().map(|x| x.extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();

            let (_, tm1) = time_function(|| self.run_sequential_worlds());
            //let worlds_seq: Vec<World> = self.worlds.iter().map(|x| x.extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();
            self.worlds = world_backup.iter().map(|x| x.extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();

            let (_, tm2) = time_function(|| self.run_parallel_step_manual());
            //let worlds_par_manual: Vec<World> = self.worlds.iter().map(|x| x.extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();
            self.worlds = world_backup.iter().map(|x| x.extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();

            let (_, tm3) = time_function(|| self.run_parallel_step_rayon_manual());
            //let worlds_par_rayon_manual: Vec<World> = self.worlds.iter().map(|x| x.extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();
            self.worlds = world_backup.iter().map(|x| x.extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();

            let (_, tm4) = time_function(|| self.worlds.par_iter_mut().for_each(|world| Self::proc_world(world, self.age, &self.sim_config, &self.hervor_config, &self.chaser_config)));
            //let worlds_par_rayon: Vec<World> = self.worlds.iter().map(|x| x.extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();
            //dont load backup

            elapsed_return[0] = tm1; elapsed_return[1] = tm2; elapsed_return[2] = tm3; elapsed_return[3] = tm4;
        }else{ // == 1
            let (_, tm4) = time_function(|| self.worlds.par_iter_mut().for_each(|world| Self::proc_world(world, self.age, &self.sim_config, &self.hervor_config, &self.chaser_config)));
            //let worlds_par_rayon: Vec<World> = self.worlds.iter().map(|x| x.extra_clone(&self.sim_config, &self.hervor_config, &self.chaser_config)).collect();
            //dont load backup
            
            elapsed_return[0] = tm4;
        }
   

        //check if all functions performed the same operation, tested: this passes, commented out because of huge overhead
        /*
        for i in 0..(self.worlds.len()) {
            for j in 0..(self.worlds[i].hervors.len()) {
                assert_eq!(worlds_seq[i].hervors[j].position(), worlds_par_manual[i].hervors[j].position());
                assert_eq!(worlds_par_manual[i].hervors[j].position(), worlds_par_rayon_manual[i].hervors[j].position());
                assert_eq!(worlds_par_rayon_manual[i].hervors[j].position(), worlds_par_rayon[i].hervors[j].position());

                assert_eq!(worlds_seq[i].hervors[j].rotation(), worlds_par_manual[i].hervors[j].rotation());
                assert_eq!(worlds_par_manual[i].hervors[j].rotation(), worlds_par_rayon_manual[i].hervors[j].rotation());
                assert_eq!(worlds_par_rayon_manual[i].hervors[j].rotation(), worlds_par_rayon[i].hervors[j].rotation());
            }

            for j in 0..(self.worlds[i].chasers.len()) {
                assert_eq!(worlds_seq[i].chasers[j].position(), worlds_par_manual[i].chasers[j].position());
                assert_eq!(worlds_par_manual[i].chasers[j].position(), worlds_par_rayon_manual[i].chasers[j].position());
                assert_eq!(worlds_par_rayon_manual[i].chasers[j].position(), worlds_par_rayon[i].chasers[j].position());

                assert_eq!(worlds_seq[i].chasers[j].rotation(), worlds_par_manual[i].chasers[j].rotation());
                assert_eq!(worlds_par_manual[i].chasers[j].rotation(), worlds_par_rayon_manual[i].chasers[j].rotation());
                assert_eq!(worlds_par_rayon_manual[i].chasers[j].rotation(), worlds_par_rayon[i].chasers[j].rotation());
            }
        }
        */

        elapsed_return[4] = fstart_time.elapsed();
        
        self.age += 1;
        if self.age > self.sim_config.generation_length {
            self.generation += 1;
            (Some(self.evolve_bench(rng)), elapsed_return)
        } else {
            (None, elapsed_return)
        }
    }

    pub fn next_gen_bench(&mut self, rng: &mut dyn RngCore, benchmark: usize) -> (((Statistics, Statistics), [Duration; 4]), [Duration; 5]) {
        let mut total_time: [Duration; 5] = [Duration::new(0, 0); 5];

        loop {
            let (evolve_ret, tms) = self.step_bench(rng, benchmark);
            total_time[0] = total_time[0].saturating_add(tms[0]);
            total_time[1] = total_time[1].saturating_add(tms[1]);
            total_time[2] = total_time[2].saturating_add(tms[2]);
            total_time[3] = total_time[3].saturating_add(tms[3]);
            total_time[4] = total_time[4].saturating_add(tms[4]); 

            if evolve_ret.is_some() {
                return (evolve_ret.unwrap(), total_time);
            }
        }
    }

    pub fn multiple_gen(&mut self, amount:usize, rng: &mut dyn RngCore, benchmark: usize) -> (Statistics, Statistics) {
        let mut stats;
        let mut step_times: [Duration; 5];
        let mut total_step_time: [Duration; 5] = [Duration::new(0, 0); 5];
        let mut evolve_times: [Duration; 4];
        let mut total_evolve_time:[Duration; 4] = [Duration::ZERO; 4];

        if benchmark > 0 {
            ((stats, evolve_times), step_times) = self.next_gen_bench(rng, benchmark);
            (0..4).for_each(|i| { total_step_time[i] = total_step_time[i].saturating_add(step_times[i]); });
            (0..4).for_each(|i| { total_evolve_time[i] = total_evolve_time[i].saturating_add(evolve_times[i]); });
        } else {
            stats = self.next_gen(rng);
        }
       
        let f = OpenOptions::new()
            .append(true)
            .create(true)
            .open("save_data/training_stats.txt")
            .expect("Unable to open file");
        let mut f = BufWriter::new(f);
       
        for _ in 1..amount {
            //println!("Generation: ");
            f.write_all(&serde_json::to_string(&stats).unwrap().as_bytes()).expect("Unable to write data");
            if benchmark > 0{
                ((stats, evolve_times), step_times) = self.next_gen_bench(rng, benchmark);
                (0..5).for_each(|i| { total_step_time[i] = total_step_time[i].saturating_add(step_times[i]); });
                (0..4).for_each(|i| { total_evolve_time[i] = total_evolve_time[i].saturating_add(evolve_times[i]); });
            } else {
                stats = self.next_gen(rng);
            }
        }

        if benchmark > 0{
            let gn = self.generation;
            let wl = self.worlds.len();
            println!("Ran benchmark for {gn} generations for {wl} worlds.");
            if benchmark == 2 {
                println!("Total step runtimes: ");
                let t1 = total_step_time[0];
                println!("Sequential: {t1:?}");
                let t2 = total_step_time[1];
                println!("Manual parallel (2 threads): {t2:?}");
                let t3 = total_step_time[2];
                println!("Manual rayon on range : {t3:?}");
                let t4 = total_step_time[3];
                println!("Rayon: {t4:?}");
                let t5 = total_step_time[4];
                println!("Total step benching time: {t5:?}");

                println!("\nAverage generation step runtimes: ");
                let t1 = total_step_time[0].checked_div(self.generation as u32).unwrap();
                println!("Sequential: {t1:?}");
                let t2 = total_step_time[1].checked_div(self.generation as u32).unwrap();
                println!("Manual parallel (2 threads): {t2:?}");
                let t3 = total_step_time[2].checked_div(self.generation as u32).unwrap();
                println!("Manual rayon on range : {t3:?}");
                let t4 = total_step_time[3].checked_div(self.generation as u32).unwrap();
                println!("Rayon: {t4:?}");
                let t5 = total_step_time[4].checked_div(self.generation as u32).unwrap();
                println!("Average generation step benching time: {t5:?}");
            }else{ // ==1
                let t1 = total_step_time[0];
                println!("Total step runtime: {t1:?}");

                let t1 = total_step_time[0].checked_div(self.generation as u32).unwrap();
                println!("\nAverage generation step runtime: {t1:?}");
            }

            println!("\nTotal evolve runtimes: ");
            let t1 = total_evolve_time[3];
            println!("Total evolution (benching costs low): {t1:?}");
            let t2 = total_evolve_time[0];
            println!("Hervor evolution: {t2:?}");
            let t3 = total_evolve_time[1];
            println!("Chaser evolution: {t3:?}");
            let t4 = total_evolve_time[2];
            println!("Plant evolution: {t4:?}");

            println!("\nAverage generation evolve runtimes: ");
            let t1 = total_step_time[3].checked_div(self.generation as u32).unwrap();
            println!("Total evolution (benching costs low): {t1:?}");
            let t2 = total_step_time[0].checked_div(self.generation as u32).unwrap();
            println!("Hervor evolution: {t2:?}");
            let t3 = total_step_time[1].checked_div(self.generation as u32).unwrap();
            println!("Chaser evolution: {t3:?}");
            let t4 = total_step_time[2].checked_div(self.generation as u32).unwrap();
            println!("Plant evolution: {t4:?}");
        }
        stats
    }

    fn evolve_hervors(&mut self, rng: &mut dyn RngCore) -> Statistics{
        let mut current_population_hervors: Vec<HervorIndividual> = vec![];
        for world in &self.worlds {
            current_population_hervors.extend(world.hervors.iter().map(|x| HervorIndividual::from_hervor(x, self.sim_config.nplants)).collect::<Vec<HervorIndividual>>());
        }
        let (evolved_population_hervors, stats_hervors) = self.ga.evolve(rng, current_population_hervors, self.sim_config.safe_evolve);

        assert_eq!(evolved_population_hervors.len(), self.sim_config.nworlds * self.sim_config.nhervors);

        let mut i = 0; //hervor counter
        let mut j = 0; //world counter
        self.worlds[0].hervors.clear();
        for hervor_ind in evolved_population_hervors{
            self.worlds[j].hervors.push(hervor_ind.into_hervor(&self.sim_config, &self.hervor_config, rng));
            i += 1;
            if i == self.sim_config.nhervors && j < self.sim_config.nworlds - 1 {
                i = 0; j += 1;
                self.worlds[j].hervors.clear();
            }
        }

        stats_hervors
    }

    fn evolve_chasers(&mut self, rng: &mut dyn RngCore) -> Statistics{
        let mut current_population_chasers: Vec<ChaserIndividual> = vec![];
        for world in &self.worlds {
            current_population_chasers.extend(world.chasers.iter().map(|cind| ChaserIndividual::from_chaser(cind, world.kill_count)).collect::<Vec<ChaserIndividual>>());
        }

        let (evolved_population_chasers, stats_chasers) = self.ga.evolve(rng, current_population_chasers, self.sim_config.safe_evolve);

        assert_eq!(evolved_population_chasers.len(), self.sim_config.nworlds * self.sim_config.nchasers);

        let mut i = 0; //chaser counter
        let mut j = 0; //world counter
        self.worlds[0].chasers.clear();

        for chaser_ind in evolved_population_chasers {
            self.worlds[j].chasers.push(chaser_ind.into_chaser(&self.sim_config, &self.chaser_config, rng));

            i += 1;
            if i == self.sim_config.nchasers && j < self.sim_config.nworlds - 1{
                i = 0; j += 1;
                self.worlds[j].chasers.clear();
            }
        }

        stats_chasers
    }

    fn evolve_plants(&mut self, rng: &mut dyn RngCore){
        for world in &mut self.worlds{
            for plant in &mut world.plants {
                plant.position = rng.gen();
                plant.eaten = false;
            }
        }
    }

    fn evolve_bench(&mut self, rng: &mut dyn RngCore) -> ((Statistics, Statistics), [Duration; 4]){
        self.age = 0;

        let total_time = Instant::now();
        let mut stats_hervor = Statistics::empty();
        let mut stats_chasers = Statistics::empty();
        let mut her_tm = Duration::ZERO;
        let mut cha_tm = Duration::ZERO;
        let mut pla_tm = Duration::ZERO;

        if self.sim_config.nhervors > 0 {
            (stats_hervor, her_tm) = time_function(|| self.evolve_hervors(rng));
        }
        if self.sim_config.nchasers > 0 {
            (stats_chasers, cha_tm) = time_function(|| self.evolve_chasers(rng));
        }
        if self.sim_config.nplants > 0 {
            (_, pla_tm) = time_function(|| self.evolve_plants(rng));
        }

        ((stats_hervor, stats_chasers), [her_tm, cha_tm, pla_tm, total_time.elapsed()])
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> (Statistics, Statistics){
        self.age = 0;
        let mut stats_hervor = Statistics::empty();
        let mut stats_chasers = Statistics::empty();

        if self.sim_config.nhervors > 0 {
            stats_hervor = self.evolve_hervors(rng);
        }
        if self.sim_config.nchasers > 0 {
            stats_chasers = self.evolve_chasers(rng);
        }
        if self.sim_config.nplants > 0 {
            self.evolve_plants(rng);
        }

        (stats_hervor, stats_chasers)
    }

    pub fn save_simulation(&self){
        let text = &serde_json::to_string(&self.sim_config).unwrap();
        std::fs::write("save_data/config.json", text).expect("Unable to write file");
        let text = &serde_json::to_string(&self.hervor_config).unwrap();
        std::fs::write("save_data/hervor_config.json", text).expect("Unable to write file");
        let text = &serde_json::to_string(&self.chaser_config).unwrap();
        std::fs::write("save_data/chaser_config.json", text).expect("Unable to write file");

        let mut text = String::new();
        for world in &self.worlds{
            for hervor in &world.hervors {
                text.push_str(&serde_json::to_string(&hervor.brain.nn.weights()).unwrap());
                text.push_str("\n");
            }
            for chaser in &world.chasers {
                text.push_str(&serde_json::to_string(&chaser.brain.nn.weights()).unwrap());
                text.push_str("\n");
            }
        }

        std::fs::write("save_data/weights", text).expect("Unable to open file");
    }

    pub fn load_simulation(&mut self, rng: &mut dyn RngCore){
        let sim_conf: SimulationConfig = serde_json::from_str(&fs::read_to_string("save_data/config.json").expect("Unable to open file")).unwrap();
        let chaser_conf: IndividualConfig = serde_json::from_str(&fs::read_to_string("save_data/chaser_config.json").expect("Unable to open file")).unwrap();
        let hervor_conf: IndividualConfig = serde_json::from_str(&fs::read_to_string("save_data/hervor_config.json").expect("Unable to open file")).unwrap();

        self.sim_config = sim_conf;
        self.chaser_config = chaser_conf;
        self.hervor_config = hervor_conf;

        let binding = fs::read_to_string("save_data/weights").expect("Unable to open file");
        let text: Vec<&str> = binding.lines().collect();

        self.worlds = Vec::with_capacity(self.sim_config.nworlds);
        let mut line_cnt = 0;
        for i in 0..self.sim_config.nworlds {
            self.worlds.push(World::random(rng, &self.sim_config, &self.hervor_config, &self.chaser_config));
            for j in 0..self.sim_config.nhervors {
                let wei:Vec<f32> = serde_json::from_str(&text[line_cnt]).unwrap();
                let topo = HervorBrain::topology(&self.sim_config, &self.hervor_config);
                self.worlds[i].hervors[j].brain.nn.from_weights_inplace(&topo, wei);
                line_cnt += 1;
            }

            for j in 0..self.sim_config.nchasers {
                let wei:Vec<f32> = serde_json::from_str(&text[line_cnt]).unwrap();
                let topo = ChaserBrain::topology(&self.sim_config, &self.chaser_config);
                self.worlds[i].chasers[j].brain.nn.from_weights_inplace(&topo, wei);
                line_cnt += 1;
            }
        }
    }
}