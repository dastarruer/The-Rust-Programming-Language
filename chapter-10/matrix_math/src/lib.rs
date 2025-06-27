struct Matrix<T> {
    content: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    fn new(cols: i32, content: Vec<i32>) -> Matrix<i32> {
        let mut final_content: Vec<Vec<i32>> = Vec::new();

        let mut row:Vec<i32> = Vec::new();
        let mut current_col = 0;
        for (i, value) in content.iter().enumerate() {
            // Execute if we still have some columns left
            if current_col <= cols {  
                // Push the current value to the current row
                row.push(*value);
                
                // Check if the current index has exceeded the numberof columns available
                if (&i + 1) % cols as usize == 0 {
                    // Push the entire current row to the 2D array
                    final_content.push(row.clone());
    
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
            content: final_content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_matrix() {
        let matrix = Matrix::<i32>::new(3, vec![1, 2, 3, 4, 5, 6]);
        let expected = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        assert_eq!(matrix.content, expected);
    }
}
