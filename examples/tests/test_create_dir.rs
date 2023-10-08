use pass_tool::{
    actions::{create_dir_perm, perm},
    checks::user_is_root,
    instruction, Playbook,
};

fn main() {
    // create new user with command: `adduser pass_test`
    // run this test
    // check dir permissions with command: `ls -ld /tmp/test_create_dir_rw_rw_r`
    // output should be:
    // drw-rw-r-- 2 pass_test pass_test 4096 Oct  8 08:01 /tmp/test_create_dir_rw_rw_r/
    Playbook::new(
        "test_create_dir",
        "",
        [user_is_root()],
        [instruction(create_dir_perm(
            "/tmp/test_create_dir_rw_rw_r",
            perm(0o664, "pass_test"),
        ))],
    )
    .apply();
}
