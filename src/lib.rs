pub mod actions;
pub mod checks;
pub mod interfaces;
pub mod playbook;
pub mod process;
pub mod search;
mod cli;
mod story_formatter;

pub use playbook::{instruction, Playbook};
pub use cli::run_with_cli;

#[cfg(test)]
mod tests {
    use crate::{actions::always_ok, checks::always_yes, playbook::instruction, Playbook};

    #[test]
    fn test_instruction_syntax() {
        instruction(always_ok())
            .with_env(always_yes())
            .confirm([always_yes(), always_yes()]);
    }

    #[test]
    fn test_playbook_syntax() {
        Playbook::new(
            "empty-playbook",
            "Playbook description",
            [always_yes(), always_yes()],
            [
                instruction(always_ok())
                    .with_env(always_yes())
                    .confirm([always_yes(), always_yes()]),
                instruction(always_ok())
                    .with_env(always_yes())
                    .confirm(always_yes()),
            ],
        );
    }
}
