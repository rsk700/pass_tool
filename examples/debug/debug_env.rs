use pass_tool::{checks::always_yes, Playbook};

pub fn main() {
    let playbook = Playbook::new(
        "env_checks",
        "Playbook with environment checks",
        [always_yes(), always_yes(), always_yes()],
        [],
    );
    assert!(playbook.apply().ok());
}
