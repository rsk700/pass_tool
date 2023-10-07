use std::{
    io::ErrorKind,
    os::unix::fs::{chown, PermissionsExt},
    path::{Path, PathBuf},
};

use crate::{
    interfaces::{Action, ActionResult},
    process::{norm_cmd, run},
    search::find_pattern,
};

/// Action which does nothing and always succeeds
pub struct AlwaysOk;

impl AlwaysOk {
    const NAME: &'static str = "AlwaysOk";
}

impl Action for AlwaysOk {
    fn name(&self) -> &str {
        Self::NAME
    }
    fn run(&self) -> ActionResult {
        ActionResult::Ok
    }
    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

pub fn always_ok() -> Box<dyn Action> {
    AlwaysOk.into_action()
}

/// Action which does nothing and always fails
pub struct AlwaysFail;

impl AlwaysFail {
    const NAME: &'static str = "AlwaysFail";
}

impl Action for AlwaysFail {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        ActionResult::Fail
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [AlwaysFail]
pub fn always_fail() -> Box<dyn Action> {
    AlwaysFail.into_action()
}

/// Renames another action
pub struct Named {
    name: String,
    action: Box<dyn Action>,
}

impl Named {
    pub fn new(name: String, action: Box<dyn Action>) -> Self {
        Self { name, action }
    }
}

impl Action for Named {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> ActionResult {
        self.action.run()
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [Named]
pub fn named<Name>(name: Name, action: Box<dyn Action>) -> Box<dyn Action>
where
    Name: Into<String>,
{
    Named::new(name.into(), action).into_action()
}

/// Runs multiple actions as one action, fails if any of actions fail
pub struct Many {
    actions: Vec<Box<dyn Action>>,
}

impl Many {
    const NAME: &'static str = "Many";

    pub fn new(actions: Vec<Box<dyn Action>>) -> Self {
        Self { actions }
    }
}

impl Action for Many {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        for action in &self.actions {
            if action.run() == ActionResult::Fail {
                return ActionResult::Fail;
            }
        }
        ActionResult::Ok
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [Many]
pub fn many(actions: Vec<Box<dyn Action>>) -> Box<dyn Action> {
    Many::new(actions).into_action()
}

/// Runs external process using provided command
pub struct Command {
    cmd: Vec<String>,
}

impl Command {
    const NAME: &'static str = "Command";

