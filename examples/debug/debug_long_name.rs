use pass_tool::{
    actions::always_ok,
    checks::{always_yes, check},
    instruction, Playbook,
};

pub fn main() {
    let playbook = Playbook::new(
        "long_name",
        "Playbook with long name check",
        [check(
            "Very long name of check aaaaaaaa bbbbbbbbbb dddddddddd aaaaaaaaaaa dddddddddd aaaaaaaa bbbbbbbbbb dddddddddd aaaaaaaaaaa dddddddddd aaaaaaaa bbbbbbbbbb dddddddddd aaaaaaaaaaa dddddddddd aaaaaaaa bbbbbbbbbb dddddddddd aaaaaaaaaaa dddddddddd aaaaaaaa bbbbbbbbbb dddddddddd aaaaaaaaaaa dddddddddd",
            always_yes(),
        )],
        [instruction(always_ok())],
    );
    playbook.apply().ok();
}
