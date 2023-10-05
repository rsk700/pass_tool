use pass_tool::{
    actions::do_nothing,
    checks::{always_no, always_yes},
    instruction, Playbook,
};

pub fn main() {
    let playbook = Playbook::new(
        "confirm_mixed",
        "Playbook with mixed confirmation checks",
        [],
        [instruction(do_nothing()).confirm([always_yes(), always_no()])],
    );
    playbook.apply().ok();
}
