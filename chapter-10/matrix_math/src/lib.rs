struct Matrix<T> {
    content: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    fn new(rows: i32, cols: i32, content: Vec<T>) -> Matrix<T> {
        let final_content = Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_matrix() {
        let matrix = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let expected = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        assert_eq!(matrix.content, expected);
    }
}
