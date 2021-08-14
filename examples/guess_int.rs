extern crate genes;
use std::borrow::Borrow;

use genes::{Optimizer, Target, Genes};

#[derive(Clone)]
struct S {
    actual: u8
}

impl Target for S {
    fn score(&self, genes: &Genes) -> f32 {
        //cast genes to u32
        let guess = genes.g8(0) as u32;

        return f32::abs(guess as f32 - self.actual as f32)
    }
}

fn main() {
    let target = S { actual: 24 };
    let copy = target.clone();
    let mut opt = Optimizer::new(100, 8, target);

    for _ in 0..20 {
        opt.step();
        println!("Best guess: {}", opt.best().g8(0) as u32);
    }
}