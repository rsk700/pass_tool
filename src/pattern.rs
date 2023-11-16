use regex::Regex;

use crate::search::{contains, contains_once};

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
}

pub fn re(re_str: &'static str) -> Pattern {
    Pattern::Regex(Regex::new(re_str).unwrap())
}

impl<T: Into<Vec<u8>>> From<T> for Pattern {
    fn from(value: T) -> Self {
        Pattern::Bin(value.into())
    }
}
