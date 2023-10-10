use pass_tool::{dgraph::is_applied, Playbook};

fn main() {
    // run before `test_mark_applied`, this test should fail at env checks
    // run `test_mark_applied`
    // run this test, it should succeed
    Playbook::new("test_is_applied", "", [is_applied("test_mark_applied")], []).apply();
}
