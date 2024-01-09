struct NumMatrix {
    pre_sum: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.is_empty() || matrix[0].is_empty() {
            return NumMatrix {
                pre_sum: vec![vec![0; 1]; 1],
            };
        }

        let (m, n) = (matrix.len(), matrix[0].len());
        let mut pre_sum = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                pre_sum[i + 1][j + 1] = pre_sum[i][j + 1]
                    + pre_sum[i + 1][j]
                    - pre_sum[i][j]
                    + matrix[i][j];
            }
        }

        NumMatrix { pre_sum }
    }

    fn sum_region(&self, row1: usize, col1: usize, row2: usize, col2: usize) -> i32 {
        self.pre_sum[row2 + 1][col2 + 1]
            - self.pre_sum[row1][col2 + 1]
            - self.pre_sum[row2 + 1][col1]
            + self.pre_sum[row1][col1]
    }
}

fn main() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5]
    ];

    let num_matrix = NumMatrix::new(matrix);
    println!("{}", num_matrix.sum_region(2, 1, 4, 3)); // Output: 8
    println!("{}", num_matrix.sum_region(1, 1, 2, 2)); // Output: 11
    println!("{}", num_matrix.sum_region(1, 2, 2, 4)); // Output: 12
}