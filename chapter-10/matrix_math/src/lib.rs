#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct Matrix<T> {
    rows: i8,
    cols: i8,
    values: Vec<T>,
}

impl<T: Clone + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>> Matrix<T> {
    /// Create a new instance of Matrix
    #[allow(dead_code)]
    fn new(rows: i8, cols: i8, values: Vec<T>) -> Result<Matrix<T>, &'static str> {
        // Check if there are more values than there are columns
        if values.len() % cols as usize != 0 {
            return Err(
                "Cannot create matrix: leftover values that can't fill an entire row are present.",
            );
        }
        // Return the final matrix
        Ok(Matrix { rows, cols, values })
    }

    // Check that two matrices have the same dimensions
    fn is_same_dimensions<U>(a: &Matrix<T>, b: &Matrix<U>) -> bool {
        a.rows == b.rows && a.cols == b.cols
    }

    /// Add two matrices 'a' and 'b' together.
    #[allow(dead_code)]
    fn add(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>, &'static str> {
        // If the matrices are of different dimensions, then they cannot be added
        if !Self::is_same_dimensions(&a, &b) {
            return Err("Cannot add matrices: Both matrices are of different dimensions.");
        }

        let mut new_values = Vec::new();

        for (i, a_value) in a.values.iter().enumerate() {
            // Add the corresponding 'a' and 'b' values and push it to the new_values array
            new_values.push(*a_value + b.values[i]);
        }

        Ok(Matrix {
            rows: a.rows,
            cols: a.cols,
            values: new_values,
        })
    }

    /// Subtract two matrices 'a' and 'b'
    #[allow(dead_code)]
    fn subtract(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>, &'static str> {
        // If the matrices are of different dimensions, then they cannot be multiplied
        if !Self::is_same_dimensions(&a, &b) {
            return Err("Cannot subtract matrices: Both matrices are of different dimensions.");
        }

        let mut new_values = Vec::new();

        for (i, a_value) in a.values.iter().enumerate() {
            // Add the corresponding 'a' and 'b' values and push it to the new_values array
            new_values.push(*a_value - b.values[i]);
        }

        Ok(Matrix {
            rows: a.rows,
            cols: a.cols,
            values: new_values,
        })
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // Initializing matrices
    mod init_matrix {
        use crate::Matrix;

        // Check that i32 matrices are supported
        #[test]
        fn init_matrix_i32() {
            let values = vec![1, 2, 3, 4, 5, 6];
            let matrix = Matrix::<i32>::new(2, 3, values.clone()).unwrap();
            assert_eq!(matrix.rows, 2);
            assert_eq!(matrix.cols, 3);
            assert_eq!(matrix.values, *values);
        }

        // Check that f64 matrices are supported
        #[test]
        fn init_matrix_f64() {
            let values = vec![1, 2, 3, 4, 5, 6];
            let matrix = Matrix::<i32>::new(2, 3, values.clone()).unwrap();
            assert_eq!(matrix.rows, 2);
            assert_eq!(matrix.cols, 3);
            assert_eq!(matrix.values, values);
        }

        // Check that invalide matrices throw an error
        #[test]
        fn init_matrix_invalid() {
            let error = Matrix::<f64>::new(2, 3, vec![1.25, 2.25, 3.25, 4.25, 5.25, 6.0, 7.25]);
            assert_eq!(
                error,
                Err(
                    "Cannot create matrix: leftover values that can't fill an entire row are present."
                )
            );

            let error = Matrix::<f64>::new(2, 3, vec![1.25, 2.25, 3.25, 4.25, 5.25]);
            assert_eq!(
                error,
                Err(
                    "Cannot create matrix: leftover values that can't fill an entire row are present."
                )
            );
        }
    }

    mod validate_dimensions {
        use crate::Matrix;

        // Compare two matrices of same dimensions
        #[test]
        fn compare_same_dimensions() {
            let a = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
            let b = Matrix::new(2, 2, vec![5, 3, 2, 1]).unwrap();

            assert!(Matrix::is_same_dimensions(&a, &b))
        }

        // Compare two matrices of same dimensions but different types
        #[test]
        fn compare_same_dimensions_different_types() {
            let a = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
            let b = Matrix::new(2, 2, vec![5.0, 3.0, 2.0, 1.0]).unwrap();

            assert!(Matrix::is_same_dimensions(&a, &b))
        }

        // Compare two matrices of different dimensions and different types
        #[test]
        #[should_panic]
        fn compare_different_dimensions_different_types() {
            let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
            let b = Matrix::new(2, 2, vec![5.0, 3.0, 2.0, 1.0]).unwrap();

            assert!(Matrix::is_same_dimensions(&a, &b))
        }

        // Compare two matrices of different dimensions
        #[test]
        #[should_panic]
        fn compare_different_dimensions() {
            let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
            let b = Matrix::new(2, 2, vec![5, 3, 2, 1]).unwrap();

            assert!(Matrix::is_same_dimensions(&a, &b))
        }
    }

    // Subtracting matrices
    mod add_matrix {
        use crate::Matrix;

        // Add matrices of i32 type
        #[test]
        fn add_matrix_i32() {
            let a = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
            let b = Matrix::new(2, 2, vec![5, 3, 2, 1]).unwrap();
            let c = Matrix::add(a, b).unwrap();

            let expected_values = vec![6, 5, 5, 5];
            assert_eq!(c.values, expected_values);
            assert_eq!(c.rows, 2);
            assert_eq!(c.cols, 2);
        }

        // Add matrices of f64 type
        #[test]
        fn add_matrix_f64() {
            let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
            let b = Matrix::new(2, 2, vec![5.0, 3.0, 2.0, 1.0]).unwrap();
            let c = Matrix::add(a, b).unwrap();

            let expected_values = vec![6.0, 5.0, 5.0, 5.0];
            assert_eq!(c.values, expected_values);
            assert_eq!(c.rows, 2);
            assert_eq!(c.cols, 2);
        }

        // Add matrices of different types
        #[test]
        fn add_matrix_different_types() {}

        // Add matrices of different dimensions (invalid)
        #[test]
        fn add_matrix_invalid() {
            let a = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
            let b = Matrix::new(2, 2, vec![5.0, 3.0, 2.0, 1.0]).unwrap();
            let error = Matrix::add(a, b);

            assert_eq!(
                error,
                Err("Cannot add matrices: Both matrices are of different dimensions.")
            );
        }
    }

     // Subtracting matrices
     mod subtract_matrix {
        use crate::Matrix;

        // Subtract matrices of i32 type
        #[test]
        fn subtract_matrix_i32() {
            let a = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
            let b = Matrix::new(2, 2, vec![5, 3, 2, 1]).unwrap();
            let c = Matrix::subtract(a, b).unwrap();

            let expected_values = vec![-4, -1, 1, 3];
            assert_eq!(c.values, expected_values);
            assert_eq!(c.rows, 2);
            assert_eq!(c.cols, 2);
        }

        // SUbtract matrices of f64 type
        #[test]
        fn subtract_matrix_f64() {
            let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
            let b = Matrix::new(2, 2, vec![5.0, 3.0, 2.0, 1.0]).unwrap();
            let c = Matrix::subtract(a, b).unwrap();

            let expected_values = vec![-4.0, -1.0, 1.0, 3.0];
            assert_eq!(c.values, expected_values);
            assert_eq!(c.rows, 2);
            assert_eq!(c.cols, 2);
        }

        // subtract matrices of different types
        #[test]
        fn subtract_matrix_different_types() {}

        // subtract matrices of different dimensions (invalid)
        #[test]
        fn subtract_matrix_invalid() {
            let a = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
            let b = Matrix::new(2, 2, vec![5.0, 3.0, 2.0, 1.0]).unwrap();
            let error = Matrix::subtract(a, b);

            assert_eq!(
                error,
                Err("Cannot subtract matrices: Both matrices are of different dimensions.")
            );
        }
    }
}
