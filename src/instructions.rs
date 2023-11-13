use std::path::PathBuf;

use crate::{
    actions::{action, create_dir_perm, PathPermissions},
    checks::{check, is_dir},
    instruction,
    interfaces::Action,
    playbook::Instruction,
};

pub fn create_dir_perm_if_missing<DirPath>(
    path: DirPath,
    permissions: PathPermissions,
) -> Instruction
where
    DirPath: Into<PathBuf>,
{
    let path: PathBuf = path.into();
    let path_name = path.to_string_lossy();
    instruction(action(
        format!("Create {path_name}"),
        create_dir_perm(&path, permissions),
    ))
    .confirm(check(format!("{path_name} directory"), is_dir(&path)))
}

pub fn named<Name>(name: Name, action_to_run: Box<dyn Action>) -> Instruction
where
    Name: Into<String>,
{
    instruction(action(name, action_to_run))
}
