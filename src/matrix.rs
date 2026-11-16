use crate::tuple::Tuple;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];
        Matrix { data, rows, cols }
    }

    pub fn from_vec(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Matrix { data, rows, cols }
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = Matrix::new(size, size);
        for i in 0..size {
            matrix.data[i][i] = 1.0;
        }
        matrix
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row][col] = value;
    }

    pub fn equal(&self, other: &Matrix, tol: Option<f64>) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                if (self.data[i][j] - other.data[i][j]).abs() > tol.unwrap_or(f64::EPSILON) {
                    return false;
                }
            }
        }
        true
    }

    fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut submatrix = Matrix::new(self.rows - 1, self.cols - 1);
        for i in 0..self.rows {
            if i == row {
                continue;
            }
            for j in 0..self.cols {
                if j == col {
                    continue;
                }
                let sub_i = if i < row { i } else { i - 1 };
                let sub_j = if j < col { j } else { j - 1 };
                submatrix.data[sub_i][sub_j] = self.data[i][j];
            }
        }
        submatrix
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        let submatrix = self.submatrix(row, col);
        return submatrix.determinant();
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 { minor } else { -minor }
    }

    pub fn transpose(&self) -> Self {
        let mut transposed = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                transposed.data[j][i] = self.data[i][j];
            }
        }
        transposed
    }

    fn determinant(&self) -> f64 {
        assert_eq!(self.rows, self.cols);
        if self.rows == 2 {
            return self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
        }
        let mut det = 0.0;
        for col in 0..self.cols {
            let submatrix = self.submatrix(0, col);
            det +=
                self.data[0][col] * submatrix.determinant() * if col % 2 == 0 { 1.0 } else { -1.0 };
        }
        det
    }

    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        assert!(det != 0.0, "Matrix is not invertible");
        let mut inverse = Matrix::new(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cofactor = self.cofactor(row, col);
                // [col][row] over [row][col] because we want the transpose of the cofactor matrix
                inverse.data[col][row] = cofactor / det;
            }
        }
        inverse
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Matrix {
        &self * &other
    }
}

