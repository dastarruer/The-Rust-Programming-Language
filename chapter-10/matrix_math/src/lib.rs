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
}

#[cfg(test)]
mod tests {
    // use super::*;
    
    mod init_matrix {
        use crate::Matrix;

        #[test]
        fn init_matrix_i32() {
            let values = vec![1, 2, 3, 4, 5, 6];
            let matrix = Matrix::<i32>::new(2, 3, values.clone()).unwrap();
            assert_eq!(matrix.rows, 2);
            assert_eq!(matrix.cols, 3);
            assert_eq!(matrix.values, *values);
        }
    
        #[test]
        fn init_matrix_f64() {
            let values = vec![1, 2, 3, 4, 5, 6];
            let matrix = Matrix::<i32>::new(2, 3, values.clone()).unwrap();
            assert_eq!(matrix.rows, 2);
            assert_eq!(matrix.cols, 3);
            assert_eq!(matrix.values, values);
        }
    
        #[test]
        fn init_matrix_invalid() {
            let error = Matrix::<f64>::new(2, 3, vec![1.25, 2.25, 3.25, 4.25, 5.25, 6.0, 7.25]);
            assert_eq!(error, Err("Number of values exceeds the number of columns available."));
        }
    }
}
