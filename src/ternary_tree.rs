use crate::matrix::Matrix;
use std::collections::HashSet;

type Leaf = Option<Box<TernaryNode>>;

#[derive(Debug, Clone, PartialEq)]
pub struct TernaryNode {
    val:    Matrix,
    left:   Leaf,
    middle: Leaf,
    right:  Leaf,
}

impl TernaryNode {
    pub fn new(matrix: Matrix) -> TernaryNode {
        Self {
            val:    matrix,
            left:   None,
            middle: None,
            right:  None,
        }
    }

    fn as_arr(&self) -> [u32; 3] {
        let data = self.val.data.clone();

        let c = data[2];
        let b = data[1];
        let a = data[0];

        [a as u32, b as u32, c as u32]
    }

    fn branch_out(self) -> Leaf { Some(Box::new(self)) }

    pub fn grow(&mut self, a_val: &Matrix, b_val: &Matrix, c_val: &Matrix) {
        self.left = Self::new(a_val * &self.val).branch_out();
        self.middle = Self::new(b_val * &self.val).branch_out();
        self.right = Self::new(c_val * &self.val).branch_out();
    }

    pub fn cond_grow<F>(&mut self, matrices: [&Matrix; 3], matcher: &F) -> HashSet<[u32; 3]>
    where F: Fn([u32; 3]) -> (Option<[u32; 3]>, bool) {
        let mut set: HashSet<[u32; 3]> = HashSet::new();
        let triplets: [u32; 3] = self.as_arr();
        let (matched_triplets, keep_growing) = matcher(triplets);
        matched_triplets.map(|val| set.insert(val));

        if !keep_growing {
            return set
        }

        self.grow(matrices[0], matrices[1], matrices[2]);

        let mut leaves: [&mut Leaf; 3] = [&mut self.left, &mut self.middle, &mut self.right];

        for leaf in leaves.iter_mut() {
            let leaf_matches = leaf.as_mut().unwrap().cond_grow(matrices, matcher);
            set.extend(leaf_matches);
        }
        set
        // if set.is_empty() { None } else { Some(set) }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::matrix::{BARNING_A, BARNING_B, BARNING_C};

    #[test]
    fn new_insert() {
        let matrix = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
        let root_node = TernaryNode::new(matrix.clone());
        assert_eq!(
            root_node,
            TernaryNode {
                val:    matrix,
                left:   None,
                middle: None,
                right:  None,
            }
        );
    }
    #[test]
    fn grow_leaf() {
        let barning_matrix_a = Matrix::new(3, 3, BARNING_A.to_vec()).unwrap();
        let barning_matrix_b = Matrix::new(3, 3, BARNING_B.to_vec()).unwrap();
        let barning_matrix_c = Matrix::new(3, 3, BARNING_C.to_vec()).unwrap();
        let root_matrix = Matrix::new(3, 1, vec![3, 4, 5]).unwrap();
        let mut root_node = TernaryNode::new(root_matrix.clone());
        root_node.grow(&barning_matrix_a, &barning_matrix_b, &barning_matrix_c);
        assert_eq!(
            root_node,
            TernaryNode {
                val:    root_matrix.clone(),
                left:   TernaryNode::new(Matrix {
                    rows: 3,
                    cols: 1,
                    data: vec![5, 12, 13],
                },)
                .branch_out(),
                middle: TernaryNode::new(Matrix {
                    rows: 3,
                    cols: 1,
                    data: vec![21, 20, 29],
                },)
                .branch_out(),
                right:  TernaryNode::new(Matrix {
                    rows: 3,
                    cols: 1,
                    data: vec![15, 8, 17],
                },)
                .branch_out(),
            }
        );
    }
}
