pub fn divisible_sum_pairs(n: i32, k: i32, ar: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 0..n as usize {
        for j in (i + 1)..n as usize {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_sum_pairs() {
        let ar = vec![1, 3, 2, 6, 1, 2];
        assert_eq!(divisible_sum_pairs(6, 3, ar), 5);
    }
}
