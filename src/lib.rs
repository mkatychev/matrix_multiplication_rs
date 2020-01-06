use crate::{matrix::*, ternary_tree::TernaryNode};
use std::collections::HashSet;

mod matrix;
mod ternary_tree;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let should_grow = |triplets: [u32; 3]| -> (Option<[u32; 3]>, bool) {
        let actual_sum: u32 = triplets.iter().sum();
        println!("{}", actual_sum);
        match actual_sum {
            actual if actual == sum => (Some(triplets), false),
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

//     | 1 2 2 |
// A = | 2 1 2 |
//     | 2 2 3 |

//     |-1 2 2 |
// B = |-2 1 2 |
//     |-2 2 3 |

//     | 1-2 2 |
// C = | 2-1 2 |
//     | 2-2 3 |
// a^2 + b^2 = c^2
// a/k = m^2 - n^2
// b/k = 2mn
// c/k = m^2 - n^2
