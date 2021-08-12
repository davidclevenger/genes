use std::usize;

struct Individual {
    genes: Vec<u8>
}

struct Optimizer {
    population: Vec<Individual>
}

impl Individual {

    /// create a new Individual with all genes
    /// set to zero
    pub fn new(n: u32) -> Individual {
        return Individual {
            genes: Vec::with_capacity((n / 8) as usize)
        };
    }

    /// create a new Individual with genes
    /// set to the genes specified by `genes`
    pub fn new_with_genes(genes: Vec<u8>) -> Individual {
        return Individual {
            genes: genes
        };
    }

    /// get the 
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

    pub fn genes(&self) -> &Vec<u8> {
        return &self.genes;
    }
}

impl Optimizer {
    pub fn new(size: u32, n: u32) -> Optimizer {
        let mut population = Vec::with_capacity(size as usize);
        for idx in 0..size {
            population[idx as usize] = Individual::new(n);
        }

        return Optimizer {
            population: population
        };
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
