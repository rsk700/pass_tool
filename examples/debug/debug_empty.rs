use pass_tool::Playbook;

pub fn main() {
    assert!(Playbook::new("empty", "empty playbook", [], [])
        .apply()
        .ok());
}
