#[derive(Debug)]
#[derive(PartialEq)]
#[allow(dead_code)]
struct Matrix<T> {
    rows: i8,
    cols: i8,
    values: Vec<T>,
}

impl<T: Clone + Copy> Matrix<T> {
    #[allow(dead_code)]
    fn new(rows: i8, cols: i8, values: Vec<T>) -> Result<Matrix<T>, &'static str> {
        // Check if there are more values than there are columns
        if values.len() % cols as usize != 0 {
            return Err("Number of values exceeds the number of columns available.")
        }        
        // Return the final matrix
        Ok(Matrix {
            rows,
            cols,
            values,
        })
    }

    #[allow(dead_code)]
    fn add(a: Matrix<T>, b: Matrix<T>) -> Matrix<i32> {
        Matrix {
            rows: 2,
            cols: 3,
            values: vec![0, 0, 0, 0, 0, 0]
        }
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
            assert_eq!(error, Err("Number of values exceeds the number of columns available."));
        }
    }

    // Adding matrices
    mod add_matrix {
        use crate::Matrix;

        #[test]
        fn add_matrix_i32() {
            let a = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
            let b = Matrix::new(2, 2, vec![5, 3, 2, 1]).unwrap();

            let expected_values = vec![6, 5, 5, 6];
            assert_eq!(Matrix::add(a, b).values, expected_values);
        }
    }
}
