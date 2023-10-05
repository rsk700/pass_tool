use pass_tool::{actions::always_ok, checks::always_yes, instruction, Playbook};

pub fn main() {
    let playbook = Playbook::new(
        "action_already_applied",
        "Playbook with already applied action",
        [],
        [instruction(always_ok()).confirm(always_yes())],
    );
    assert!(playbook.apply().ok());
}
