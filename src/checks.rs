use crate::Check;

/// [Check] which always succeed
#[derive(Clone)]
pub struct AlwaysOk;

impl AlwaysOk {
    const NAME: &'static str = "AlwaysOk";

    pub fn as_action(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

impl Check for AlwaysOk {
    fn name(&self) -> &str {
        Self::NAME
    }
    fn yes(&self) -> bool {
        true
    }
}

pub fn always_ok() -> Box<dyn Check> {
    AlwaysOk.as_action()
}

/// [Check] which always fails
#[derive(Clone)]
pub struct AlwaysFail;

impl AlwaysFail {
    const NAME: &'static str = "AlwaysFail";

    pub fn as_action(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

impl Check for AlwaysFail {
    fn name(&self) -> &str {
        Self::NAME
    }
    fn yes(&self) -> bool {
        false
    }
}

pub fn always_fail() -> Box<dyn Check> {
    AlwaysFail.as_action()
}

/// [Check] which allows to rename another check
pub struct Named {
    name: String,
    check: Box<dyn Check>,
}

impl Named {
    pub fn new(name: String, check: Box<dyn Check>) -> Self {
        Self { name, check }
    }

    pub fn as_action(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

impl Check for Named {
    fn name(&self) -> &str {
        &self.name
    }
    fn yes(&self) -> bool {
        self.check.yes()
    }
}

pub fn named<Name>(name: Name, check: Box<dyn Check>) -> Box<dyn Check>
where
    Name: Into<String>,
{
    Named::new(name.into(), check).as_action()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_checks() {
        assert!(always_ok().yes());
        assert!(!always_fail().yes());
    }

    #[test]
    fn test_named() {
        let c = named("aaa", always_ok());
        assert_eq!(c.name(), "aaa");
        assert!(c.yes());
        let c = named("aaa", always_fail());
        assert!(!c.yes());
    }
}
