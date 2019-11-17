use std::collections::HashSet;

mod matrix;
mod ternary_tree;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32 = 0;
    let mut m: u32 = 2;
    let mut n: u32;
    let mut matching_triplets: HashSet<[u32; 3]> = HashSet::new();
    while c < sum {
        n = 1;
        while n < m {
            a = m.pow(2) - n.pow(2);
            b = 2 * m * n;
            c = m.pow(2) + n.pow(2);
            if a.pow(2) + b.pow(2) == c.pow(2) {
                matching_triplets.insert([a, b, c]);
            }
            n += 1;
        }
        m += 1;
    }
    matching_triplets
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
