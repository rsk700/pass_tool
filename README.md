# Pass

`Pass` - is a tool for system configuration. Configuration change is described as `checks` and `actions`. `Check` is for checking current state of system (eg. if nginx installed). `Action` changes state of the system (eg. install nginx). `Pass` allows either apply changes or verify if changes can be applied based on described checks.

# How to use

`Checks` and `Actions` are organized into `Instructions`, and list of `Instructions` is making `Playbook`.

`Check` - only checks current state of system, doesn't change anything, it is just true/false flag

`Action` - changes system settings, can fail

`Playbook` - contains `environment checks` and list of `instructions`. `Environment checks` checked before any of instructions is performed.

`Instruction` - contains `environment checks`, `confirmation checks` and `action` to be applied:

- `Environment checks` checked before action, and must be all `true`
- `Confirmation checks` checked before and after action, if it `false` before action - action will be applied and `confirmation checks` checked again after action now all checks must be `true`, if it `true` before action - action will be skipped (action considered already applied)

# Example

Here is "Hello, World!" example:

```rust
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
```

Another example available in `examples/` folder is `https_webserver`, it will configure nginx with https support using letsencrypt certificate.

# More links

Also I'm making note taking and todo list app https://heaplist.app/