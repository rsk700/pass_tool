use regex::Regex;

use crate::search::{contains, contains_once, find_pattern_iter};

#[derive(Clone)]
pub enum Pattern {
    Bin(Vec<u8>),
    Regex(Regex),
}

impl Pattern {
    pub fn contains_once(&self, data: &[u8]) -> Option<bool> {
        match self {
            Pattern::Bin(b) => Some(contains_once(data, b)),
            Pattern::Regex(r) => {
                let s = String::from_utf8(data.to_vec()).ok()?;
                let mut matches = r.find_iter(&s);
                if let (Some(_), None) = (matches.next(), matches.next()) {
                    Some(true)
                } else {
                    Some(false)
                }
            }
        }
    }

    pub fn contains(&self, data: &[u8]) -> Option<bool> {
        match self {
            Pattern::Bin(b) => Some(contains(data, b)),
            Pattern::Regex(r) => {
                let s = String::from_utf8(data.to_vec()).ok()?;
                Some(r.is_match(&s))
            }
        }
    }

    pub fn replace_once(&self, data: &[u8], replacement: &[u8]) -> Option<Vec<u8>> {
        match self {
            Pattern::Bin(p) => {
                let mut it = find_pattern_iter(data, p);
                if let (Some(start), None) = (it.next(), it.next()) {
                    Some([&data[0..start], replacement, &data[start + p.len()..]].concat())
                } else {
                    None
                }
            }
            Pattern::Regex(r) => {
                let s = String::from_utf8(data.to_vec()).ok()?;
                let mut it = r.find_iter(&s);
                if let (Some(m), None) = (it.next(), it.next()) {
                    Some([&data[0..m.start()], replacement, &data[m.end()..]].concat())
                } else {
                    None
                }
            }
        }
    }
}

pub fn re(re_str: &'static str) -> Pattern {
    Pattern::Regex(Regex::new(re_str).unwrap())
}

impl<T: Into<Vec<u8>>> From<T> for Pattern {
    fn from(value: T) -> Self {
        Pattern::Bin(value.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pattern_replace_once() {
        {
            let p = Pattern::Bin("123".as_bytes().to_vec());
            assert_eq!(
                p.replace_once("111 123 333".as_bytes(), "aaa".as_bytes())
                    .unwrap(),
                "111 aaa 333".as_bytes()
            );
        }
        {
            let p = Pattern::Bin("1".as_bytes().to_vec());
            assert!(p
                .replace_once("111 123 333".as_bytes(), "aaa".as_bytes())
                .is_none());
        }
        {
            let p = Pattern::Bin("".as_bytes().to_vec());
            assert!(p
                .replace_once("111 123 333".as_bytes(), "aaa".as_bytes())
                .is_none());
        }
        {
            let p = re("1.3");
            assert_eq!(
                p.replace_once("111 123 333".as_bytes(), "aaa".as_bytes())
                    .unwrap(),
                "111 aaa 333".as_bytes()
            );
        }
        {
            let p = re(".1");
            assert!(p
                .replace_once("111 123 333".as_bytes(), "aaa".as_bytes())
                .is_none());
        }
        {
            let p = re("");
            assert!(p
                .replace_once("111 123 333".as_bytes(), "aaa".as_bytes())
                .is_none());
        }
    }
}
