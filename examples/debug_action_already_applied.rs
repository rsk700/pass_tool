use pass_tool::{actions::do_nothing, checks::always_ok, instruction, Playbook};

pub fn main() {
    let playbook = Playbook::new(
        "action_already_applied",
        "Playbook with already applied action",
        [],
        [instruction(do_nothing()).confirm(always_ok())],
    );
    assert!(playbook.apply().ok());
}
