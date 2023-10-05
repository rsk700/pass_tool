use pass_tool::{
    checks::{is_service_active, is_service_failed, is_service_inactive},
    Playbook,
};

fn main() {
    // install nginx
    // after `systemctl start nginx` checks should be `YNN`
    // after `systemctl stop nginx` checks should be `NYN`
    // make error in nginx conf file (add "aaaa" as first line in
    // /etc/nginx/nginx.conf) and start `systemctl start nginx`, checks should
    // be `NNY`
    Playbook::new(
        "test_is_service_status",
        "",
        [
            is_service_active("nginx"),
            is_service_inactive("nginx"),
            is_service_failed("nginx"),
        ],
        [],
    )
    .apply();
}
