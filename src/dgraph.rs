//! Allows building dependency graph of playbooks

use std::path::PathBuf;

use crate::{
    actions::{create_dir_perm, perm, write_file_perm},
    checks::{check, is_dir, is_file, user_is_root},
    instruction,
    interfaces::Check,
    playbook::Instruction,
    Playbook,
};

fn dep_flag_path<Name>(name: Name) -> PathBuf
where
    Name: AsRef<str>,
{
    let mut path = PathBuf::from("/srv/pass/applied");
    path.push(name.as_ref());
    path
}

/// This playbook need to be run before dependency graph can be used
pub fn init_dgraph() -> Playbook {
    Playbook::new(
        "Init dgraph",
        "Init dependency graph",
        [user_is_root(), is_dir("/srv")],
        [
            instruction(create_dir_perm("/srv/pass", perm(0o775, "root")))
                .confirm(is_dir("/srv/pass")),
            instruction(create_dir_perm("/srv/pass/applied", perm(0o775, "root")))
                .confirm(is_dir("/srv/pass/applied")),
        ],
    )
}

/// Check if dgraph supported
pub fn is_dgraph() -> Box<dyn Check> {
    check("Is dgraph", is_dir("/srv/pass/applied"))
}

/// Creates check for verifying if playbook is applied
pub fn is_applied<Name>(playbook_name: Name) -> Box<dyn Check>
where
    Name: Into<String>,
{
    let playbook_name: String = playbook_name.into();
    check(
        format!("Playbook `{}` applied", playbook_name),
        is_file(dep_flag_path(playbook_name)),
    )
}

/// Creates instruction to be used to mark playbook as applied
pub fn mark_applied<Name>(applied_playbook: Name) -> Instruction
where
    Name: Into<String>,
{
    let flag_path = dep_flag_path(applied_playbook.into());
    instruction(write_file_perm(&flag_path, "", perm(0o444, "root")))
        .with_env([
            user_is_root(),
            check("Dependency graph is supported", is_dir("/srv/pass/applied")),
        ])
        .confirm(is_file(&flag_path))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_dgraph() {
        // use manual tests
        //   - test_init_dgraph
        //   - test_is_applied
        //   - test_mark_applied
    }
}
