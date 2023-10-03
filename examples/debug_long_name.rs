use pass_tool::{
    actions::do_nothing,
    checks::{always_ok, named},
    instruction, Playbook,
};

pub fn main() {
    let playbook = Playbook::new(
        "long_name",
        "Playbook with long name check",
        [named(
            "Very long name of check aaaaaaaa bbbbbbbbbb dddddddddd aaaaaaaaaaa dddddddddd aaaaaaaa bbbbbbbbbb dddddddddd aaaaaaaaaaa dddddddddd aaaaaaaa bbbbbbbbbb dddddddddd aaaaaaaaaaa dddddddddd aaaaaaaa bbbbbbbbbb dddddddddd aaaaaaaaaaa dddddddddd aaaaaaaa bbbbbbbbbb dddddddddd aaaaaaaaaaa dddddddddd",
            always_ok(),
        )],
        [instruction(do_nothing())],
    );
    playbook.apply().ok();
}
