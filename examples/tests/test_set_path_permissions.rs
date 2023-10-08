use pass_tool::{actions::set_perm, checks::user_is_root, instruction, Playbook};

fn main() {
    // create new user with command: `adduser pass_test`
    // create file: `touch /tmp/test_set_path_permissions_rw_rw_r`
    // run this test
    // check file permissions with command: `ls -l /tmp/test_set_path_permissions_rw_rw_r`
    // output should be:
    // -rw-rw-r-- 1 pass_test pass_test 0 Oct  8 08:08 /tmp/test_set_path_permissions_rw_rw_r
    Playbook::new(
        "test_set_path_permissions",
        "",
        [user_is_root()],
        [instruction(set_perm(
            "/tmp/test_set_path_permissions_rw_rw_r",
            0o664,
            "pass_test",
        ))],
    )
    .apply();
}
