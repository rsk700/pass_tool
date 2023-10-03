use pass_tool::{checks::always_ok, Playbook};

pub fn main() {
    let playbook = Playbook::new(
        "env_checks",
        "Playbook with environment checks",
        [always_ok(), always_ok(), always_ok()],
        [],
    );
    assert!(playbook.apply().ok());
}
