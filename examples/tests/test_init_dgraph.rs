use pass_tool::dgraph::init_dgraph;

fn main() {
    // run, check `/srv/pass/applied` is created with permissions:
    //   ls -l /srv/
    //   drwxrwxr-x 3 root root 4096 Oct 10 03:37 pass/
    //   ls -l /srv/pass/
    //   drwxrwxr-x 2 root root 4096 Oct 10 03:37 applied/
    init_dgraph().apply();
}
