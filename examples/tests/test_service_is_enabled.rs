use pass_tool::{
    checks::{service_is_disabled, service_is_enabled},
    Playbook,
};

fn main() {
    // install nginx `apt install nginx`
    // after `systemctl enable nginx`, check should be `YN`
    // after `systemctl disable nginx`, check should be `NY`
    Playbook::new(
        "test_is_service_enabled",
        "",
        [service_is_enabled("nginx"), service_is_disabled("nginx")],
        [],
    )
    .apply();
}
