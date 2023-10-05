use pass_tool::{actions::do_nothing, checks::always_no, instruction, Playbook};

pub fn main() {
    let playbook = Playbook::new(
        "confirm_all_no",
        "Playbook with all failed confirmation checks",
        [],
        [instruction(do_nothing()).confirm([always_no(), always_no()])],
    );
    playbook.apply().ok();
}
