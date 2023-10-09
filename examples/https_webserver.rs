use pass_tool::{run_cli_with_input, Playbook};

const HELP: &str = "For configuration you need to provide comma separated values of your email address (for letsencrypt registration) and your domain name, example:

```
apply your_email@domain.com,your_domain.com
```";

fn main() {
    run_cli_with_input(
        |input| {
            Playbook::new(
                "Install nginx with https",
                "Example installing nginx webserver with letsencrypt certificate",
                [],
                [],
            )
        },
        HELP,
        include_str!("https_webserver.rs"),
    );
}
