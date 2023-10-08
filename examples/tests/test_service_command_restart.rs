use pass_tool::{actions::restart_service, checks::user_is_root, instruction, Playbook};

fn main() {
    // check `nginx` uptime with command `systemctl status nginx`
    // run test
    // check `nginx` uptime (it should be reset) with command `systemctl status nginx`
    Playbook::new(
        "test_service_command_restart",
        "",
        [user_is_root()],
        [instruction(restart_service("nginx"))],
    )
    .apply();
}
