/// Checks if pattern appears once in data (without overlapping, `true` for "aa"
/// inside "aaa"), will report `false` if pattern is empty
pub fn contains_once<D, P, T>(data: D, pattern: P) -> bool
where
    D: AsRef<[T]>,
    P: AsRef<[T]>,
    T: PartialEq,
{
    let data = data.as_ref();
    let pattern = pattern.as_ref();
    if let Some(index) = find_pattern(data, pattern) {
        find_pattern(&data[index + pattern.len()..], pattern).is_none()
    } else {
        false
    }
}

pub fn contains<D, P, T>(data: D, pattern: P) -> bool
where
    D: AsRef<[T]>,
    P: AsRef<[T]>,
    T: PartialEq,
{
    let data = data.as_ref();
    let pattern = pattern.as_ref();
    find_pattern(data, pattern).is_some()
}

pub fn find_pattern<Data, Pattern, T>(data: Data, pattern: Pattern) -> Option<usize>
where
    Data: AsRef<[T]>,
    Pattern: AsRef<[T]>,
    T: PartialEq,
{
    let data = data.as_ref();
    let pattern = pattern.as_ref();
    if pattern.len() > data.len() {
        return None;
    }
    (0..=data.len() - pattern.len()).find(|i| &data[*i..*i + pattern.len()] == pattern)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_once() {
        assert!(!contains_once("", ""));
        assert!(!contains_once("", "bc"));
        assert!(!contains_once("abcd", ""));
        assert!(contains_once("abcd", "cd"));
        assert!(!contains_once("abcdabcd", "cd"));
        assert!(!contains_once("abc", "cd"));
        assert!(contains_once("aaa", "aa"));
        assert!(contains_once("aaa", "aaa"));
    }

    #[test]
    fn test_find_pattern() {
        assert_eq!(find_pattern("", ""), Some(0));
        assert_eq!(find_pattern("abc", ""), Some(0));
        assert_eq!(find_pattern("abc", "abc"), Some(0));
        assert_eq!(find_pattern("abc", "d"), None);
        assert_eq!(find_pattern("abbbc", "bb"), Some(1));
        assert_eq!(find_pattern("", "aaaa"), None);
        assert_eq!(find_pattern("cc", "aaaa"), None);
    }
}
