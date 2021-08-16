extern crate genes;

use genes::{OptimizerBuilder, Target, genes::Genes};

#[derive(Clone)]
struct S {
    actual: u64
}

impl Target for S {
    fn score(&mut self, genes: &Genes) -> f64 {
        //cast genes to u32
        let guess = genes.g32(0);

        return f64::abs(guess as f64 - self.actual as f64)
    }
}

fn main() {
    let target = S { actual: 0xBEEFBEEF };
    let mut opt = OptimizerBuilder::new()
        .size(5)
        .n(64)
        .mutation_rate(0.2)
        .target(target)
        .build();

    for _ in 0..100 {
        opt.step();
        let guess = opt.best().g32(0);
        println!("Actual: {} |  Best guess: {} | % Difference: {:.6}", 
            0xBEEFBEEFu32, 
            guess, 
            (1.0 - (guess as f64 / 0xBEEFBEEFu32 as f64)) * 100.0
        );
    }
}