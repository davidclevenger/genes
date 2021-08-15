use core::f64;
use std::usize;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub trait Target {
    fn score(&mut self, genes: &Genes) -> f64;
}

#[derive(Clone)]
pub struct Genes {
    inner: Vec<u8>
}

struct Individual {
    score: f64,
    genes: Genes
}

/// API entrypoint
pub struct GeneticOptimizer<T: Target> {
    population: Vec<Individual>,
    n: u32,
    mutation_rate: f64,
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

    #[inline(always)]
    pub fn flip(&mut self, idx: u32) {
        let bucket = (idx / 8) as usize;
        let loc = idx & 0b0000_0111;
        if bucket < self.inner.len() { 
            self.inner[bucket] ^= 1 << loc;
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

    /// convenience method to get the nth 16-bit part of genes
    pub fn g16(&self, loc: usize) -> u16 {
        let adj_factor = 2;

        let loc = loc * adj_factor; // the Nth 16-bit slice is begins at 2N

        if loc < self.inner.len() - (adj_factor - 1) {
            return 0u16 
                | ((self.inner[loc] as u16) << 8)
                | ((self.inner[loc + 1] as u16));
        } else {
            return 0;
        }
    }

    /// convenience method to get the nth 32-bit part of genes
    pub fn g32(&self, loc: usize) -> u32 {
        let adj_factor = 4; 

        let loc = loc * adj_factor; // the Nth 32-bit slice is begins at 4N

        if loc < self.inner.len() - (adj_factor - 1) {
            return 0u32
                | ((self.inner[loc] as u32) << 24)
                | ((self.inner[loc + 1] as u32) << 16)
                | ((self.inner[loc + 2] as u32) << 8)
                | ((self.inner[loc + 3] as u32));
        } else {
            return 0;
        }
    }

    /// convenience method to get the nth 64-bit part of genes
    pub fn g64(&self, loc: usize) -> u64 {
        let adj_factor = 8; 

        let loc = loc * adj_factor; // the Nth 64-bit slice is begins at 8N

        if loc < self.inner.len() - (adj_factor - 1) {
            return 0u64
                | ((self.inner[loc] as u64) << 56)
                | ((self.inner[loc + 1] as u64) << 48)
                | ((self.inner[loc + 2] as u64) << 40)
                | ((self.inner[loc + 3] as u64) << 32)
                | ((self.inner[loc + 4] as u64) << 24)
                | ((self.inner[loc + 5] as u64) << 16)
                | ((self.inner[loc + 6] as u64) << 8)
                | ((self.inner[loc + 7] as u64));
        } else {
            return 0;
        }
    }

    /// convenience method to get the nth 128-bit part of genes
    pub fn g128(&self, loc: usize) -> u128 {
        let adj_factor = 16; 

        let loc = loc * adj_factor; // the Nth 128-bit slice is begins at 16N

        if loc < self.inner.len() - (adj_factor - 1) {
            return 0u128
                | ((self.inner[loc] as u128) << 120)
                | ((self.inner[loc + 1] as u128) << 112)
                | ((self.inner[loc + 2] as u128) << 104)
                | ((self.inner[loc + 3] as u128) << 96)
                | ((self.inner[loc + 4] as u128) << 88)
                | ((self.inner[loc + 5] as u128) << 80)
                | ((self.inner[loc + 6] as u128) << 72)
                | ((self.inner[loc + 7] as u128) << 64)
                | ((self.inner[loc + 8] as u128) << 56)
                | ((self.inner[loc + 9] as u128) << 48)
                | ((self.inner[loc + 10] as u128) << 40)
                | ((self.inner[loc + 11] as u128) << 32)
                | ((self.inner[loc + 12] as u128) << 24)
                | ((self.inner[loc + 13] as u128) << 16)
                | ((self.inner[loc + 14] as u128) << 8)
                | ((self.inner[loc + 15] as u128));
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
    pub fn new(size: u32, n: u32, mutation_rate: f64, target: T) -> GeneticOptimizer<T> {
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
            let score = self.target.score(individual.genes());
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
