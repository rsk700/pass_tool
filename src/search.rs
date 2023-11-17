/// Checks if pattern appears once in data (without overlapping, `true` for "aa"
/// inside "aaa"), will report `false` if pattern is empty
pub fn contains_once<T>(data: &[T], pattern: &[T]) -> bool
where
    T: PartialEq,
{
    let mut it = find_pattern_iter(data, pattern);
    matches!((it.next(), it.next()), (Some(_), None))
}

pub fn contains<T>(data: &[T], pattern: &[T]) -> bool
where
    T: PartialEq,
{
    find_pattern(data, pattern).is_some()
}

pub fn find_pattern<T>(data: &[T], pattern: &[T]) -> Option<usize>
where
    T: PartialEq,
{
    if pattern.is_empty() {
        Some(0)
    } else {
        data.windows(pattern.len()).position(|d| d == pattern)
    }
}

/// Iterator over non overlapping patterns
struct FindPatternIter<'a, T: PartialEq> {
    data: &'a [T],
    pattern: &'a [T],
    cursor: usize,
}

impl<'a, T: PartialEq> FindPatternIter<'a, T> {
    fn new(data: &'a [T], pattern: &'a [T]) -> Self {
        Self {
            data,
            pattern,
            cursor: 0,
        }
    }
}

impl<'a, T: PartialEq> Iterator for FindPatternIter<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index = find_pattern(&self.data[self.cursor..], self.pattern)?;
        index += self.cursor;
        // cursor will always be inside data (cursor <= data.len())
        self.cursor = index + self.pattern.len();
        Some(index)
    }
}

pub fn find_pattern_iter<'a, T>(data: &'a [T], pattern: &'a [T]) -> impl Iterator<Item = usize> + 'a
where
    T: PartialEq,
{
    FindPatternIter::new(data, pattern)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_once() {
        assert!(!contains_once("".as_bytes(), "".as_bytes()));
        assert!(!contains_once("".as_bytes(), "bc".as_bytes()));
        assert!(!contains_once("abcd".as_bytes(), "".as_bytes()));
        assert!(contains_once("abcd".as_bytes(), "cd".as_bytes()));
        assert!(!contains_once("abcdabcd".as_bytes(), "cd".as_bytes()));
        assert!(!contains_once("abc".as_bytes(), "cd".as_bytes()));
        assert!(contains_once("aaa".as_bytes(), "aa".as_bytes()));
        assert!(contains_once("aaa".as_bytes(), "aaa".as_bytes()));
    }

    #[test]
    fn test_find_pattern() {
        assert_eq!(find_pattern("".as_bytes(), "".as_bytes()), Some(0));
        assert_eq!(find_pattern("abc".as_bytes(), "".as_bytes()), Some(0));
        assert_eq!(find_pattern("abc".as_bytes(), "abc".as_bytes()), Some(0));
        assert_eq!(find_pattern("abc".as_bytes(), "d".as_bytes()), None);
        assert_eq!(find_pattern("abbbc".as_bytes(), "bb".as_bytes()), Some(1));
        assert_eq!(find_pattern("abbbcc".as_bytes(), "cc".as_bytes()), Some(4));
        assert_eq!(find_pattern("".as_bytes(), "aaaa".as_bytes()), None);
        assert_eq!(find_pattern("cc".as_bytes(), "aaaa".as_bytes()), None);
    }

    #[test]
    fn test_find_pattern_iter() {
        {
            let mut iter = find_pattern_iter("1".as_bytes(), "1".as_bytes());
            assert_eq!(iter.next().unwrap(), 0);
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }
        {
            let mut iter = find_pattern_iter("abcd 111 22 11 2".as_bytes(), "11".as_bytes());
            assert_eq!(iter.next().unwrap(), 5);
            assert_eq!(iter.next().unwrap(), 12);
            assert_eq!(iter.next(), None);
        }
        {
            let mut iter = find_pattern_iter("abcd 11 22 11 2".as_bytes(), "5".as_bytes());
            assert_eq!(iter.next(), None);
        }
    }
}
