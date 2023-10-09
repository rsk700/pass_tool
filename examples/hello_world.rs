use pass_tool::{actions::write_file, checks::is_file, instruction, run_cli, Playbook};

fn main() {
    let file_path = "pass-example__hello_world.txt";
    let playbook = Playbook::new(
        "Hello world",
        "This example creates file with \"Hello, world!\" text, if file already exists it will do nothing",
        [],
        [instruction(write_file(file_path, "Hello, world!")).confirm(is_file(file_path))],
    );
    run_cli(playbook, include_str!("hello_world.rs"));
}
