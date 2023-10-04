use pass_tool::{
    actions::do_nothing,
    checks::{always_fail, always_ok},
    instruction, Playbook,
};

pub fn main() {
    let playbook = Playbook::new(
        "apply_action",
        "Playbook which applies action",
        [],
        [instruction(do_nothing())
            .confirm(always_fail())
            .with_env(always_ok())],
    );
    playbook.apply().ok();
}
