pub mod actions;
pub mod checks;
mod cli;
pub mod dgraph;
pub mod dir_context;
pub mod instructions;
pub mod interfaces;
pub mod list_builder;
pub mod pattern;
pub mod playbook;
pub mod process;
pub mod search;
mod story_formatter;

pub use cli::{run_cli, run_cli_with_input};
pub use playbook::{instruction, Playbook};

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
