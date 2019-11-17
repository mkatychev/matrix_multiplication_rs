use crate::matrix::Matrix;

type Leaf = Option<Box<TernaryNode>>;

#[derive(Debug, PartialEq)]
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

    pub fn branch_out(self) -> Leaf { Some(Box::new(self)) }

    pub fn grow(&mut self, a_val: &Matrix, b_val: &Matrix, c_val: &Matrix) {
        self.left = Self::new(&self.val * a_val).branch_out();
        self.middle = Self::new(&self.val * b_val).branch_out();
        self.right = Self::new(&self.val * c_val).branch_out();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
    fn grow_leaf() {
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
}
