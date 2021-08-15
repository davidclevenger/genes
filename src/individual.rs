use crate::genes::*;

pub(crate)struct Individual {
    pub score: f64,
    pub genes: Genes
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