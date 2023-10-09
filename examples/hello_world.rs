use pass_tool::{run_cli, Playbook};

fn main() {
    let playbook = Playbook::new("Hello world", "Hello world example", [], []);
    run_cli(playbook, include_str!("hello_world.rs"));
}
