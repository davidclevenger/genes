extern crate genes;

use genes::{GeneticOptimizer, Target, Genes};

#[derive(Clone)]
struct S {
    actual: u64
}

impl Target for S {
    fn score(&self, genes: &Genes) -> f64 {
        //cast genes to u32
        let guess = genes.g64(0);

        return f64::abs(guess as f64 - self.actual as f64)
    }
}

fn main() {
    let target = S { actual: std::u64::MAX };
    let mut opt = GeneticOptimizer::new(100, 64, 0.2, target);

    for _ in 0..100 {
        opt.step();
        let guess = opt.best().g64(0);
        println!("Actual: {} |  Best guess: {} | % Difference: {:.6}", 
            std::u64::MAX, 
            guess, 
            ((std::u64::MAX - guess) as f64 / std::u64::MAX as f64) * 100.0
        );
    }
}