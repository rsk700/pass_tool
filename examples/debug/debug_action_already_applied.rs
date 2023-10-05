use pass_tool::{actions::do_nothing, checks::always_yes, instruction, Playbook};

pub fn main() {
    let playbook = Playbook::new(
        "action_already_applied",
        "Playbook with already applied action",
        [],
        [instruction(do_nothing()).confirm(always_yes())],
    );
    assert!(playbook.apply().ok());
}
