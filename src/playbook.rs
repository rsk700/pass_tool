use crate::interfaces::{Action, ActionResult, Check};
use crate::story_formatter::StoryFormatter;

// note: currently using result for convinience instead of ActionResult, allows
// using `?` operator, when it will be possible to overload it, need return
// `ActionResult`, now Ok == ActionResult::Ok, Err == ActionResult::Fail

pub struct Instruction {
    // status of action depending on check
    // b - before action
    // a - after action
    // T - all true
    // f - some false
    // F - all false
    // m - mixed (some true, some false)
    // __________________________
    // | action |__env__|confirm|
    // |________|___b___|_b_|_a_|
    // | fail   |   f   |   |   |
    // | fail   |       |   |   |
    // | fail   |       |   |   |
    // | fail   |       | m |   |
    // | fail   |       |   | f |
    // | ok     |   T   | F | T |
    // | ok     |       | T |   | action skipped in this case
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^
    env_checks: Vec<Box<dyn Check>>,
    confirm_checks: Vec<Box<dyn Check>>,
    action: Box<dyn Action>,
}

impl Instruction {
    pub fn new(action: Box<dyn Action>) -> Self {
        Self {
            env_checks: vec![],
            confirm_checks: vec![],
            action,
        }
    }

    pub fn with_env<Checks>(mut self, env: Checks) -> Self
    where
        Checks: Into<Vec<Box<dyn Check>>>,
    {
        self.env_checks = env.into();
        self
    }

    pub fn confirm<Checks>(mut self, confirm: Checks) -> Self
    where
        Checks: Into<Vec<Box<dyn Check>>>,
    {
        self.confirm_checks = confirm.into();
        self
    }
}

pub fn instruction(action: Box<dyn Action>) -> Instruction {
    Instruction::new(action)
}

// todo: add "instruction on fail/on success/on finish" to Playbook, can be useful for reporting issues
// todo: option to not hide stdout/err output of external processes (show_external_output: bool)
pub struct Playbook {
    // todo: check no spaces, allow only lowercase ascii, allow filename safe symbols?
    /// short [Playbook] name, which can be used as unqiue [Playbook] id
    pub name: &'static str,
    /// description of `Playbook`, explaining its purpose for user
    pub description: &'static str,
    env_checks: Vec<Box<dyn Check>>,
    instructions: Vec<Instruction>,
}

impl Playbook {
    pub fn new<Checks, Instructions>(
        name: &'static str,
        description: &'static str,
        env_checks: Checks,
        instructions: Instructions,
    ) -> Self
    where
        Checks: Into<Vec<Box<dyn Check>>>,
        Instructions: Into<Vec<Instruction>>,
    {
        let name = if name.is_empty() {
            "?without_name?"
        } else {
            name
        };
        let description = if description.is_empty() {
            "?Without description?"
        } else {
            description
        };
        Self {
            name,
            description,
            env_checks: env_checks.into(),
            instructions: instructions.into(),
        }
    }

    fn check_checks(
        story: &StoryFormatter,
        checks: &[Box<dyn Check>],
        ok_is_true: bool,
    ) -> Result<(), ()> {
        let mut ok = true;
        for (i, next_check) in checks.iter().enumerate() {
            let mut check_ok = next_check.yes();
            if !ok_is_true {
                check_ok = !check_ok;
            }
            ok = check_ok && ok;
            story.checklist_item(check_ok, i + 1, next_check.name());
        }
        if ok {
            Ok(())
        } else {
            Err(())
        }
    }

    fn print_check_results(story: &StoryFormatter, checks: &[(&str, bool)]) {
        for (i, (name, yes)) in checks.iter().enumerate() {
            story.checklist_item(*yes, i + 1, name);
        }
    }

