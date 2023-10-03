use pass_tool::{
    actions::do_nothing,
    checks::{always_fail, always_ok},
    instruction, Playbook,
};

pub fn main() {
    let playbook = Playbook::new(
        "confirm_mixed",
        "Playbook with mixed confirmation checks",
        [],
        [instruction(do_nothing()).confirm([always_ok(), always_fail()])],
    );
    playbook.apply().ok();
}
