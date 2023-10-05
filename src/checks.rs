use crate::{
    interfaces::Check,
    process::{norm_cmd, run, ProcessOutput},
    search::contains_once,
};
use nix::unistd::Uid;
use std::{fs::OpenOptions, path::PathBuf};

/// Check which always succeed
pub struct AlwaysOk;

impl AlwaysOk {
    const NAME: &'static str = "AlwaysOk";
}

impl Check for AlwaysOk {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        true
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [AlwaysOk]
pub fn always_ok() -> Box<dyn Check> {
    AlwaysOk.as_check()
}

/// Check which always fails
pub struct AlwaysFail;

impl AlwaysFail {
    const NAME: &'static str = "AlwaysFail";
}

impl Check for AlwaysFail {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        false
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [AlwaysFail]
pub fn always_fail() -> Box<dyn Check> {
    AlwaysFail.as_check()
}

/// Check which allows to rename another check
pub struct Named {
    name: String,
    check: Box<dyn Check>,
}

impl Named {
    pub fn new(name: String, check: Box<dyn Check>) -> Self {
        Self { name, check }
    }
}

impl Check for Named {
    fn name(&self) -> &str {
        &self.name
    }

    fn yes(&self) -> bool {
        self.check.yes()
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [Named]
pub fn named<Name>(name: Name, check: Box<dyn Check>) -> Box<dyn Check>
where
    Name: Into<String>,
{
    Named::new(name.into(), check).as_check()
}

/// Checks if current user is root
pub struct UserIsRoot;

impl UserIsRoot {
    const NAME: &'static str = "UserIsRoot";
}

impl Check for UserIsRoot {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        Uid::effective().is_root()
    }
    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [UserIsRoot]
pub fn user_is_root() -> Box<dyn Check> {
    UserIsRoot.as_check()
}

/// Checks if provided path is a file, does not test if file can be read/written
pub struct IsFile {
    path: PathBuf,
}

impl IsFile {
    const NAME: &'static str = "IsFile";

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Check for IsFile {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        if let Ok(m) = std::fs::metadata(&self.path) {
            m.is_file()
        } else {
            false
        }
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [IsFile]
pub fn is_file<FilePath>(path: FilePath) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
{
    IsFile::new(path.into()).as_check()
}

/// Checks if provided path is a directory
pub struct IsDir {
    path: PathBuf,
}

impl IsDir {
    const NAME: &'static str = "IsDir";

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Check for IsDir {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        if let Ok(m) = std::fs::metadata(&self.path) {
            m.is_dir()
        } else {
            false
        }
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [IsDir]
pub fn is_dir<FilePath>(path: FilePath) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
{
    IsDir::new(path.into()).as_check()
}

/// Checks if can read provided path
pub struct CanRead {
    path: PathBuf,
}

impl CanRead {
    const NAME: &'static str = "CanRead";

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Check for CanRead {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        std::fs::File::open(&self.path).is_ok()
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [CanRead]
pub fn can_read<FilePath>(path: FilePath) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
{
    CanRead::new(path.into()).as_check()
}

/// Checks if can write provided path
pub struct CanWrite {
    path: PathBuf,
}

impl CanWrite {
    const NAME: &'static str = "CanWrite";

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Check for CanWrite {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        OpenOptions::new()
            .create(false)
            .append(true)
            .truncate(false)
            .open(&self.path)
            .is_ok()
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [CanWrite]
pub fn can_write<FilePath>(path: FilePath) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
{
    CanWrite::new(path.into()).as_check()
}

/// Negates another check
pub struct NotOp {
    check: Box<dyn Check>,
}

impl NotOp {
    const NAME: &'static str = "NotOp";

    pub fn new(check: Box<dyn Check>) -> Self {
        Self { check }
    }
}

impl Check for NotOp {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        !self.check.yes()
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [NotOp]
pub fn not_op(check: Box<dyn Check>) -> Box<dyn Check> {
    NotOp::new(check).as_check()
}

/// Checks if any of provided checks succeed
pub struct OrOp {
    checks: Vec<Box<dyn Check>>,
}

impl OrOp {
    const NAME: &'static str = "OrOp";

    pub fn new(checks: Vec<Box<dyn Check>>) -> Self {
        Self { checks }
    }
}

impl Check for OrOp {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        for c in &self.checks {
            if c.yes() {
                return true;
            }
        }
        false
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [OrOp]
pub fn or_op<Checks>(checks: Checks) -> Box<dyn Check>
where
    Checks: Into<Vec<Box<dyn Check>>>,
{
    OrOp::new(checks.into()).as_check()
}

/// Checks if all provided checks succeed
pub struct AndOp {
    checks: Vec<Box<dyn Check>>,
}

impl AndOp {
    const NAME: &'static str = "AndOp";

    pub fn new(checks: Vec<Box<dyn Check>>) -> Self {
        Self { checks }
    }
}

impl Check for AndOp {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        self.checks.iter().all(|c| c.yes())
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [AndOp]
pub fn and_op<Checks>(checks: Checks) -> Box<dyn Check>
where
    Checks: Into<Vec<Box<dyn Check>>>,
{
    AndOp::new(checks.into()).as_check()
}

/// Checks if stdout output of command contains provided data exactly once
pub struct StdoutContainsOnce {
    cmd: Vec<String>,
    data: Vec<u8>,
}

impl StdoutContainsOnce {
    const NAME: &'static str = "StdoutContainsOnce";

    pub fn new(cmd: Vec<String>, data: Vec<u8>) -> Self {
        Self { cmd, data }
    }
}

impl Check for StdoutContainsOnce {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        let result = run(&self.cmd);
        if let Some(ProcessOutput { stdout, .. }) = result.output {
            contains_once(&stdout, &self.data)
        } else {
            false
        }
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [StdoutContainsOnce]
pub fn stdout_contains_once<Cmd, Arg, Data>(cmd: Cmd, data: Data) -> Box<dyn Check>
where
    Arg: Into<String>,
    Cmd: Into<Vec<Arg>>,
    Data: Into<Vec<u8>>,
{
    StdoutContainsOnce::new(norm_cmd(cmd), data.into()).as_check()
}

/// Checks if stderr output of command contains provided data exactly once
pub struct StderrContainsOnce {
    cmd: Vec<String>,
    data: Vec<u8>,
}

impl StderrContainsOnce {
    const NAME: &'static str = "StderrContainsOnce";

    pub fn new(cmd: Vec<String>, data: Vec<u8>) -> Self {
        Self { cmd, data }
    }
}

impl Check for StderrContainsOnce {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        let result = run(&self.cmd);
        if let Some(ProcessOutput { stderr, .. }) = result.output {
            contains_once(&stderr, &self.data)
        } else {
            false
        }
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [StderrContainsOnce]
pub fn stderr_contains_once<Cmd, Arg, Data>(cmd: Cmd, data: Data) -> Box<dyn Check>
where
    Arg: Into<String>,
    Cmd: Into<Vec<Arg>>,
    Data: Into<Vec<u8>>,
{
    StderrContainsOnce::new(norm_cmd(cmd), data.into()).as_check()
}

/// Checks if file has provided content
pub struct IsFileContent {
    path: PathBuf,
    content: Vec<u8>,
}

impl IsFileContent {
    const NAME: &'static str = "IsFileContent";

    pub fn new(path: PathBuf, content: Vec<u8>) -> Self {
        Self { path, content }
    }
}

impl Check for IsFileContent {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        // todo: check file size equal content size first
        if let Ok(file_content) = std::fs::read(&self.path) {
            file_content == self.content
        } else {
            false
        }
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [IsFileContent]
pub fn is_file_content<FilePath, Content>(path: FilePath, content: Content) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
    Content: Into<Vec<u8>>,
{
    IsFileContent::new(path.into(), content.into()).as_check()
}

/// Checks if file contains provided data exactly once
pub struct FileContainsOnce {
    path: PathBuf,
    data: Vec<u8>,
}

impl FileContainsOnce {
    const NAME: &'static str = "FileContainsOnce";

    pub fn new(path: PathBuf, data: Vec<u8>) -> Self {
        Self { path, data }
    }
}

impl Check for FileContainsOnce {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        if let Ok(file_content) = std::fs::read(&self.path) {
            contains_once(&file_content, &self.data)
        } else {
            false
        }
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [FileContainsOnce]
pub fn file_contains_once<FilePath, Data>(path: FilePath, data: Data) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
    Data: Into<Vec<u8>>,
{
    FileContainsOnce::new(path.into(), data.into()).as_check()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServiceStatus {
    Active,
    Inactive,
    Failed,
}

/// Checks if service has provided status
pub struct IsServiceStatus {
    service: String,
    status: ServiceStatus,
}

impl IsServiceStatus {
    const NAME: &'static str = "IsServiceStatus";

    pub fn new(service: String, status: ServiceStatus) -> Self {
        Self { service, status }
    }
}

impl Check for IsServiceStatus {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        let result = run(&[
            "/usr/bin/systemctl".to_owned(),
            "is-active".to_owned(),
            self.service.clone(),
        ]);
        if let Some(ProcessOutput { stdout, .. }) = result.output {
            let status = String::from_utf8(stdout).unwrap_or("".to_owned());
            let status = status.trim();
            status
                == match self.status {
                    ServiceStatus::Active => "active",
                    ServiceStatus::Inactive => "inactive",
                    ServiceStatus::Failed => "failed",
                }
        } else {
            false
        }
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [IsServiceStatus], checks if service active
pub fn is_service_active<Service>(service: Service) -> Box<dyn Check>
where
    Service: Into<String>,
{
    IsServiceStatus::new(service.into(), ServiceStatus::Active).as_check()
}

/// init [IsServiceStatus], checks if service inactive
pub fn is_service_inactive<Service>(service: Service) -> Box<dyn Check>
where
    Service: Into<String>,
{
    IsServiceStatus::new(service.into(), ServiceStatus::Inactive).as_check()
}

/// init [IsServiceStatus], checks if service failed
pub fn is_service_failed<Service>(service: Service) -> Box<dyn Check>
where
    Service: Into<String>,
{
    IsServiceStatus::new(service.into(), ServiceStatus::Failed).as_check()
}

/// Checks if service is enabled
pub struct IsServiceEnabled {
    service: String,
    is_enabled: bool,
}

impl IsServiceEnabled {
    const NAME: &'static str = "IsServiceEnabled";

    pub fn new(service: String, is_enabled: bool) -> Self {
        Self {
            service,
            is_enabled,
        }
    }
}

impl Check for IsServiceEnabled {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        let result = run(&[
            "/usr/bin/systemctl".to_owned(),
            "is-enabled".to_owned(),
            self.service.clone(),
        ]);
        if let Some(ProcessOutput { stdout, .. }) = result.output {
            let status = String::from_utf8(stdout).unwrap_or("".to_owned());
            let status = status.trim();
            status
                == if self.is_enabled {
                    "enabled"
                } else {
                    "disabled"
                }
        } else {
            false
        }
    }

    fn as_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [IsServiceEnabled], checks if service enabled
pub fn is_service_enabled<Service>(service: Service) -> Box<dyn Check>
where
    Service: Into<String>,
{
    IsServiceEnabled::new(service.into(), true).as_check()
}

/// init [IsServiceEnabled], checks if service disabled
pub fn is_service_disabled<Service>(service: Service) -> Box<dyn Check>
where
    Service: Into<String>,
{
    IsServiceEnabled::new(service.into(), false).as_check()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_checks() {
        assert!(always_ok().yes());
        assert!(!always_fail().yes());
    }

    #[test]
    fn test_named() {
        let c = named("aaa", always_ok());
        assert_eq!(c.name(), "aaa");
        assert!(c.yes());
        let c = named("aaa", always_fail());
        assert!(!c.yes());
    }
}
