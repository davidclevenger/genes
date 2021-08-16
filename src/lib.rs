use core::f64;
use std::usize;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub mod genes;
pub mod individual;
pub mod options;

use genes::*;
use individual::*;

const DEFAULT_POP_SIZE: u32 = 100;
const DEFAULT_MUT_RATE: f64 = 0.05;
const DEFAULT_TGT_TYPE: options::TargetType = options::TargetType::MAXIMIZE;
const DEFAULT_SEL_METH: options::SelectionMethod = options::SelectionMethod::Weighted;
const DEFAULT_CRS_METH: options::CrossoverMethod = options::CrossoverMethod::BARRIER;

pub trait Target {
    fn score(&mut self, genes: &Genes) -> f64;
}

/// API entrypoint
pub struct Optimizer<T: Target> {
    population: Vec<Individual>,
    n: u32,
    mutation_rate: f64,
    target: T,
    target_type: options::TargetType,
    selection_method: options::SelectionMethod,
    crossover_method: options::CrossoverMethod,
    rng: SmallRng
}

impl<T: Target> Optimizer<T>{
    fn new(
        size: u32,
        n: u32,
        mutation_rate: f64,
        target: T,
        target_type: options::TargetType,
        selection_method: options::SelectionMethod,
        crossover_method: options::CrossoverMethod
    ) -> Optimizer<T> {
        let mut population = Vec::with_capacity(size as usize);

        let mut rng = SmallRng::from_entropy();

        for _ in 0..size {
            let mut genes: Vec<u8> = vec![0u8; (n / 8) as usize];

            // random gene initialization
            for gene in genes.iter_mut() {
                *gene = rng.gen()
            }

            population.push(Individual::new_with_genes(genes));
        }

        // reuse the rng created above
        return Optimizer {
            population,
            n,
            mutation_rate,
            target,
            target_type,
            selection_method,
            crossover_method,
            rng
        };
    }

    /// perform a number of steps of evolution
    pub fn evolve(&mut self, epochs: u32) {
        for _ in 0..epochs {
            self.step()
        }
    }

    /// perform a single step of evolution
    pub fn step(&mut self) {
        for individual in self.population.iter_mut() {
            individual.score = self.target.score(individual.genes());
        }

        // sort population by fitness
        self.population.sort_by(
            |a,b | a.score.partial_cmp(&b.score).unwrap()
        );
        
        // drop the lower scoring individuals
        let keep_percent = 0.5;
        let current_size = self.population.len();
        let keep = (current_size as f64 * keep_percent) as usize;

        for individual in self.population[keep..].iter_mut() {
            individual.genes.wipe();
        }

        // recreate by randomly matching remaining population and crossing-over
        for idx in keep..current_size {
            let parent1_idx = self.rng.gen_range(0..keep);
            let mut parent2_idx = self.rng.gen_range(0..keep);
            while parent1_idx == parent2_idx { parent2_idx = self.rng.gen_range(0..keep); }

            let parent1 = &self.population[parent1_idx].genes.clone();
            let parent2 = &self.population[parent2_idx].genes.clone();
            *self.population[idx].genes_mut() = self.crossover(parent1, parent2);
        }
    }

    /// the genes of the best scoring individual
    pub fn best(&mut self) -> &Genes {
        let mut best_score: f64 = -1.0;
        let mut best: &Individual = &self.population[0];

        for individual in self.population.iter() {
            let score = self.target.score(&individual.genes);
            if score < best_score { best_score = score; best = &individual};
        }

        return best.genes();
    }

    /// crossover two Genes to create child Genes
    fn crossover(&mut self, p1: &Genes, p2: &Genes) -> Genes {
        //random crossover
        let mut c = p1.clone();

        for idx in 0..self.n {
            match self.rng.gen_bool(0.5) {
                true => if p1.get(idx) == 1 { c.set(idx); } else { c.clear(idx); },
                false => if p2.get(idx) == 1 { c.set(idx); } else { c.clear(idx); },
            }
        }

        for idx in 0..self.n {
            if self.rng.gen_bool(self.mutation_rate) {
                c.flip(idx);
            }
        }

        return c;
    }
}

pub struct OptimizerBuilder<T: Target> {
    size: Option<u32>,
    n: Option<u32>,
    mutation_rate: Option<f64>,
    target: Option<T>,
    target_type: Option<options::TargetType>,
    selection_method: Option<options::SelectionMethod>,
    crossover_method: Option<options::CrossoverMethod>
}

impl <T: Target> Default for OptimizerBuilder<T> {
    fn default() -> Self {
        return OptimizerBuilder {
            size: None,
            n: None,
            mutation_rate: None,
            target: None,
            target_type: None,
            selection_method: None,
            crossover_method: None,
        };
    }
}

impl <T: Target> OptimizerBuilder<T> {
    pub fn new() -> OptimizerBuilder<T> {
        return OptimizerBuilder { ..Default::default() };
    }

    pub fn size(mut self, size: u32) -> OptimizerBuilder<T> {
        self.size = Some(size);
        return self;
    }

    pub fn n(mut self, n: u32) -> OptimizerBuilder<T> {
        self.n = Some(n);
        return self;
    }

    pub fn mutation_rate(mut self, mutation_rate: f64) -> OptimizerBuilder<T> {
        self.mutation_rate = Some(mutation_rate);
        return self;
    }

    pub fn target(mut self, target: T) -> OptimizerBuilder<T> {
        self.target= Some(target);
        return self;
    }

    pub fn target_type(mut self, target_type: options::TargetType) -> OptimizerBuilder<T> {
        self.target_type = Some(target_type);
        return self;
    }

    pub fn selection_method(mut self, selection_method: options::SelectionMethod) -> OptimizerBuilder<T> {
        self.selection_method = Some(selection_method);
        return self;
    }

    pub fn crossover_method(mut self, crossover_method: options::CrossoverMethod) -> OptimizerBuilder<T> {
        self.crossover_method = Some(crossover_method);
        return self;
    }

    pub fn build(self) -> Optimizer<T> {
        return Optimizer::new(
            self.size.unwrap_or(DEFAULT_POP_SIZE),
            self.n.expect("'n': number of genes must be provided"),
            self.mutation_rate.unwrap_or(DEFAULT_MUT_RATE),
            self.target.expect("'target': optimization target must be provided"),
            self.target_type.unwrap_or(DEFAULT_TGT_TYPE),
            self.selection_method.unwrap_or(DEFAULT_SEL_METH),
            self.crossover_method.unwrap_or(DEFAULT_CRS_METH)
        );
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn builder_can_build() {
        use crate::{OptimizerBuilder, Target, options};

        struct S {}

        impl Target for S {
            fn score(&mut self, _: &crate::genes::Genes) -> f64 {
                return 0.0;
            }
        }

        let target = S {};

        let _ = OptimizerBuilder::new()
            .size(100)
            .n(100)
            .mutation_rate(0.1)
            .target(target)
            .target_type(options::TargetType::MAXIMIZE)
            .selection_method(options::SelectionMethod::Weighted)
            .crossover_method(options::CrossoverMethod::BARRIER)
            .build();

        assert!(true);
    }
}
