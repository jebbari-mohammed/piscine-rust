use std::ops::{Add, Sub};
use lalgebra_scalar::Scalar;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]]) 
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut mat = vec![vec![T::zero(); n]; n]; 
        let mut i = 0;
        while i < n {
            mat[i][i] = T::one();  
            i += 1;
        }
        Matrix(mat)
    }
}

impl<T> Add for Matrix<T> 
where 
    T: Add<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Self::Output {
        // Check if both matrices have same number of rows
        if self.0.len() != other.0.len() {
            return None;
        }
        
        // Check if matrices are empty
        if self.0.len() == 0 || other.0.len() == 0 {
            return None;
        }
        
        // Check if both matrices have same number of columns
        if self.0[0].len() != other.0[0].len() {
            return None;
        }
        
        // Make sure all rows have the same length in first matrix
        let mut i = 0;
        while i < self.0.len() {
            if self.0[i].len() != self.0[0].len() {
                return None;
            }
            i += 1;
        }
        
        // Make sure all rows have the same length in second matrix
        i = 0;
        while i < other.0.len() {
            if other.0[i].len() != other.0[0].len() {
                return None;
            }
            i += 1;
        }

        // Create result matrix
        let mut result = Vec::new();
        
        // Add matrices element by element
        i = 0;
        while i < self.0.len() {
            let mut result_row = Vec::new();
            let mut j = 0;
            while j < self.0[i].len() {
                let sum = self.0[i][j].clone() + other.0[i][j].clone();
                result_row.push(sum);
                j += 1;
            }
            result.push(result_row);
            i += 1;
        }
        
        Some(Matrix(result))
    }
}

impl<T> Sub for Matrix<T> 
where 
    T: Sub<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Self::Output {
        // Check if both matrices have same number of rows
        if self.0.len() != other.0.len() {
            return None;
        }
        
        // Check if matrices are empty
        if self.0.len() == 0 || other.0.len() == 0 {
            return None;
        }
        
        // Check if both matrices have same number of columns
        if self.0[0].len() != other.0[0].len() {
            return None;
        }
        
        // Make sure all rows have the same length in first matrix
        let mut i = 0;
        while i < self.0.len() {
            if self.0[i].len() != self.0[0].len() {
                return None;
            }
            i += 1;
        }
        
        // Make sure all rows have the same length in second matrix
        i = 0;
        while i < other.0.len() {
            if other.0[i].len() != other.0[0].len() {
                return None;
            }
            i += 1;
        }

        // Create result matrix
        let mut result = Vec::new();
        
        // Subtract matrices element by element
        i = 0;
        while i < self.0.len() {
            let mut result_row = Vec::new();
            let mut j = 0;
            while j < self.0[i].len() {
                let diff = self.0[i][j].clone() - other.0[i][j].clone();
                result_row.push(diff);
                j += 1;
            }
            result.push(result_row);
            i += 1;
        }
        
        Some(Matrix(result))
    }
}