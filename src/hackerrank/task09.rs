pub fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut freq = vec![0; 6];

    for bird in arr {
        freq[bird as usize] += 1;
    }

    let mut best_id = 1;
    let mut best_count = freq[1];

    for i in 2..=5 {
        if freq[i] > best_count {
            best_count = freq[i];
            best_id = i as i32;
        }
    }

    best_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(arr), 4);
    }
}