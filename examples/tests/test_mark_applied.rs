use pass_tool::{
    dgraph::{is_dgraph, mark_applied},
    Playbook, checks::user_is_root,
};

fn main() {
    // run before dgraph init, this test should fail at env check
    // init dgraph
    // run again, check dgraph flag set `ls -l /srv/pass/applied`
    //   -r--r--r-- 1 root root 0 Oct 10 03:45 test_mark_applied
    Playbook::new(
        "test_mark_applied",
        "",
        [is_dgraph(), user_is_root()],
        [mark_applied("test_mark_applied")],
    )
    .apply();
}