    pub fn new(cmd: Vec<String>) -> Self {
        Self { cmd }
    }
}

impl Action for Command {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        let result = run(&self.cmd);
        if result.ok() {
            ActionResult::Ok
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [Command]
pub fn command<Cmd, Arg>(cmd: Cmd) -> Box<dyn Action>
where
    Arg: Into<String>,
    Cmd: Into<Vec<Arg>>,
{
    Command::new(norm_cmd(cmd)).into_action()
}

/// Inverts result of another action, Ok becomes Fail
pub struct Invert {
    action: Box<dyn Action>,
}

impl Invert {
    const NAME: &'static str = "Invert";

    pub fn new(action: Box<dyn Action>) -> Self {
        Self { action }
    }
}

impl Action for Invert {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        match self.action.run() {
            ActionResult::Ok => ActionResult::Fail,
            ActionResult::Fail => ActionResult::Ok,
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [Invert]
pub fn invert(action: Box<dyn Action>) -> Box<dyn Action> {
    Invert::new(action).into_action()
}

/// Install provided apt packages
pub struct InstallAptPackages {
    packages: Vec<String>,
}

impl InstallAptPackages {
    const NAME: &'static str = "InstallAptPackages";

    pub fn new(packages: Vec<String>) -> Self {
        Self { packages }
    }
}

impl Action for InstallAptPackages {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        let mut apt_cmd = vec!["apt", "install", "-y"];
        let mut packages: Vec<&str> = self.packages.iter().map(|p| p.as_str()).collect();
        apt_cmd.append(&mut packages);
        let result = run(&norm_cmd(apt_cmd));
        if result.ok() {
            ActionResult::Ok
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [InstallAptPackages]
pub fn install_apt_packages<Package, Packages>(packages: Packages) -> Box<dyn Action>
where
    Package: Into<String>,
    Packages: Into<Vec<Package>>,
{
    let packages = packages.into().into_iter().map(|c| c.into()).collect();
    InstallAptPackages::new(packages).into_action()
}

/// Deletes file
pub struct DeleteFile {
    path: PathBuf,
}

impl DeleteFile {
    const NAME: &'static str = "DeleteFile";

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Action for DeleteFile {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        match std::fs::remove_file(&self.path) {
            Ok(_) => ActionResult::Ok,
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    // file does not exist, nothing to delete
                    ActionResult::Ok
                } else {
                    ActionResult::Fail
                }
            }
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [DeleteFile]
pub fn delete_file<FilePath>(path: FilePath) -> Box<dyn Action>
where
    FilePath: Into<PathBuf>,
{
    DeleteFile::new(path.into()).into_action()
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PathPermissions {
    pub access_mode: Option<u32>,
    pub user_owner: Option<String>,
    pub group_owner: Option<String>,
}

impl PathPermissions {
    pub fn new(access_mode: u32, user_owner: String, group_owner: String) -> Self {
        Self {
            access_mode: Some(access_mode),
            user_owner: Some(user_owner),
            group_owner: Some(group_owner),
        }
    }

    pub fn access(mut self, access_mode: u32) -> Self {
        self.access_mode = Some(access_mode);
        self
    }

    pub fn user<Name>(mut self, user: Name) -> Self
    where
        Name: Into<String>,
    {
        self.user_owner = Some(user.into());
        self
    }

    pub fn group<Name>(mut self, group: Name) -> Self
    where
        Name: Into<String>,
    {
        self.group_owner = Some(group.into());
        self
    }

    pub fn owner<Name>(self, owner: Name) -> Self
    where
        Name: Into<String>,
    {
        let owner: String = owner.into();
        self.user(owner.clone()).group(owner)
    }

    pub fn apply<FilePath>(&self, path: FilePath) -> Option<()>
    where
        FilePath: AsRef<Path>,
    {
        if let Some(access_mode) = self.access_mode {
            let mut permissions = std::fs::metadata(path.as_ref()).ok()?.permissions();
            permissions.set_mode(access_mode);
            std::fs::set_permissions(path.as_ref(), permissions).ok()?;
        }
        let uid: Option<u32> = if let Some(user_owner) = self.user_owner.as_ref() {
            Some(nix::unistd::User::from_name(user_owner).ok()??.uid.into())
        } else {
            None
        };
        let gid: Option<u32> = if let Some(group_owner) = self.group_owner.as_ref() {
            Some(nix::unistd::Group::from_name(group_owner).ok()??.gid.into())
        } else {
            None
        };
        // noop if both uid and gid is None
        chown(path.as_ref(), uid, gid).ok()
    }
}

pub fn perm<Name>(access_mode: u32, owner: Name) -> PathPermissions
where
    Name: Into<String>,
{
    PathPermissions::default().access(access_mode).owner(owner)
}

/// Write provided data into file
pub struct WriteFile {
    path: PathBuf,
    data: Vec<u8>,
    perm: PathPermissions,
}

impl WriteFile {
    const NAME: &'static str = "WriteFile";

    pub fn new(path: PathBuf, data: Vec<u8>, perm: PathPermissions) -> Self {
        Self { path, data, perm }
    }
}

impl Action for WriteFile {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        // todo: use exclusive file access? no support in std, need external lib
        if std::fs::write(&self.path, &self.data).is_ok() {
            // if permissions not set this is noop
            if self.perm.apply(&self.path).is_some() {
                ActionResult::Ok
            } else {
                ActionResult::Fail
            }
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [WriteFile]
pub fn write_file<FilePath, Content>(path: FilePath, content: Content) -> Box<dyn Action>
where
    FilePath: Into<PathBuf>,
    Content: Into<Vec<u8>>,
{
    WriteFile::new(path.into(), content.into(), PathPermissions::default()).into_action()
}

/// init [WriteFile] with setting custom permission for file
pub fn write_file_perm<FilePath, Content>(
    path: FilePath,
    content: Content,
    perm: PathPermissions,
) -> Box<dyn Action>
where
    FilePath: Into<PathBuf>,
    Content: Into<Vec<u8>>,
{
    WriteFile::new(path.into(), content.into(), perm).into_action()
}

/// Create directory
pub struct CreateDir {
    path: PathBuf,
    perm: PathPermissions,
}

impl CreateDir {
    const NAME: &'static str = "CreateDir";

    pub fn new(path: PathBuf, perm: PathPermissions) -> Self {
        Self { path, perm }
    }
}

impl Action for CreateDir {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        if let Err(e) = std::fs::create_dir(&self.path) {
            if e.kind() != ErrorKind::AlreadyExists {
                // error creating new directory, and directory not exists yet
                return ActionResult::Fail;
            }
        }
        if self.perm.apply(&self.path).is_some() {
            ActionResult::Ok
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [CreateDir]
pub fn create_dir<DirPath>(path: DirPath) -> Box<dyn Action>
where
    DirPath: Into<PathBuf>,
{
    CreateDir::new(path.into(), PathPermissions::default()).into_action()
}

/// init [CreateDir] with custom permissions for created directory
pub fn create_dir_perm<DirPath>(path: DirPath, perm: PathPermissions) -> Box<dyn Action>
where
    DirPath: Into<PathBuf>,
{
    CreateDir::new(path.into(), perm).into_action()
}

/// Set custom permissions for path (file or directory)
pub struct SetPathPermissions {
    path: PathBuf,
    perm: PathPermissions,
}

impl SetPathPermissions {
    const NAME: &'static str = "SetPathPermissions";

    pub fn new(path: PathBuf, perm: PathPermissions) -> Self {
        Self { path, perm }
    }
}

impl Action for SetPathPermissions {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        if self.perm.apply(&self.path).is_some() {
            ActionResult::Ok
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [SetPathPermissions] with access_mode and user, group equal to owner
pub fn set_perm<FilePath, Name>(path: FilePath, access_mode: u32, owner: Name) -> Box<dyn Action>
where
    FilePath: Into<PathBuf>,
    Name: Into<String>,
{
    SetPathPermissions::new(path.into(), perm(access_mode, owner)).into_action()
}

/// init [SetPathPermissions]
pub fn set_perm_full<FilePath, Name>(
    path: FilePath,
    access_mode: u32,
    user: Name,
    group: Name,
) -> Box<dyn Action>
where
    FilePath: Into<PathBuf>,
    Name: Into<String>,
{
    SetPathPermissions::new(
        path.into(),
        PathPermissions::default()
            .access(access_mode)
            .user(user)
            .group(group),
    )
    .into_action()
}

/// Replaces target pattern in file with new data exactly once, will fail if
/// file contains target pattern multiple times
pub struct ReplaceInFileOnce {
    path: PathBuf,
    target: Vec<u8>,
    new_data: Vec<u8>,
}

impl ReplaceInFileOnce {
    const NAME: &'static str = "ReplaceInFileOnce";

    pub fn new(path: PathBuf, target: Vec<u8>, new_data: Vec<u8>) -> Self {
        Self {
            path,
            target,
            new_data,
        }
    }
}

impl Action for ReplaceInFileOnce {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        // helps to deal with replacing "" with "" (otherwise will be Fail)
        if self.target == self.new_data {
            return ActionResult::Ok;
        }
        if let Ok(c) = std::fs::read(&self.path) {
            if let Some(i) = find_pattern(&c, &self.target) {
                if find_pattern(&c[i + self.target.len()..], &self.target).is_some() {
                    // found second entry of target, but expecting to have it only once in file
                    return ActionResult::Fail;
                }
                let new_content: Vec<u8> = c[0..i]
                    .iter()
                    .copied()
                    .chain(self.new_data.iter().copied())
                    .chain(c[i + self.target.len()..].iter().copied())
                    .collect();
                if std::fs::write(&self.path, new_content).is_err() {
                    ActionResult::Fail
                } else {
                    ActionResult::Ok
                }
            } else {
                ActionResult::Fail
            }
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [ReplaceInFileOnce]
pub fn replace_in_file_once<FilePath, Data>(
    path: FilePath,
    target: Data,
    new_data: Data,
) -> Box<dyn Action>
where
    FilePath: Into<PathBuf>,
    Data: Into<Vec<u8>>,
{
    ReplaceInFileOnce::new(path.into(), target.into(), new_data.into()).into_action()
}

/// Rename file or directory
pub struct RenamePath {
    path: PathBuf,
    new_path: PathBuf,
}

impl RenamePath {
    const NAME: &'static str = "RenamePath";

    pub fn new(path: PathBuf, new_path: PathBuf) -> Self {
        Self { path, new_path }
    }
}

impl Action for RenamePath {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        if std::fs::rename(&self.path, &self.new_path).is_ok() {
            ActionResult::Ok
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [RenameDir]
pub fn rename_dir<FilePath>(path: FilePath, new_path: FilePath) -> Box<dyn Action>
where
    FilePath: Into<PathBuf>,
{
    RenamePath::new(path.into(), new_path.into()).into_action()
}

pub enum ServiceCommands {
    Start,
    Stop,
    Restart,
    Reload,
    Enable,
    Disable,
}

/// Sends control comand to service
pub struct ServiceCommand {
    service: String,
    command: ServiceCommands,
}

impl ServiceCommand {
    const NAME: &'static str = "ServiceCommand";

    pub fn new(service: String, command: ServiceCommands) -> Self {
        Self { service, command }
    }
}

impl Action for ServiceCommand {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        let command = match self.command {
            ServiceCommands::Start => "start",
            ServiceCommands::Stop => "stop",
            ServiceCommands::Restart => "restart",
            ServiceCommands::Reload => "reload",
            ServiceCommands::Enable => "enable",
            ServiceCommands::Disable => "disable",
        };
        let result = run(&norm_cmd(["systemctl", command, &self.service]));
        if result.ok() {
            ActionResult::Ok
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [ServiceCommand] with start service command
pub fn start_service<Name>(service: Name) -> Box<dyn Action>
where
    Name: Into<String>,
{
    ServiceCommand::new(service.into(), ServiceCommands::Start).into_action()
}

/// init [ServiceCommand] with stop service command
pub fn stop_service<Name>(service: Name) -> Box<dyn Action>
where
    Name: Into<String>,
{
    ServiceCommand::new(service.into(), ServiceCommands::Stop).into_action()
}

/// init [ServiceCommand] with restart service command
pub fn restart_service<Name>(service: Name) -> Box<dyn Action>
where
    Name: Into<String>,
{
    ServiceCommand::new(service.into(), ServiceCommands::Restart).into_action()
}

/// init [ServiceCommand] with reload service command
pub fn reload_service<Name>(service: Name) -> Box<dyn Action>
where
    Name: Into<String>,
{
    ServiceCommand::new(service.into(), ServiceCommands::Reload).into_action()
}

/// init [ServiceCommand] with enable service command
pub fn enable_service<Name>(service: Name) -> Box<dyn Action>
where
    Name: Into<String>,
{
    ServiceCommand::new(service.into(), ServiceCommands::Enable).into_action()
}

/// init [ServiceCommand] with disable service command
pub fn disable_service<Name>(service: Name) -> Box<dyn Action>
where
    Name: Into<String>,
{
    ServiceCommand::new(service.into(), ServiceCommands::Disable).into_action()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_actions() {
        {
            let actions = always_ok();
            matches!(actions.run(), ActionResult::Ok);
        }
    }
}
