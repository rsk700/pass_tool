use pass_tool::{actions::enable_service, checks::user_is_root, instruction, Playbook};

fn main() {
    // disable `nginx` service with command `systemctl disable nginx`
    // run test
    // check `nginx` is enabled again with command `systemctl is-enabled nginx`
    Playbook::new(
        "test_service_command_enable",
        "",
        [user_is_root()],
        [instruction(enable_service("nginx"))],
    )
    .apply();
}
