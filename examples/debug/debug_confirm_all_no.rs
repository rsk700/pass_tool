use pass_tool::{actions::always_ok, checks::always_no, instruction, Playbook};

pub fn main() {
    let playbook = Playbook::new(
        "confirm_all_no",
        "Playbook with all failed confirmation checks",
        [],
        [instruction(always_ok()).confirm([always_no(), always_no()])],
    );
    playbook.apply().ok();
}
