use pass_tool::{
    checks::{always_fail, always_ok},
    Playbook,
};

pub fn main() {
    let playbook = Playbook::new(
        "env_checks_one_fail",
        "Playbook with environment checks one of which fails",
        [always_ok(), always_fail(), always_ok()],
        [],
    );
    assert!(!playbook.apply().ok());
}
