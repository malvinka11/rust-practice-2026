pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        "NO".to_string()
    } else if (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_yes() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }
}