    // todo: test cases for each `if`
    pub fn apply(&self) -> ActionResult {
        let mut story = StoryFormatter::new();
        story.playbook_header(self.description);
        let playbook_result: ActionResult = story.section("Playbook", |story| {
            if !self.env_checks.is_empty() {
                story.checklist("Environment", |story| {
                    Self::check_checks(story, &self.env_checks, true)
                })?;
            }
            if !self.instructions.is_empty() {
                story.section("Actions", |story| {
                    for (i, instruction) in self.instructions.iter().enumerate() {
                        let action_name = format!("{}.{}", i + 1, instruction.action.name());
                        story.section(action_name, |story| {
                            // checks before action
                            let mut action_already_applied = false;
                            story.section("pre", |story| {
                                if !instruction.confirm_checks.is_empty() {
                                    let confirm_checks: Vec<(_, _)> = instruction
                                        .confirm_checks
                                        .iter()
                                        .map(|c| (c.name(), c.yes()))
                                        .collect();
                                    let all_confirm_yes =
                                        confirm_checks.iter().all(|(_, yes)| *yes);
                                    let all_confirm_no =
                                        confirm_checks.iter().all(|(_, yes)| !*yes);
                                    story.checklist("Confirmation", |story| {
                                        if !all_confirm_yes && !all_confirm_no {
                                            Self::print_check_results(story, &confirm_checks);
                                            story.checklist_note("confirmation checks should be *all yes* or *all no*");
                                            return Err(());
                                        }
                                        if all_confirm_yes {
                                            Self::print_check_results(story, &confirm_checks);
                                            story
                                                .checklist_note("action already applied, skipping");
                                            action_already_applied = true;
                                        } else if all_confirm_no {
                                            story.checklist_title_note("checking all confirmations is *no*");
                                            Self::print_check_results(story, &confirm_checks);
                                        } else {
                                            unreachable!()
                                        }
                                        Ok(()) // -- end .Confirmation
                                    })?;
                                }
                                if action_already_applied {
                                    return Ok(());
                                }
                                if !instruction.env_checks.is_empty() {
                                    story.checklist("Environment", |story| {
                                        Self::check_checks(story, &instruction.env_checks, true)
                                    })?;
                                }
                                Ok(()) // -- end .pre
                            })?;
                            if action_already_applied {
                                return Ok(());
                            }
                            story.process("apply", |_| {
                                match instruction.action.run() {
                                    ActionResult::Ok => Ok(()),
                                    ActionResult::Fail => Err(()),
                                }
                            })?;
                            // checks after action
                            story.section("post", |story| {
                                if !instruction.confirm_checks.is_empty() {
                                    story.checklist("Confirmation", |story|{
                                        Self::check_checks(story, &instruction.confirm_checks, true)
                                    })
                                } else {
                                    Ok(())
                                }
                            })?;
                            Ok(()) // -- end .Instruction
                        })?;
                    }
                    Ok(()) // -- end .Actions
                })?;
            }
            Ok(()) // -- end .Playbook
        }).into();
        StoryFormatter::playbook_result(self.description, playbook_result.ok());
        playbook_result
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::always_ok,
        checks::{always_no, always_yes},
    };
    use std::cell::RefCell;

    use super::*;

    /// Check which changes result each time it checked
    struct AlwaysFlips {
        result: RefCell<bool>,
    }

    impl AlwaysFlips {
        const NAME: &'static str = "AlwaysFlips";

        pub fn new(next_result: bool) -> Self {
            Self {
                result: RefCell::new(next_result),
            }
        }
    }

    impl Check for AlwaysFlips {
        fn name(&self) -> &str {
            Self::NAME
        }

        fn yes(&self) -> bool {
            let mut result = self.result.try_borrow_mut().unwrap();
            let next_result = *result;
            *result = !*result;
            next_result
        }

        fn into_check(self) -> Box<dyn Check> {
            Box::new(self)
        }
    }

    fn flip(result: bool) -> Box<dyn Check> {
        AlwaysFlips::new(result).into_check()
    }

    #[test]
    fn test_playbook() {
        assert!(!Playbook::new("env-fail", "", [always_no()], [])
            .apply()
            .ok());
        assert!(!Playbook::new(
            "action-env-fail",
            "",
            [],
            [instruction(always_ok()).with_env(always_no())]
        )
        .apply()
        .ok());
        assert!(!Playbook::new(
            "action-confirm-mixed",
            "",
            [],
            [instruction(always_ok()).confirm([always_yes(), always_no()])]
        )
        .apply()
        .ok());
        assert!(!Playbook::new(
            "action-confirm-f-after",
            "",
            [],
            [instruction(always_ok()).confirm([always_no()])]
        )
        .apply()
        .ok());
        assert!(Playbook::new(
            "action-env-t-confirm-f-t",
            "",
            [],
            [instruction(always_ok())
                .with_env([always_yes()])
                .confirm([flip(false)])]
        )
        .apply()
        .ok());
        assert!(Playbook::new(
            "action-confirm-t-before",
            "",
            [],
            [instruction(always_ok()).confirm([always_yes()])]
        )
        .apply()
        .ok());
        assert!(Playbook::new("empty", "", [], []).apply().ok());
        assert!(Playbook::new("env-ok", "", [always_yes()], []).apply().ok());
        assert!(Playbook::new(
            "action-env-ok",
            "",
            [],
            [instruction(always_ok()).with_env([always_yes()])]
        )
        .apply()
        .ok());
        assert!(!Playbook::new(
            "action-env-fail",
            "",
            [],
            [instruction(always_ok()).with_env([always_no()])]
        )
        .apply()
        .ok());
    }
}