impl Mul<Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Tuple {
        assert_eq!(self.cols, 4);
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        let tuple_vec = vec![tuple.x, tuple.y, tuple.z, tuple.w];
        for i in 0..self.rows {
            let mut sum = 0.0;
            for j in 0..self.cols {
                sum += self.data[i][j] * tuple_vec[j];
            }
            match i {
                0 => result.x = sum,
                1 => result.y = sum,
                2 => result.z = sum,
                3 => result.w = sum,
                _ => {}
            }
        }
        result
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Tuple {
        &self * tuple
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let data = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ];
        let matrix = Matrix::from_vec(data);
        assert_eq!(matrix.rows, 4);
        assert_eq!(matrix.cols, 4);
        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(0, 3), 4.0);
        assert_eq!(matrix.get(1, 0), 5.5);
        assert_eq!(matrix.get(1, 2), 7.5);
        assert_eq!(matrix.get(2, 2), 11.0);
        assert_eq!(matrix.get(3, 0), 13.5);
        assert_eq!(matrix.get(3, 2), 15.5);
    }

    #[test]
    fn test_identity_matrix() {
        let identity = Matrix::identity(3);
        assert_eq!(identity.get(0, 0), 1.0);
        assert_eq!(identity.get(0, 1), 0.0);
        assert_eq!(identity.get(0, 2), 0.0);
        assert_eq!(identity.get(1, 0), 0.0);
        assert_eq!(identity.get(1, 1), 1.0);
        assert_eq!(identity.get(1, 2), 0.0);
        assert_eq!(identity.get(2, 0), 0.0);
        assert_eq!(identity.get(2, 1), 0.0);
        assert_eq!(identity.get(2, 2), 1.0);
    }

    #[test]
    fn test_matrix_creation() {
        let matrix = Matrix::new(2, 3);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 3);
        assert_eq!(matrix.get(0, 0), 0.0);
        assert_eq!(matrix.get(1, 2), 0.0);
    }

    #[test]
    fn test_equality() {
        let m1 = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let m2 = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let m3 = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn test_matrix_multiplication() {
        let m1: Matrix = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix = Matrix::from_vec(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);

        let expected: Matrix = Matrix::from_vec(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);
        let result = m1 * m2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_tuple_multiplication() {
        let m: Matrix = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let t = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let expected = Tuple::new(18.0, 24.0, 33.0, 1.0);
        let result = m * t;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_multiply_by_identity() {
        let m: Matrix = Matrix::from_vec(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);
        let i = Matrix::identity(4);
        let expected = m.clone();
        let result = m * i;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_matrix_transpose() {
        let m: Matrix = Matrix::from_vec(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        let expected: Matrix = Matrix::from_vec(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);
        let result = m.transpose();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_determinant_2x2() {
        let m: Matrix = Matrix::from_vec(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        let result = m.determinant();
        assert_eq!(result, 17.0);
    }

    #[test]
    fn test_determinant_3x3() {
        let m: Matrix = Matrix::from_vec(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);
        let result = m.determinant();
        assert_eq!(result, -196.0);
    }

    #[test]
    fn test_determinant_4x4() {
        let m: Matrix = Matrix::from_vec(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);
        let result = m.determinant();
        assert_eq!(result, -4071.0);
    }

    #[test]
    fn test_submatrix_3_3() {
        let m: Matrix = Matrix::from_vec(vec![
            vec![1.0, 5.0, 0.0],
            vec![3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);
        let expected: Matrix = Matrix::from_vec(vec![vec![3.0, 2.0], vec![0.0, 6.0]]);
        let result = m.submatrix(0, 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_submatrix_4_4() {
        // Input M
        // -6.0  1.0  1.0  6.0
        // -8.0  5.0  8.0  6.0
        //  1.0  8.0  5.0 -6.0
        //  7.0 -6.0 -7.0  1.0
        // Submatrix of 2, 1 removes row 2 and column 1, resulting in
        // -6.0  xxx  1.0  6.0
        // -8.0  xxx  8.0  6.0
        //  xxx  xxx  xxx  xxx
        //  7.0  xxx -7.0  1.0
        let m: Matrix = Matrix::from_vec(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![1.0, 8.0, 5.0, -6.0],
            vec![7.0, -6.0, -7.0, 1.0],
        ]);
        let expected: Matrix = Matrix::from_vec(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![7.0, -7.0, 1.0],
        ]);
        let result = m.submatrix(2, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_minor_3_3() {
        let m: Matrix = Matrix::from_vec(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        // The submatrix of M[1,0] is [[5, 0], [-1, 5]] and its determinant is 25
        let expected = 25.0;
        let result = m.minor(1, 0);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_cofactor() {
        let m: Matrix = Matrix::from_vec(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        // The minor of M[0,0] is -12, so the cofactor is -12
        let expected = -12.0;
        let result = m.cofactor(0, 0);
        assert_eq!(result, expected);

        // The minor of M[1,0] is 25, so the cofactor is -25
        let expected = -25.0;
        let result = m.cofactor(1, 0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_inverse() {
        let m: Matrix = Matrix::from_vec(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ]);

        let expected: Matrix = Matrix::from_vec(vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        let result = m.inverse();
        assert!(result.equal(&expected, Some(0.00001)));
    }

    #[test]
    fn test_inverse_multiplication() {
        // PROOF: if you multiply some matrix A by another matrix B
        // producing C, you can multiply C by the inverse of B to get A again.
        let a: Matrix = Matrix::from_vec(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ]);
        let b: Matrix = Matrix::from_vec(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a.clone() * b.clone();
        let b_inv = b.inverse();
        let result = c * b_inv;
        assert!(result.equal(&a, Some(0.00001)));
    }
}
