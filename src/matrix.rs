extern crate ifmt;
use self::ifmt::iprintln;
use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<u32>,
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

impl Mul for Matrix {
    type Output = Self;

    //     | 0 3 5 |
    // A = | 5 5 2 |

    //     | 3 4 |
    // B = | 3-2 |
    //     |-2 2 |

    fn mul(self, other: Self) -> Self {
        // "Number of columns in the first matrix should equal to the number of rows in
        // the second matrix",
        if self.cols != other.rows {
            return Matrix {
                rows: 0,
                cols: 0,
                data: vec![],
            }
        }
        let new_row = self.rows;
        let new_col = other.cols;
        let cell_iter = self.rows * other.cols;
        let mut new_data = Vec::with_capacity(cell_iter);
        for i in 0..cell_iter {
            let mut cell_val = 0;
            let a_start = self.cols * (i / other.cols);
            let a_stop = a_start + self.cols;
            let b_start = i % other.cols;
            let b_stop = b_start + other.rows;
            for (a, b) in (a_start..a_stop).zip(b_start..b_stop) {
                let step_by = other.cols * (a % other.rows);
                let b_index = b_start + step_by;
                // iprintln!("b_start:{b_start}, b_stop:{b_stop}, step_by:{step_by}");
                cell_val += (self.data[a] * other.data[b_index]) as u32;
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

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}
impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<u32>) -> Result<Self, &'static str> {
        let data_len = data.len();
        let matrix = Self {
            rows: rows,
            cols: cols,
            data: data,
        };
        if (matrix.rows * matrix.cols) != data_len {
            return Err("No good long stuff")
        }
        Ok(matrix)
    }
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
    fn mul_matrix() {
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
    fn ii_iii_matrix() {
        let a = Matrix::new(3, 2, vec![2, 4, 6, 4, 7, 3]).unwrap();
        let b = Matrix::new(2, 2, vec![2, 1, 8, 5]).unwrap();
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
    fn iii_iv_matrix() {
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
    fn iv_iv_matrix() {
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
                data: [
                    210, 267, 236, 271, 93, 149, 104, 149, 171, 146, 172, 268, 105, 169, 128, 169
                ]
                .to_vec(),
            }
        );
    }
}
//     fn 3by3_x_1() {
// }
