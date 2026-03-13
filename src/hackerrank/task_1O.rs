pub fn sock_merchant(ar: Vec<i32>) -> i32 {
    let mut counts = [0; 101];

    for sock in ar {
        counts[sock as usize] += 1;
    }

    let mut pairs = 0;

    for count in counts {
        pairs += count / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        let result = sock_merchant(vec![10, 20, 20, 10, 10, 30, 50, 10, 20]);
        assert_eq!(result, 3);
    }
}
