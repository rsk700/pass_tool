pub fn contains_once<T>(data: &[T], pattern: &[T]) -> bool
where
    T: PartialEq,
{
    let mut matched = false;
    for section in data.windows(pattern.len()) {
        if section == pattern {
            if matched {
                return false;
            }
            matched = true;
        }
    }
    matched
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_once() {
        assert!(contains_once(&[1, 2, 3, 4], &[3, 4]));
        assert!(!contains_once(&[1, 2, 3, 4, 1, 3, 4, 2], &[3, 4]));
        assert!(!contains_once(&[1, 2, 3], &[3, 4]));
        assert!(!contains_once(&[], &[3, 4]));
    }
}
