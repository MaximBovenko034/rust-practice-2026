pub fn grading_students(grades: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for grade in grades {
        if grade < 38 {
            result.push(grade);
        } else {
            let next_multiple = ((grade / 5) + 1) * 5;
            if next_multiple - grade < 3 {
                result.push(next_multiple);
            } else {
                result.push(grade);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let grades = vec![73, 67, 38, 33];
        let result = grading_students(grades);

        assert_eq!(result, vec![75, 67, 40, 33]);
    }
}
