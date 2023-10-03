use crate::{Action, ActionResult};

/// [Action] which does nothing and always succeed
#[derive(Clone)]
pub struct DoNothing;

impl DoNothing {
    const NAME: &'static str = "DoNothing";

    pub fn as_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

impl Action for DoNothing {
    fn name(&self) -> &str {
        Self::NAME
    }
    fn run(&self) -> ActionResult {
        ActionResult::Ok
    }
}

pub fn do_nothing() -> Box<dyn Action> {
    DoNothing.as_action()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_actions() {
        {
            let actions = do_nothing();
            matches!(actions.run(), ActionResult::Ok);
        }
    }
}
