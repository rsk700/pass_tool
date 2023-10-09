use pass_tool::{
    actions::{
        command, create_dir_perm, delete_file, install_apt_packages, many, named as action, perm,
        start_service, stop_service, write_file, write_file_perm,
    },
    checks::{
        is_file, named as check, not_op, service_is_inactive, stdout_contains_once, user_is_root,
    },
    instruction, run_cli_with_input, Playbook,
};

const ABOUT: &str = "Example installing nginx webserver with letsencrypt certificate.
This playbook will:
  - configure firewall (will allow ssh, http, https)
  - enable letsencrypt certificates for domain
  - configure nginx static website with https support";
const HELP: &str = "For configuration you need to provide comma separated values of your email address (for letsencrypt registration) and your domain name, example:

```
apply your_email@domain.com,your_domain.com
```";
const NGINX_CONF: &str = include_str!("https_webserver_nginx.conf");
const CERTBOT_RENEW: &str = include_str!("https_webserver_certbot-renew");
const INDEX_HTML: &str = include_str!("https_webserver_index.html");

fn main() {
    run_cli_with_input(
        |input| {
            let input = String::from_utf8(input.to_vec()).or(Err(HELP.to_owned()))?;
            let mut parts = input.split(',');
            let (email, domain) = if let [Some(e), Some(d)] = [parts.next(), parts.next()] {
                (e, d)
            } else {
                return Err(HELP.to_owned());
            };
            // todo: replace domain name
            let nginx_conf = NGINX_CONF;
            Ok(Playbook::new(
                "Install and configure nginx with https",
                ABOUT,
                [
                    user_is_root(),
                    check(
                        "Os is Ubuntu 20.04",
                        stdout_contains_once(["lsb_release", "-a"], "Ubuntu 20.04"),
                    ),
                ],
                [
                    instruction(action(
                        "Upgrade apt packages",
                        many([
                            command(["apt", "update", "-y"]),
                            command(["apt", "upgrade", "-y"]),
                        ]),
                    )),
                    instruction(install_apt_packages(["nginx", "certbot"])),
                    instruction(action(
                        "Configure firewall",
                        many([
                            command(["ufw", "allow", "ssh"]),
                            command(["ufw", "allow", "http"]),
                            command(["ufw", "allow", "https"]),
                            command(["ufw", "default", "deny", "incoming"]),
                            command(["ufw", "default", "allow", "outgoing"]),
                        ]),
                    ))
                    .with_env(check(
                        "Firewall is inactive",
                        stdout_contains_once(["ufw", "status"], "Status: inactive"),
                    )),
                    instruction(action("Stop nginx", stop_service("nginx"))),
                    instruction(action(
                        "Request ssl certificate",
                        command([
                            "certbot",
                            "certonly",
                            "--standalone",
                            "--agree-tos",
                            "--no-eff-email",
                            "-m",
                            email,
                            "-d",
                            domain,
                        ]),
                    ))
                    .with_env(check("Nginx is inactive", service_is_inactive("nginx")))
                    .confirm(check(
                        "Ssl certificate exists",
                        is_file(format!("/etc/letsencrypt/live/{domain}/fullchain.pem")),
                    )),
                    instruction(action(
                        "Enable certbot renew",
                        write_file_perm(
                            "/etc/cron.weekly/certbot-renew",
                            CERTBOT_RENEW,
                            perm(0o555, "root"),
                        ),
                    ))
                    .confirm(check(
                        "Certbot renew is enabled",
                        is_file("/etc/cron.weekly/certbot-renew"),
                    )),
                    instruction(action(
                        "Delete default nginx site",
                        delete_file("/etc/nginx/sites-enabled/default"),
                    ))
                    .confirm(check(
                        "Default nginx site deleted",
                        not_op(is_file("/etc/nginx/sites-enabled/default")),
                    )),
                    instruction(action(
                        "Create pass demo site nginx configuration",
                        write_file("/etc/nginx/sites-enabled/pass-demo", nginx_conf),
                    ))
                    .confirm(check(
                        "Pass demo site nginx configuration is exists",
                        is_file("/etc/nginx/sites-enabled/pass-demo"),
                    )),
                    instruction(action(
                        "Create website files",
                        many([
                            create_dir_perm("/srv/pass-demo-site", perm(0o774, "www-data")),
                            write_file_perm(
                                "/srv/pass-demo-site/index.html",
                                INDEX_HTML,
                                perm(0o664, "www-data"),
                            ),
                        ]),
                    )),
                    instruction(action("Start nginx", start_service("nginx"))),
                    instruction(action("Start firewall", start_service("ufw"))),
                    instruction(action(
                        "Enable firewall",
                        command(["ufw", "--force", "enable"]),
                    )),
                ],
            ))
        },
        HELP,
        include_str!("https_webserver.rs"),
    );
}
