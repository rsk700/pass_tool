pub trait Check {
    /// Short name of [Check]
    fn name(&self) -> &str;
    /// Performs check and returns [true] in case of success, [false] - if check
    /// negative or failed (eg. have not enough permission to perform check).
    /// Returned [true] value here is "stronger", for example if file checked
    /// for some content, it possible file contains it, but check returns
    /// [false] if not enough permission. Because of that there is no point in
    /// negating check, for example it is incorrect to use `not(is_dir)` need to
    /// use `path_is_missing`.
    fn yes(&self) -> bool;
    fn into_check(self) -> Box<dyn Check>;
}

impl From<Box<dyn Check>> for Vec<Box<dyn Check>> {
    fn from(value: Box<dyn Check>) -> Self {
        vec![value]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionResult {
    /// Action succeed
    Ok,
    /// Action failed
    Fail,
}

impl ActionResult {
    pub fn ok(&self) -> bool {
        *self == ActionResult::Ok
    }
}

impl From<Result<(), ()>> for ActionResult {
    fn from(value: Result<(), ()>) -> Self {
        match value {
            Ok(_) => ActionResult::Ok,
            Err(_) => ActionResult::Fail,
        }
    }
}

// todo: maybe add Instruction trait, and Action is just Instruction with no checks
pub trait Action {
    /// Short name of [Action]
    fn name(&self) -> &str;
    /// Run action, return status if it succeed or failed
    fn run(&self) -> ActionResult;
    fn into_action(self) -> Box<dyn Action>;
}

impl From<Box<dyn Action>> for Vec<Box<dyn Action>> {
    fn from(value: Box<dyn Action>) -> Self {
        vec![value]
    }
}
