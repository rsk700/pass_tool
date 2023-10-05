use pass_tool::{
    actions::always_ok,
    checks::{always_no, always_yes},
    instruction, Playbook,
};

pub fn main() {
    let playbook = Playbook::new(
        "apply_action",
        "Playbook which applies action",
        [],
        [instruction(always_ok())
            .confirm(always_no())
            .with_env(always_yes())],
    );
    playbook.apply().ok();
}
