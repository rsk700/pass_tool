use std::path::{Path, PathBuf};

use crate::interfaces::{Action, ActionResult, Check};

/// Changes current working directory for action or check (and revert back after
/// it)
pub struct DirContext(PathBuf);

impl DirContext {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    /// wraps check into [DirContextCheck]
    pub fn check(&self, c: Box<dyn Check>) -> Box<dyn Check> {
        DirContextCheck::new(self.0.clone(), c).into_check()
    }

    /// wraps action into [DirContextAction]
    pub fn action(&self, a: Box<dyn Action>) -> Box<dyn Action> {
        DirContextAction::new(self.0.clone(), a).into_action()
    }
}

/// init [DirContext]
pub fn dir<Dir>(path: Dir) -> DirContext
where
    Dir: Into<PathBuf>,
{
    DirContext(path.into())
}

/// Changes current working directory for check (and revert back after it)
pub struct DirContextCheck {
    path: PathBuf,
    check: Box<dyn Check>,
}

impl DirContextCheck {
    const NAME: &str = "DirContext";

    pub fn new(path: PathBuf, check: Box<dyn Check>) -> Self {
        Self { path, check }
    }
}

impl Check for DirContextCheck {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        let Ok(current_dir) = std::env::current_dir() else {
            return false;
        };
        if std::env::set_current_dir(&self.path).is_err() {
            return false;
        }
        let result = self.check.yes();
        if std::env::set_current_dir(current_dir).is_ok() {
            result
        } else {
            false
        }
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// Changes current working directory for action (and revert back after it)
pub struct DirContextAction {
    path: PathBuf,
    action: Box<dyn Action>,
}

impl DirContextAction {
    const NAME: &str = "DirContext";

    pub fn new(path: PathBuf, action: Box<dyn Action>) -> Self {
        Self { path, action }
    }
}

impl Action for DirContextAction {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        let Ok(current_dir) = std::env::current_dir() else {
            return ActionResult::Fail;
        };
        if std::env::set_current_dir(&self.path).is_err() {
            return ActionResult::Fail;
        }
        let result = self.action.run();
        if std::env::set_current_dir(current_dir).is_ok() {
            result
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{actions::always_ok, checks::always_yes};

    #[test]
    fn test_dir_context_check() {
        let dir_path: PathBuf = "/tmp/pass-test-dir-111222333-test_dir_context_check".into();
        std::fs::create_dir(&dir_path).unwrap();
        let current = std::env::current_dir().unwrap();
        let dir_copy = dir_path.clone();
        assert!(dir(&dir_path)
            .check(
                ("test dir context check", move || {
                    std::env::current_dir().unwrap() == dir_copy
                })
                    .into_check()
            )
            .yes());
        assert_ne!(std::env::current_dir().unwrap(), dir_path);
        assert_eq!(std::env::current_dir().unwrap(), current);
        assert!(!dir("/aaaaaaaaaaaaaa/bbbbbbbbbbbbb/11111111111/error-path")
            .check(always_yes())
            .yes());
        std::fs::remove_dir(dir_path).unwrap();
        assert_eq!(std::env::current_dir().unwrap(), current);
    }

    #[test]
    fn test_dir_context_action() {
        let dir_path: PathBuf = "/tmp/pass-test-dir-111222333-test_dir_context_action".into();
        std::fs::create_dir(&dir_path).unwrap();
        let current = std::env::current_dir().unwrap();
        let dir_copy = dir_path.clone();
        assert_eq!(
            dir(&dir_path)
                .action(
                    ("test dir context action", move || {
                        if std::env::current_dir().unwrap() == dir_copy {
                            ActionResult::Ok
                        } else {
                            ActionResult::Fail
                        }
                    })
                        .into_action()
                )
                .run(),
            ActionResult::Ok
        );
        assert_ne!(std::env::current_dir().unwrap(), dir_path);
        assert_eq!(std::env::current_dir().unwrap(), current);
        assert_eq!(
            dir("/aaaaaaaaaaaaaa/bbbbbbbbbbbbb/11111111111/error-path")
                .action(always_ok())
                .run(),
            ActionResult::Fail
        );
        std::fs::remove_dir(dir_path).unwrap();
        assert_eq!(std::env::current_dir().unwrap(), current);
    }
}
