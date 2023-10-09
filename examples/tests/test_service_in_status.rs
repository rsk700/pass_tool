use pass_tool::{
    checks::{service_is_active, service_is_failed, service_is_inactive},
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
            service_is_active("nginx"),
            service_is_inactive("nginx"),
            service_is_failed("nginx"),
        ],
        [],
    )
    .apply();
}
