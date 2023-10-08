use pass_tool::{
    actions::{perm, write_file_perm},
    checks::user_is_root,
    instruction, Playbook,
};

fn main() {
    // create new user with command: `adduser pass_test`
    // run this test
    // check file permissions with command: `ls -l /tmp/test_write_file_rw_rw_r`
    // output should be:
    // -rw-rw-r-- 1 pass_test pass_test 3 Oct  8 07:55 /tmp/test_write_file_rw_rw_r
    Playbook::new(
        "test_write_file",
        "",
        [user_is_root()],
        [instruction(write_file_perm(
            "/tmp/test_write_file_rw_rw_r",
            "123",
            perm(0o664, "pass_test"),
        ))],
    )
    .apply();
}
