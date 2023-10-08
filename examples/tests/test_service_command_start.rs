use pass_tool::{actions::start_service, checks::user_is_root, instruction, Playbook};

fn main() {
    // stop `nginx` service with command `systemctl stop nginx`
    // run test
    // check `nginx` is running with command `systemctl status nginx`
    Playbook::new(
        "test_service_command_start",
        "",
        [user_is_root()],
        [instruction(start_service("nginx"))],
    )
    .apply();
}
