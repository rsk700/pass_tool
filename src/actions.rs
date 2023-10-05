use crate::interfaces::{Action, ActionResult};

/// [Action] which does nothing and always succeed
pub struct DoNothing;

impl DoNothing {
    const NAME: &'static str = "DoNothing";
}

impl Action for DoNothing {
    fn name(&self) -> &str {
        Self::NAME
    }
    fn run(&self) -> ActionResult {
        ActionResult::Ok
    }
    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

pub fn do_nothing() -> Box<dyn Action> {
    DoNothing.into_action()
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
