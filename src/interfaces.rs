pub trait Check {
    /// Short name of [Check]
    fn name(&self) -> &str;
    /// Perform check and return if succeed or not
    fn yes(&self) -> bool;
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
}

impl From<Box<dyn Action>> for Vec<Box<dyn Action>> {
    fn from(value: Box<dyn Action>) -> Self {
        vec![value]
    }
}
