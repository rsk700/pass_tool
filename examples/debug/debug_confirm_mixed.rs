use pass_tool::{
    actions::always_ok,
    checks::{always_no, always_yes},
    instruction, Playbook,
};

pub fn main() {
    let playbook = Playbook::new(
        "confirm_mixed",
        "Playbook with mixed confirmation checks",
        [],
        [instruction(always_ok()).confirm([always_yes(), always_no()])],
    );
    playbook.apply().ok();
}
