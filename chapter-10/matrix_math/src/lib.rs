#[allow(dead_code)]
struct Matrix<T> {
    content: Vec<Vec<T>>,
}

impl<T: Clone + Copy> Matrix<T> {
    #[allow(dead_code)]
    fn new(cols: i8, values: Vec<T>) -> Matrix<T> {
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
                // TODO: Return error if number of elements in content exceeds cols
                break
            }
        }
        
        // Silence the compiler for now with dummy data
        Matrix {
            content: final_values,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_matrix_i32() {
        let matrix = Matrix::<i32>::new(3, vec![1, 2, 3, 4, 5, 6]);
        let expected = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        assert_eq!(matrix.content, expected);
    }

    #[test]
    fn init_matrix_f64() {
        let matrix = Matrix::<f64>::new(3, vec![1.25, 2.25, 3.25, 4.25, 5.25, 6.25]);
        let expected = vec![
            vec![1.25, 2.25, 3.25],
            vec![4.25, 5.25, 6.25],
        ];
        assert_eq!(matrix.content, expected);
    }
}
