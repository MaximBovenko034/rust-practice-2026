pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut count = 0;

    for x in 1..=100 {
        let mut ok = true;

        for ai in &a {
            if x % ai != 0 {
                ok = false;
            }
        }

        for bi in &b {
            if bi % x != 0 {
                ok = false;
            }
        }

        if ok {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_sets() {
        let result = get_total_x(vec![2, 4], vec![16, 32, 96]);
        assert_eq!(result, 3);
    }
}
