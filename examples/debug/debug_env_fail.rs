use pass_tool::{
    checks::{always_no, always_yes},
    Playbook,
};

pub fn main() {
    let playbook = Playbook::new(
        "env_checks_one_fail",
        "Playbook with environment checks one of which fails",
        [always_yes(), always_no(), always_yes()],
        [],
    );
    assert!(!playbook.apply().ok());
}
