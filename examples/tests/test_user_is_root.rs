use pass_tool::{checks::user_is_root, Playbook};

fn main() {
    // run under regular user, check should be `N`
    // run under root check should be `Y`
    Playbook::new("test_user_is_root", "", [user_is_root()], []).apply();
}
