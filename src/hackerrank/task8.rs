pub fn breaking_records(scores: Vec<i32>) -> (i32, i32) {
    let mut max_score = scores[0];
    let mut min_score = scores[0];

    let mut max_break = 0;
    let mut min_break = 0;

    for &score in &scores[1..] {
        if score > max_score {
            max_score = score;
            max_break += 1;
        }

        if score < min_score {
            min_score = score;
            min_break += 1;
        }
    }

    (max_break, min_break)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records() {
        let result = breaking_records(vec![10, 5, 20, 20, 4, 5, 2, 25, 1]);
        assert_eq!(result, (2, 4));
    }
}
