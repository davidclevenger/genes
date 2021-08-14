use std::usize;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub trait Target {
    fn score(&self, genes: &Genes) -> f32;
}

#[derive(Clone)]
pub struct Genes {
    inner: Vec<u8>
}

struct Individual {
    score: f32,
    genes: Genes
}

/// API entrypoint
pub struct GeneticOptimizer<T: Target> {
    population: Vec<Individual>,
    n: u32,
    target: T,
    rng: SmallRng
}

impl Genes {
    pub fn new(n: u32) -> Genes {
        // zero-initialized genes
        return Genes {
            inner: vec![0u8; (n / 8) as usize]
        };
    }

    pub fn new_with_genes(genes: Vec<u8>) -> Genes {
        return Genes {
            inner: genes
        };
    }

    /// get a single gene
    #[inline(always)]
    pub fn get(&self, idx: u32) -> u8 {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.inner.len() { 
            return (self.inner[bucket] >> loc) & 1
        } else {
            return 0u8;
        }
    }

    /// set a single gene to 1
    #[inline(always)]
    pub fn set(&mut self, idx: u32) {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.inner.len() { 
            self.inner[bucket] |= 1 << loc;
        }
    }

    /// set a single gene to 0
    #[inline(always)]
    pub fn clear(&mut self, idx: u32) {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.inner.len() { 
            self.inner[bucket] &= !(1 << loc);
        }
    }

    /// reset all the genes to 0
    #[inline(always)]
    pub fn wipe(&mut self) {
        for block in self.inner.iter_mut() {
            *block = 0;
        }
    }

    /// convenience method to get the nth 8-bit part of genes
    pub fn g8(&self, loc: usize) -> u8 {
        if loc < self.inner.len() {
            return self.inner[loc];
        } else {
            return 0;
        }
    }
}

impl Individual {
    /// create a new Individual with all genes
    /// set to zero
    pub fn new(n: u32) -> Individual {
        return Individual {
            score: 0.0,
            genes: Genes::new(n)
        };
    }

    /// create a new Individual with genes
    /// set to the genes specified by `genes`
    pub fn new_with_genes(genes: Vec<u8>) -> Individual {
        return Individual {
            score: 0.0,
            genes: Genes::new_with_genes(genes)
        };
    }

    /// immutable reference to the genes
    pub fn genes(&self) -> &Genes {
        return &self.genes;
    }

    /// mutable reference to the genes
    pub fn genes_mut(&mut self) -> &mut Genes {
        return &mut self.genes;
    }
}

impl<T: Target> GeneticOptimizer<T>{
    pub fn new(size: u32, n: u32, target: T) -> GeneticOptimizer<T> {
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
        return GeneticOptimizer {
            population,
            n,
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
    pub fn best(&self) -> &Genes {
        let mut best_score: f32 = -1.0;
        let mut best: &Individual = &self.population[0];

        for individual in self.population.iter() {
            let score = self.target.score(individual.genes());
            if score > best_score { best_score = score; best = &individual};
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

        return c;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
