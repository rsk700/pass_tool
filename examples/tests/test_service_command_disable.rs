use pass_tool::{actions::disable_service, checks::user_is_root, instruction, Playbook};

fn main() {
    // enable `nginx` service with command `systemctl enable nginx`
    // run test
    // check `nginx` is disabled now with command `systemctl is-enabled nginx`
    Playbook::new(
        "test_service_command_disable",
        "",
        [user_is_root()],
        [instruction(disable_service("nginx"))],
    )
    .apply();
}
