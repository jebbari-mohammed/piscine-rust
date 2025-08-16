use lalgebra_scalar::Scalar;
use std::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(Vec::new())
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            matrix[i][i] = T::one();
        }
        Matrix(matrix)
    }
}

impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut result = Vec::new();
        for i in 0..self.0.len() {
            result.push(self.0[i][n]); 
        }
        result
    }
}

impl<T: Scalar + Copy + std::ops::Mul<Output = T> + std::ops::Add<Output = T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Self) -> Self::Output {
        if self.0[0].len() != other.0.len() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in 0..other.0[0].len() {
                let mut sum = T::zero();
                for k in 0..self.0[0].len() {
                    sum = sum + (self.0[i][k] * other.0[k][j]);
                }
                row.push(sum);
            }
            result.push(row);
        }

        Some(Matrix(result))
    }
}