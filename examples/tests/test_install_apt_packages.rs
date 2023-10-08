use pass_tool::{
    actions::{command, install_apt_packages},
    checks::user_is_root,
    instruction, Playbook,
};

fn main() {
    // run, check packages is installed using commands `which nginx`, `which certbot`
    Playbook::new(
        "test_install_apt_packages",
        "",
        user_is_root(),
        [
            instruction(command(["apt", "update"])),
            instruction(install_apt_packages(["nginx", "certbot"])),
        ],
    )
    .apply();
}
