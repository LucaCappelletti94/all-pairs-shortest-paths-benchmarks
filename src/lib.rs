use geometric_traits::{
    impls::{CSR2D, SymmetricCSR2D},
    prelude::*,
};

pub type Graph = SymmetricCSR2D<CSR2D<usize, usize, usize>>;

pub fn graph_label(g: &Graph) -> String {
    format!("V{}_E{}", g.order(), g.number_of_defined_values() / 2)
}

pub fn sampling_params(vertex_count: usize) -> (usize, u64) {
    if vertex_count >= 500 {
        (10, 120)
    } else if vertex_count >= 200 {
        (10, 60)
    } else if vertex_count >= 50 {
        (30, 20)
    } else {
        (100, 10)
    }
}

pub fn random_weight(seed: u64) -> impl Fn((usize, usize)) -> f64 {
    move |(row, col): (usize, usize)| {
        let h = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add((row as u64).wrapping_mul(2654435761))
            .wrapping_add((col as u64).wrapping_mul(2246822519));
        1.0 + (h % 9) as f64
    }
}
