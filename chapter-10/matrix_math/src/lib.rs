#[derive(Debug)]
#[derive(PartialEq)]
#[allow(dead_code)]
struct Matrix<T> {
    content: Vec<Vec<T>>,
}

impl<T: Clone + Copy> Matrix<T> {
    #[allow(dead_code)]
    fn new<'a>(cols: i8, values: Vec<T>) -> Result<Matrix<T>, &'static str> {
        // Check if there are more values than there are columns
        if values.len() % cols as usize != 0 {
            return Err("Number of values exceeds the number of columns available.")
        }

        // Stores the 2D array which will hold all the values in the matrix
        let mut final_values: Vec<Vec<T>> = Vec::new();

        // Stores the current row
        let mut row:Vec<T> = Vec::new();

        // Keep track of what the current column is
        let mut current_col = 0;
        for (i, value) in values.iter().enumerate() {
            // Execute if we still have some columns left
            if current_col <= cols {  
                // Push the current value to the current row
                row.push(*value);

                // Check if the current index has exceeded the numberof columns available
                if (&i + 1) % cols as usize == 0 {
                    // Push the entire current row to the 2D array
                    final_values.push(row.clone());
    
                    // Empty current row to make room for next row
                    row = Vec::new();
    
                    // Since we're on a different column, increment this
                    current_col += 1;
                } 
            } else {
                // Since we've run out of columns, we can simply exit the loop
                break
            }
        }
        
        // Return the final matrix
        Ok(Matrix {
            content: final_values,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_matrix_i32() {
        let matrix = Matrix::<i32>::new(3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let expected = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        assert_eq!(matrix.content, expected);
    }

    #[test]
    fn init_matrix_f64() {
        let matrix = Matrix::<f64>::new(3, vec![1.25, 2.25, 3.25, 4.25, 5.25, 6.25]).unwrap();
        let expected = vec![
            vec![1.25, 2.25, 3.25],
            vec![4.25, 5.25, 6.25],
        ];
        assert_eq!(matrix.content, expected);
    }

    #[test]
    fn init_matrix_invalid() {
        let error = Matrix::<f64>::new(3, vec![1.25, 2.25, 3.25, 4.25, 5.25, 6.0, 7.25]);
        assert_eq!(error, Err("Number of values exceeds the number of columns available."));
    }
}
