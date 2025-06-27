struct Matrix<T> {
    content: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    fn new(rows: i32, cols: i32, content: Vec<i32>) -> Matrix<i32> {
        let _final_content: Vec<i32> = Vec::new();

        for (i, value) in content.iter().enumerate() {

        }
        
        // Silence the compiler for now with dummy data
        Matrix {
            content: vec![vec![0, 0, 0], vec![0, 0, 0]],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_matrix() {
        let matrix = Matrix::<i32>::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let expected = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        assert_eq!(matrix.content, expected);
    }
}
