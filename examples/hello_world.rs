use pass_tool::{run_with_cli, Playbook};

fn main() {
    let playbook = Playbook::new("Hello world", "Hello world example", [], []);
    run_with_cli(playbook, include_str!("hello_world.rs"));
}
