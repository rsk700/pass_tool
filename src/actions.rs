use std::{
    ffi::OsString,
    io::ErrorKind,
    os::unix::fs::{chown, PermissionsExt},
    path::{Path, PathBuf},
};

use crate::{
    interfaces::{Action, ActionResult},
    pattern::Pattern,
    process::{norm_cmd, run},
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
pub fn action<Name>(name: Name, action: Box<dyn Action>) -> Box<dyn Action>
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
pub fn many<Actions>(actions: Actions) -> Box<dyn Action>
where
    Actions: Into<Vec<Box<dyn Action>>>,
{
    Many::new(actions.into()).into_action()
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

/// Replaces provided pattern in file with new data exactly once, will fail if
/// file contains pattern multiple times or no pattern at all
pub struct ReplaceInFileOnce {
    path: PathBuf,
    pattern: Pattern,
    replacement: Vec<u8>,
}

impl ReplaceInFileOnce {
    const NAME: &'static str = "ReplaceInFileOnce";

    pub fn new(path: PathBuf, pattern: Pattern, replacement: Vec<u8>) -> Self {
        Self {
            path,
            pattern,
            replacement,
        }
    }
}

impl Action for ReplaceInFileOnce {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        let Some(new_content) = std::fs::read(&self.path)
            .ok()
            .and_then(|c| self.pattern.replace_once(&c, &self.replacement))
        else {
            return ActionResult::Fail;
        };
        if std::fs::write(&self.path, new_content).is_err() {
            ActionResult::Fail
        } else {
            ActionResult::Ok
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [ReplaceInFileOnce]
pub fn replace_in_file_once<TPath, TPattern, TData>(
    path: TPath,
    pattern: TPattern,
    replacement: TData,
) -> Box<dyn Action>
where
    TPath: Into<PathBuf>,
    TPattern: Into<Pattern>,
    TData: Into<Vec<u8>>,
{
    ReplaceInFileOnce::new(path.into(), pattern.into(), replacement.into()).into_action()
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

/// init [RenamePath]
pub fn rename_path<FilePath>(path: FilePath, new_path: FilePath) -> Box<dyn Action>
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

/// Sends control command to service
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

/// Copy file into provided directory, if optional new name set file will be
/// renamed
pub struct CopyFile {
    file_path: PathBuf,
    target_dir: PathBuf,
    new_name: Option<OsString>,
}

impl CopyFile {
    const NAME: &'static str = "CopyFile";

    pub fn new(file_path: PathBuf, target_dir: PathBuf, new_name: Option<OsString>) -> Self {
        Self {
            file_path,
            target_dir,
            new_name,
        }
    }
}

impl Action for CopyFile {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        let target_path = if let Some(name) = &self.new_name {
            self.target_dir.join(name)
        } else {
            let Some(name) = self.file_path.file_name() else {
                return ActionResult::Fail;
            };
            self.target_dir.join(name)
        };
        if std::fs::copy(&self.file_path, target_path).is_ok() {
            ActionResult::Ok
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [CopyFile], will copy file without rename
pub fn copy_file<FilePath, TargetDir>(file_path: FilePath, target_dir: TargetDir) -> Box<dyn Action>
where
    FilePath: Into<PathBuf>,
    TargetDir: Into<PathBuf>,
{
    CopyFile::new(file_path.into(), target_dir.into(), None).into_action()
}

/// init [CopyFile], will copy file with rename
pub fn copy_file_named<FilePath, TargetDir, NewName>(
    file_path: FilePath,
    target_dir: TargetDir,
    new_name: NewName,
) -> Box<dyn Action>
where
    FilePath: Into<PathBuf>,
    TargetDir: Into<PathBuf>,
    NewName: Into<OsString>,
{
    CopyFile::new(file_path.into(), target_dir.into(), Some(new_name.into())).into_action()
}

/// Changes current working directory to the provided one
pub struct SetDir(PathBuf);

impl SetDir {
    const NAME: &'static str = "SetDir";

    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

impl Action for SetDir {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        if std::env::set_current_dir(&self.0).is_ok() {
            ActionResult::Ok
        } else {
            ActionResult::Fail
        }
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// init [SetDir]
pub fn set_dir<Dir>(dir: Dir) -> Box<dyn Action>
where
    Dir: Into<PathBuf>,
{
    SetDir(dir.into()).into_action()
}

/// Wait until provided file appears on disk (can be used for some type of
/// synchronization)
pub struct WaitForFile(PathBuf);

impl WaitForFile {
    const NAME: &'static str = "WaitForFile";

    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

impl Action for WaitForFile {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self) -> ActionResult {
        while !self.0.is_file() {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        ActionResult::Ok
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

/// inits [WaitForFile]
pub fn wait_for_file<P>(path: P) -> Box<dyn Action>
where
    P: Into<PathBuf>,
{
    WaitForFile::new(path.into()).into_action()
}

/// implements [Action] for tuple with name and function
impl<N, F> Action for (N, F)
where
    N: AsRef<str> + 'static,
    F: Fn() -> ActionResult + 'static,
{
    fn name(&self) -> &str {
        self.0.as_ref()
    }

    fn run(&self) -> ActionResult {
        self.1()
    }

    fn into_action(self) -> Box<dyn Action> {
        Box::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pattern::re;
    use std::time::{Duration, Instant};

    const NOT_A_FILE: &str = "/tmp/not-a-pass-test-file-5555555555";

    fn create_test_file(name: &str) -> String {
        let path = format!("/tmp/pass-test-file-111222333-{}", name);
        std::fs::write(&path, "aaabbbccc").unwrap();
        path
    }

    #[test]
    fn test_always_ok() {
        assert_eq!(always_ok().run(), ActionResult::Ok);
    }

    #[test]
    fn test_always_fail() {
        assert_eq!(always_fail().run(), ActionResult::Fail);
    }

    #[test]
    fn test_named() {
        let a = action("aaa", always_ok());
        assert_eq!(a.name(), "aaa");
        assert_eq!(a.run(), ActionResult::Ok);
    }

    #[test]
    fn test_many() {
        assert_eq!(many([]).run(), ActionResult::Ok);
        assert_eq!(many([always_ok(), always_ok()]).run(), ActionResult::Ok);
        assert_eq!(many([always_ok(), always_fail()]).run(), ActionResult::Fail);
    }

    #[test]
    fn test_command() {
        assert_eq!(command(["echo", "1"]).run(), ActionResult::Ok);
        assert_eq!(command(["false"]).run(), ActionResult::Fail);
        assert_eq!(
            command(["random-incorrect-command-aaabbb222"]).run(),
            ActionResult::Fail
        );
    }

    #[test]
    fn test_invert() {
        assert_eq!(invert(always_ok()).run(), ActionResult::Fail);
    }

    #[test]
    fn test_install_apt_packages() {
        // use manual test test_install_apt_packages
    }

    #[test]
    fn test_delete_file() {
        assert_eq!(delete_file(NOT_A_FILE).run(), ActionResult::Ok);
        let p = create_test_file("test_delete_file");
        assert_eq!(delete_file(&p).run(), ActionResult::Ok);
        let p: PathBuf = p.into();
        assert!(!p.exists());
    }

    #[test]
    fn test_write_file() {
        let p = create_test_file("test_write_file");
        assert_eq!(write_file(&p, "111").run(), ActionResult::Ok);
        assert_eq!(std::fs::read(&p).unwrap(), "111".as_bytes());
        std::fs::remove_file(&p).unwrap();
        // use manual test test_write_file
    }

    #[test]
    fn test_create_dir() {
        assert_eq!(
            create_dir("/tmp/pass-test-file-111222333-test_create_dir/1/1/1").run(),
            ActionResult::Fail
        );
        let path = "/tmp/pass-test-file-111222333-test_create_dir";
        assert_eq!(create_dir(path).run(), ActionResult::Ok);
        // checking no error if directory already exists
        assert_eq!(create_dir(path).run(), ActionResult::Ok);
        std::fs::remove_dir(path).unwrap();
        // use manual test test_create_dir
    }

    #[test]
    fn test_set_path_permissions() {
        // use manual test test_set_path_permissions
    }

    #[test]
    fn test_replace_in_file_once() {
        {
            let p = create_test_file("test_replace_in_file_once");
            assert_eq!(replace_in_file_once(&p, "ab", "11").run(), ActionResult::Ok);
            assert_eq!(std::fs::read(&p).unwrap(), "aa11bbccc".as_bytes());
            std::fs::remove_file(&p).unwrap();
        }
        {
            let p = create_test_file("test_replace_in_file_once");
            assert_eq!(replace_in_file_once(&p, "a", "a").run(), ActionResult::Fail);
            assert_eq!(std::fs::read(&p).unwrap(), "aaabbbccc".as_bytes());
            assert_eq!(
                replace_in_file_once(&p, "a", "11").run(),
                ActionResult::Fail
            );
            assert_eq!(std::fs::read(&p).unwrap(), "aaabbbccc".as_bytes());
            assert_eq!(
                replace_in_file_once(&p, "111", "222").run(),
                ActionResult::Fail
            );
            assert_eq!(replace_in_file_once(&p, "", "").run(), ActionResult::Fail);
            assert_eq!(std::fs::read(&p).unwrap(), "aaabbbccc".as_bytes());
            assert_eq!(
                replace_in_file_once(&p, "", "111").run(),
                ActionResult::Fail
            );
            assert_eq!(std::fs::read(&p).unwrap(), "aaabbbccc".as_bytes());
            std::fs::remove_file(&p).unwrap();
        }
        // regex
        {
            let p = create_test_file("test_replace_in_file_once");
            assert_eq!(
                replace_in_file_once(&p, re("a+"), "1").run(),
                ActionResult::Ok
            );
            assert_eq!(std::fs::read(&p).unwrap(), "1bbbccc".as_bytes());
            std::fs::remove_file(&p).unwrap();
        }
        {
            let p = create_test_file("test_replace_in_file_once");
            assert_eq!(
                replace_in_file_once(&p, re("a."), "1").run(),
                ActionResult::Fail
            );
            std::fs::remove_file(&p).unwrap();
        }
    }

    #[test]
    fn test_rename_path() {
        {
            let path1 = "/tmp/pass-test-file-111222333-test_rename_path_1";
            let path2 = "/tmp/pass-test-file-111222333-test_rename_path_2";
            std::fs::write(path1, "path1").unwrap();
            assert_eq!(rename_path(path1, path2).run(), ActionResult::Ok);
            let path_buf: PathBuf = path2.into();
            assert!(path_buf.exists());
            std::fs::remove_file(path2).unwrap();
        }
        {
            let path1 = "/tmp/pass-test-dir-111222333-test_rename_path_1";
            let path2 = "/tmp/pass-test-dir-111222333-test_rename_path_2";
            std::fs::create_dir(path1).unwrap();
            assert_eq!(rename_path(path1, path2).run(), ActionResult::Ok);
            let path_buf: PathBuf = path2.into();
            assert!(path_buf.exists());
            std::fs::remove_dir(path2).unwrap();
        }
    }

    #[test]
    fn test_service_command() {
        // use manual tests:
        //  - test_service_command_start
        //  - test_service_command_stop
        //  - test_service_command_restart
        //  - test_service_command_reload
        //  - test_service_command_enable
        //  - test_service_command_disable
    }

    #[test]
    fn test_tuple_as_action() {
        {
            let a = ("a1", || ActionResult::Ok).into_action();
            assert_eq!(a.name(), "a1");
            matches!(a.run(), ActionResult::Ok);
        }
        {
            let a = ("a2".to_owned(), || ActionResult::Fail).into_action();
            assert_eq!(a.name(), "a2");
            matches!(a.run(), ActionResult::Fail);
        }
    }

    #[test]
    fn test_copy_file() {
        let d: PathBuf = "/tmp/pass-test-dir-111222333-copy-file".into();
        let p = create_test_file("original_file");
        let p_path: PathBuf = p.clone().into();
        let p_name = p_path.file_name().unwrap();
        let new_name = "pass-test-file-111222333-copy-file-new-name";
        std::fs::create_dir(&d).unwrap();
        assert_eq!(copy_file(&p, &d).run(), ActionResult::Ok);
        assert!(d.join(p_name).is_file());
        assert_eq!(copy_file_named(&p, &d, new_name).run(), ActionResult::Ok);
        assert!(d.join(new_name).is_file());
        std::fs::remove_file(&p).unwrap();
        std::fs::remove_file(d.join(p_name)).unwrap();
        std::fs::remove_file(d.join(new_name)).unwrap();
        std::fs::remove_dir(&d).unwrap();
    }

    #[test]
    fn test_set_dir() {
        // tested in dir_context.rs
    }

    #[test]
    fn test_wait_for_file() {
        let path: &'static str = "/tmp/pass-test-file-111222333-test_wait_for_file";
        let time_a = Instant::now();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(4));
            std::fs::write(path, "").unwrap();
        });
        assert_eq!(wait_for_file(path).run(), ActionResult::Ok);
        let time_b = Instant::now();
        std::fs::remove_file(path).unwrap();
        assert!((time_b - time_a).as_secs_f32() >= 4.0);
    }
}
