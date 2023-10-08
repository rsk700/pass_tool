use pass_tool::{actions::stop_service, checks::user_is_root, instruction, Playbook};

fn main() {
    // start `nginx` service with command `systemctl start nginx`
    // run test
    // check `nginx` is stopped with command `systemctl status nginx`
    Playbook::new(
        "test_service_command_stop",
        "",
        [user_is_root()],
        [instruction(stop_service("nginx"))],
    )
    .apply();
}
