use crate::{matrix::*, ternary_tree::TernaryNode};
use std::collections::HashSet;

mod matrix;
mod ternary_tree;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let should_grow = |triplets: [u32; 3]| -> (Option<[u32; 3]>, bool) {
        let actual_sum: u32 = triplets.iter().sum();
        match actual_sum {
            actual if sum % actual == 0 => {
                let factor = sum / actual;
                // If the factor is greater than 1, then there are more possible solution
                // branches
                let keep_growing = if factor == 1 { false } else { true };
                // barning matrix multiplication has no expectation of being a sorted result
                let mut sorted = triplets.clone();
                sorted.sort();
                (
                    // multiply by found factor
                    Some([sorted[0] * factor, sorted[1] * factor, sorted[2] * factor]),
                    keep_growing,
                )
            },
            actual if actual > sum => (None, false),
            _ => (None, true),
        }
    };
    let mut root_node = TernaryNode::new(Matrix::new(3, 1, vec![3, 4, 5]).unwrap());

    // Barning matrices
    let a = Matrix::new(3, 3, BARNING_A.to_vec()).unwrap();
    let b = Matrix::new(3, 3, BARNING_B.to_vec()).unwrap();
    let c = Matrix::new(3, 3, BARNING_C.to_vec()).unwrap();
    let abc: [&Matrix; 3] = [&a, &b, &c];

    root_node.cond_grow(abc, &should_grow)
}
