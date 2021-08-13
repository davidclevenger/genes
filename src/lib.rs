use std::usize;
use rand::{thread_rng, Rng};


pub type Genes = Vec<u8>;

pub trait Target {
    fn score(&self, genes: &Genes) -> f32;
}

struct Individual {
    score: f32,
    genes: Genes
}

pub struct Optimizer<T: Target> {
    population: Vec<Individual>,
    n: u32,
    target: T
}

impl Individual {

    /// create a new Individual with all genes
    /// set to zero
    pub fn new(n: u32) -> Individual {
        return Individual {
            score: 0.0,
            genes: Genes::with_capacity((n / 8) as usize)
        };
    }

    /// create a new Individual with genes
    /// set to the genes specified by `genes`
    pub fn new_with_genes(genes: Genes) -> Individual {
        return Individual {
            score: 0.0,
            genes: genes
        };
    }

    /// get a single gene
    #[inline(always)]
    pub fn get(&self, idx: u32) -> u8 {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.genes.len() { 
            return (self.genes[bucket] >> loc) & 1
        } else {
            return 0;
        }
    }

    /// set a single gene to 1
    #[inline(always)]
    pub fn set(&mut self, idx: u32) {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.genes.len() { 
            self.genes[bucket] |= 1 << loc;
        }
    }

    /// set a single gene to 0
    #[inline(always)]
    pub fn unset(&mut self, idx: u32) {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.genes.len() { 
            self.genes[bucket] &= !(1 << loc);
        }
    }

    /// reset all the genes to 0
    #[inline(always)]
    pub fn clear(&mut self) {
        for block in self.genes.iter_mut() {
            *block = 0;
        }
    }

    /// an immutable reference to the genes
    pub fn genes(&self) -> &Genes {
        return &self.genes;
    }
}

impl<T: Target> Optimizer<T>{
    pub fn new(size: u32, n: u32, target: T) -> Optimizer<T> {
        let mut population = Vec::with_capacity(size as usize);

        let mut rng = thread_rng();

        for idx in 0..size {
            let mut genes: Vec<u8> = Vec::with_capacity((n / 8) as usize);

            // random gene initialization
            for gene in genes.iter_mut() {
                *gene = rng.gen()
            }

            population[idx as usize] = Individual::new_with_genes(genes);
        }

        return Optimizer {
            population,
            n,
            target
        };
    }

    pub fn evolve(&mut self, epochs: u32) {
        for _ in 0..epochs {
            self.step()
        }
    }

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
        self.population.truncate(keep);

        // recreate by randomly matching remaining population and crossing-over
        let mut rng = thread_rng();

        for _ in 0..(current_size - keep) {
            let parent1_idx = rng.gen_range(0..keep);
            let mut parent2_idx = rng.gen_range(0..keep);
            while parent1_idx == parent2_idx { parent2_idx = rng.gen_range(0..keep); }

            let parent1 = &self.population[parent1_idx];
            let parent2 = &self.population[parent2_idx];

            let mut child = Individual::new_with_genes(parent1.genes().clone());

            // random crossover
            for idx in 0..self.n {
                if rng.gen_bool(0.5) {
                    if parent1.get(idx) == 1 { child.set(idx); } else { child.unset(idx); }
                } else {
                    if parent2.get(idx) == 1 { child.set(idx); } else { child.unset(idx); }
                }
            }

            self.population.push(child);
        }

        // ensure size has not changed
        assert_eq!(self.population.len(), current_size);
    }

    // print the score of the best individual
    pub fn best(&self) {
        let mut best_score: f32 = -1.0;
        for individual in self.population.iter() {
            let score = self.target.score(individual.genes());
            if score > best_score { best_score = score };
        }

        println!("Best: {}", best_score);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
