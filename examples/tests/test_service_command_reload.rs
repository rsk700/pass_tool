use pass_tool::{actions::reload_service, checks::user_is_root, instruction, Playbook};

fn main() {
    // run test
    // check logs with command `tail /var/log/syslog`
    // it should say:
    //   Reloaded A high performance web server and a reverse proxy server.
    Playbook::new(
        "test_service_command_reload",
        "",
        [user_is_root()],
        [instruction(reload_service("nginx"))],
    )
    .apply();
}
