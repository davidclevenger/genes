extern crate genes;
use genes::{OptimizerBuilder, Target, genes::Genes};


use std::{env, path::Path};
use image::GenericImageView;


struct ApproxImage {
    actual: image::DynamicImage
}

impl Target for ApproxImage {
    fn score(&mut self, genes: &Genes) -> f64 {
        let (x, y) = self.actual.dimensions();

        let mut error: f64 = 0.0;
        for i in 0..x {
            for j in 0..y {
                // accumulate absolute error of r, g, b, and alpha components    
                let actual_pixel = self.actual.get_pixel(i, j).0;
                
                // pixel location in approx image is row_index * column_width + column_index
                // where each "cell" is *4* u8's wide -- cell components addressed with [0, 3] offset
                error += f64::abs( actual_pixel[0] as f64 - genes.g8((4 * (i * y + j) + 0) as usize) as f64);
                error += f64::abs( actual_pixel[1] as f64 - genes.g8((4 * (i * y + j) + 1) as usize) as f64);
                error += f64::abs( actual_pixel[2] as f64 - genes.g8((4 * (i * y + j) + 2) as usize) as f64);
                error += f64::abs( actual_pixel[3] as f64 - genes.g8((4 * (i * y + j) + 3) as usize) as f64);
            }
        }

        return error;
    }
}

pub fn write_approx(x: u32, y: u32, genes: &Genes) {
    let path= Path::join(
        Path::new(&env::current_dir().unwrap()), "examples/approximatesmile.png"
    );

    let mut imgbuf = image::ImageBuffer::new(x, y);

    for i in 0..x {
        for j in 0..y {
            // accumulate absolute error of r, g, b, and alpha components    
            let pixel = imgbuf.get_pixel_mut(i, j);

            *pixel = image::Rgba([
                genes.g8((4 * (i * y + j) + 0) as usize),
                genes.g8((4 * (i * y + j) + 1) as usize),
                genes.g8((4 * (i * y + j) + 2) as usize),
                genes.g8((4 * (i * y + j) + 3) as usize),
            ]);
        }
    }

    imgbuf.save(path).unwrap();
}


fn main() {
    let path= Path::join(
        Path::new(&env::current_dir().unwrap()), "examples/smile.png"
    );

    let img = image::open(path).unwrap();
    let (rows, columns) = img.dimensions();

    let target = ApproxImage {
        actual: img.clone()
    };

    let mut check = ApproxImage {
        actual: img
    };

    // each pixel has 3 u8 components (24 bits)
    let mut opt = OptimizerBuilder::new()
    .size(100000)
    .n(rows * columns * 32)
    .mutation_rate(0.2)
    .target(target)
    .build();

    for s in 0..20 {
        println!("step: {}", s);
        opt.step();
        println!("delta: {}", check.score(opt.best()))
    }

    let best_genes = opt.best();
    write_approx(rows, columns, best_genes);
}
