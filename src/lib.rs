use core::f64;
use std::usize;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

mod genes;
mod individual;
mod options;

use genes::*;
use individual::*;
use options::*;

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
    pub fn new(size: u32, n: u32, mutation_rate: f64, target: T) -> Optimizer<T> {
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

// TODO: WIP
struct OptimizerBuilder {
    size: Option<u32>
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
