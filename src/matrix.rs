use std::ops::Mul;

// https://en.wikipedia.org/wiki/Tree_of_primitive_Pythagorean_triples
//     | 1-2 2 |
// A = | 2-1 2 |
//     | 2-2 3 |
pub static BARNING_A: &[i32] = &[1, -2, 2, 2, -1, 2, 2, -2, 3];

//     | 1 2 2 |
// B = | 2 1 2 |
//     | 2 2 3 |
pub static BARNING_B: &[i32] = &[1, 2, 2, 2, 1, 2, 2, 2, 3];

//     |-1 2 2 |
// C = |-2 1 2 |
//     |-2 2 3 |
pub static BARNING_C: &[i32] = &[-1, 2, 2, -2, 1, 2, -2, 2, 3];

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<i32>,
}
impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<i32>) -> Result<Self, &'static str> {
        let data_len = data.len();
        let matrix = Self { rows, cols, data };
        if (matrix.rows * matrix.cols) != data_len {
            return Err("No good long stuff")
        }
        Ok(matrix)
    }
}

impl Mul for Matrix {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, b_matrix: Self) -> Matrix {
        let a_matrix = self;
        if a_matrix.cols != b_matrix.rows {
            dbg!(a_matrix);
            dbg!(b_matrix);
            panic!(
                "Number of columns in the first matrix should equal to the number of rows in the \
                 second matrix!"
            )
        }
        let new_row = a_matrix.rows;
        let new_col = b_matrix.cols;
        let cell_iter = a_matrix.rows * b_matrix.cols;
        let mut new_data = Vec::with_capacity(cell_iter);
        for i in 0..cell_iter {
            let mut cell_val = 0;
            let a_start = a_matrix.cols * (i / b_matrix.cols);
            let a_stop = a_start + a_matrix.cols;
            let b_start = i % b_matrix.cols;
            for a in a_start..a_stop {
                let step_by = b_matrix.cols * (a % b_matrix.rows);
                let b = b_start + step_by;
                cell_val += (a_matrix.data[a] * b_matrix.data[b]) as i32;
            }
            new_data.push(cell_val);
        }

        Matrix {
            rows: new_row,
            cols: new_col,
            data: new_data,
        }
    }
}

// Multiplying two referenced Matrices should return an unreferenced matrix
impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, other: &'b Matrix) -> Matrix { self.clone() * other.clone() }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_matrix() {
        assert_eq!(
            Matrix::new(6, 2, (0..12).collect()).unwrap(),
            Matrix {
                rows: 6,
                cols: 2,
                data: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].to_vec(),
            }
        );
    }
    #[test]
    fn ii_x_ii_matrix() {
        let a = Matrix::new(2, 3, (0..6).collect()).unwrap();
        let b = Matrix::new(3, 2, (0..6).collect()).unwrap();
        assert_eq!(
            a * b,
            Matrix {
                rows: 2,
                cols: 2,
                data: [10, 13, 28, 40].to_vec(),
            }
        );
    }
    #[test]
    fn ii_x_iii_matrix() {
        let a = Matrix::new(3, 2, vec![2, 4, 6, 4, 7, 3]).unwrap();
        let b = Matrix::new(2, 2, vec![2, 1, 8, 5]).unwrap();
        assert_eq!(
            &a * &b,
            Matrix {
                rows: 3,
                cols: 2,
                data: [36, 22, 44, 26, 38, 22].to_vec(),
            }
        );
        assert_eq!(
            a * b,
            Matrix {
                rows: 3,
                cols: 2,
                data: [36, 22, 44, 26, 38, 22].to_vec(),
            }
        );
    }
    #[test]
    fn iii_x_iv_matrix() {
        let a = Matrix::new(3, 2, vec![1, 3, 2, 4, 2, 5]).unwrap();
        let b = Matrix::new(2, 4, vec![1, 3, 2, 2, 2, 4, 5, 1]).unwrap();
        assert_eq!(
            a * b,
            Matrix {
                rows: 3,
                cols: 4,
                data: [7, 15, 17, 5, 10, 22, 24, 8, 12, 26, 29, 9].to_vec(),
            }
        );
    }
    #[test]
    fn iv_xx_iv_matrix() {
        let a = Matrix::new(4, 4, vec![5, 7, 9, 10, 2, 3, 3, 8, 8, 10, 2, 3, 3, 3, 4, 8]).unwrap();
        let b = Matrix::new(
            4,
            4,
            vec![3, 10, 12, 18, 12, 1, 4, 9, 9, 10, 12, 2, 3, 12, 4, 10],
        )
        .unwrap();
        assert_eq!(
            a * b,
            Matrix {
                rows: 4,
                cols: 4,
                data: vec![
                    210, 267, 236, 271, 93, 149, 104, 149, 171, 146, 172, 268, 105, 169, 128, 169
                ],
            }
        );
    }
}
