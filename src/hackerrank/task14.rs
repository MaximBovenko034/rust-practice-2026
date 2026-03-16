pub fn bon_appetit(bill: Vec<i32>, k: usize, b: i32) -> String {
    let mut sum = 0;

    for (i, price) in bill.iter().enumerate() {
        if i != k {
            sum += price;
        }
    }

    let anna_share = sum / 2;

    if b == anna_share {
        "Bon Appetit".to_string()
    } else {
        (b - anna_share).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(bill, 1, 12), "5");
    }
}
