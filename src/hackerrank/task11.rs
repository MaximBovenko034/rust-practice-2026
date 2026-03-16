pub fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len();
    let mut primary = 0;
    let mut secondary = 0;

    for i in 0..n {
        primary += arr[i][i];
        secondary += arr[i][n - i - 1];
    }

    (primary - secondary).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![9, 8, 9]];

        assert_eq!(diagonal_difference(matrix), 2);
    }
}
