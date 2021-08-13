extern crate genes;
use genes::{Optimizer, Target, Genes};

struct S {
    actual: u32
}

impl Target for S {
    fn score(&self, genes: &Genes) -> f32 {
        //cast genes to u32
        let guess = genes[0] as u32;

        return f32::abs(guess as f32 - self.actual as f32)
    }
}

fn main() {
    let target = S { actual: 24 };
    let mut opt = Optimizer::new(100, 32, target);

    for _ in 0..20 {
        opt.step();
        opt.best();
    }
}