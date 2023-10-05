use pass_tool::{
    checks::{is_service_disabled, is_service_enabled},
    Playbook,
};

fn main() {
    // install nginx `apt install nginx`
    // after `systemctl enable nginx`, check should be `YN`
    // after `systemctl disable nginx`, check should be `NY`
    Playbook::new(
        "test_is_service_enabled",
        "",
        [is_service_enabled("nginx"), is_service_disabled("nginx")],
        [],
    )
    .apply();
}
