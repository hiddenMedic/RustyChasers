use rand::RngCore;
use rand::Rng;
use rand::seq::SliceRandom;
use lib_individual::*;
use lib_statistics::*;

//ooga booga do this: https://setu677.medium.com/how-to-perform-roulette-wheel-and-rank-based-selection-in-a-genetic-algorithm-d0829a37a189
pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I where I: Individual;
}

#[derive(Default)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I 
    where I: Individual {
        population
        .choose_weighted(rng, |individual| individual.fitness())
        .expect("got an empty population")
    }
}

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S> where S: SelectionMethod{
    pub fn new(selection_method: S, crossover_method: impl CrossoverMethod + 'static, mutation_method: impl MutationMethod + 'static) -> Self {
        Self { selection_method, crossover_method: Box::new(crossover_method), mutation_method: Box::new(mutation_method)}
    }

    //unoptimized AF function
    fn safe_evolve<I>(&self, rng: &mut dyn RngCore, mut population: Vec<I>) -> (Vec<I>, Statistics) where I: Individual {
        let stats = Statistics::new(&population);
        population.sort_by(|x, y| x.fitness().total_cmp(&y.fitness()));
        let mut top_50:Vec<I> = population[(population.len() / 2)..population.len()].iter().map(|ind| I::create_fit(ind.chromosome().clone(), ind.fitness())).collect();

        let mut addon_population:Vec<I> = (0..(population.len() - top_50.len()))
        .map(|_| {
            let parent1 = self.selection_method.select(rng, &top_50).chromosome();
            let parent2 = self.selection_method.select(rng, &top_50).chromosome();
            let mut child = self.crossover_method.crossover(rng, parent1, parent2);
            self.mutation_method.mutate(rng, &mut child);
            
            return I::create(child);
        }).collect();
        top_50.append(&mut addon_population);

        (top_50, stats)
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: Vec<I>, safe_evolve: bool) -> (Vec<I>, Statistics) where I: Individual{
        assert!(!population.is_empty());

        if safe_evolve {
            return self.safe_evolve(rng, population);
        }

        let stats = Statistics::new(&population);
        let new_population = (0..population.len())
        .map(|_| {
            let parent1 = self.selection_method.select(rng, &population).chromosome();
            let parent2 = self.selection_method.select(rng, &population).chromosome();
            let mut child = self.crossover_method.crossover(rng, parent1, parent2);
            self.mutation_method.mutate(rng, &mut child);
            
            return I::create(child);
        })
        .collect();

        (new_population, stats)
    }
}

pub trait CrossoverMethod: Send + Sync {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

#[derive(Clone, Debug, Default)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        let parent_a = parent_a.iter();
        let parent_b = parent_b.iter();
    
        parent_a
            .zip(parent_b)
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

pub trait MutationMethod: Send + Sync{
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    /// Probability of changing a gene:
    /// - 0.0 = no genes will be touched
    /// - 1.0 = all genes will be touched
    chance: f32,

    /// Magnitude of that change:
    /// - 0.0 = touched genes will not be modified
    /// - 3.0 = touched genes will be += or -= by at most 3.0
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}