use pass_tool::{
    actions::do_nothing,
    checks::{always_no, always_yes},
    instruction, Playbook,
};

pub fn main() {
    let playbook = Playbook::new(
        "apply_action",
        "Playbook which applies action",
        [],
        [instruction(do_nothing())
            .confirm(always_no())
            .with_env(always_yes())],
    );
    playbook.apply().ok();
}
