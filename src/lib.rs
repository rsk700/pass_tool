pub mod actions;
pub mod checks;
pub mod interfaces;
pub mod playbook;
pub mod process;
pub mod search;
mod story_formatter;

pub use playbook::{instruction, Playbook};

#[cfg(test)]
mod tests {
    use crate::{actions::do_nothing, checks::always_ok, playbook::instruction, Playbook};

    #[test]
    fn test_instruction_syntax() {
        instruction(do_nothing())
            .with_env(always_ok())
            .confirm([always_ok(), always_ok()]);
    }

    #[test]
    fn test_playbook_syntax() {
        Playbook::new(
            "empty-playbook",
            "Playbook description",
            [always_ok(), always_ok()],
            [
                instruction(do_nothing())
                    .with_env(always_ok())
                    .confirm([always_ok(), always_ok()]),
                instruction(do_nothing())
                    .with_env(always_ok())
                    .confirm(always_ok()),
            ],
        );
    }
}
