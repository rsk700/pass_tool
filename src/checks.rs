use crate::{
    interfaces::Check,
    pattern::Pattern,
    process::{norm_cmd, run, ExitCode, ProcessOutput},
};
use nix::unistd::Uid;
use std::{fs::OpenOptions, path::PathBuf};

/// Check which always `true`
pub struct AlwaysYes;

impl AlwaysYes {
    const NAME: &'static str = "AlwaysYes";
}

impl Check for AlwaysYes {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        true
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [AlwaysYes]
pub fn always_yes() -> Box<dyn Check> {
    AlwaysYes.into_check()
}

/// Check which always `false`
pub struct AlwaysNo;

impl AlwaysNo {
    const NAME: &'static str = "AlwaysNo";
}

impl Check for AlwaysNo {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        false
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [AlwaysNo]
pub fn always_no() -> Box<dyn Check> {
    AlwaysNo.into_check()
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

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [Named]
pub fn check<Name>(name: Name, check: Box<dyn Check>) -> Box<dyn Check>
where
    Name: Into<String>,
{
    Named::new(name.into(), check).into_check()
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
    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [UserIsRoot]
pub fn user_is_root() -> Box<dyn Check> {
    UserIsRoot.into_check()
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

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [IsFile]
pub fn is_file<FilePath>(path: FilePath) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
{
    IsFile::new(path.into()).into_check()
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

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [IsDir]
pub fn is_dir<FilePath>(path: FilePath) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
{
    IsDir::new(path.into()).into_check()
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

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [CanRead]
pub fn can_read<FilePath>(path: FilePath) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
{
    CanRead::new(path.into()).into_check()
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

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [CanWrite]
pub fn can_write<FilePath>(path: FilePath) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
{
    CanWrite::new(path.into()).into_check()
}

/// Checks if provided path is missing (no such file or directory)
pub struct PathIsMissing(PathBuf);

impl PathIsMissing {
    const NAME: &'static str = "PathIsMissing";

    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

impl Check for PathIsMissing {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        if let Ok(y) = self.0.try_exists() {
            !y
        } else {
            // check failed, not possible to answer if path is missing
            false
        }
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [PathIsMissing]
pub fn path_is_missing<P>(path: P) -> Box<dyn Check>
where
    P: Into<PathBuf>,
{
    PathIsMissing::new(path.into()).into_check()
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
        if self.checks.is_empty() {
            true
        } else {
            self.checks.iter().any(|c| c.yes())
        }
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [OrOp]
pub fn or_op<Checks>(checks: Checks) -> Box<dyn Check>
where
    Checks: Into<Vec<Box<dyn Check>>>,
{
    OrOp::new(checks.into()).into_check()
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

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [AndOp]
pub fn and_op<Checks>(checks: Checks) -> Box<dyn Check>
where
    Checks: Into<Vec<Box<dyn Check>>>,
{
    AndOp::new(checks.into()).into_check()
}

/// Checks if stdout output of the command contains provided pattern exactly
/// once
pub struct StdoutContainsOnce {
    cmd: Vec<String>,
    pattern: Pattern,
}

impl StdoutContainsOnce {
    const NAME: &'static str = "StdoutContainsOnce";

    pub fn new(cmd: Vec<String>, pattern: Pattern) -> Self {
        Self { cmd, pattern }
    }
}

impl Check for StdoutContainsOnce {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        let result = run(&self.cmd);
        if let Some(ProcessOutput { stdout, .. }) = result.output {
            self.pattern.contains_once(&stdout).unwrap_or_default()
        } else {
            false
        }
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [StdoutContainsOnce]
pub fn stdout_contains_once<TCmd, TArg, TPattern>(cmd: TCmd, pattern: TPattern) -> Box<dyn Check>
where
    TArg: Into<String>,
    TCmd: Into<Vec<TArg>>,
    TPattern: Into<Pattern>,
{
    StdoutContainsOnce::new(norm_cmd(cmd), pattern.into()).into_check()
}

/// Checks if stderr output of the command contains provided pattern exactly once
pub struct StderrContainsOnce {
    cmd: Vec<String>,
    pattern: Pattern,
}

impl StderrContainsOnce {
    const NAME: &'static str = "StderrContainsOnce";

    pub fn new(cmd: Vec<String>, pattern: Pattern) -> Self {
        Self { cmd, pattern }
    }
}

impl Check for StderrContainsOnce {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        let result = run(&self.cmd);
        if let Some(ProcessOutput { stderr, .. }) = result.output {
            self.pattern.contains_once(&stderr).unwrap_or_default()
        } else {
            false
        }
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [StderrContainsOnce]
pub fn stderr_contains_once<TCmd, TArg, TPattern>(cmd: TCmd, pattern: TPattern) -> Box<dyn Check>
where
    TArg: Into<String>,
    TCmd: Into<Vec<TArg>>,
    TPattern: Into<Pattern>,
{
    StderrContainsOnce::new(norm_cmd(cmd), pattern.into()).into_check()
}

/// Checks if stdout of the command contains no provided pattern
pub struct StdoutLacks {
    cmd: Vec<String>,
    pattern: Pattern,
}

impl StdoutLacks {
    const NAME: &'static str = "StdoutLacks";

    pub fn new(cmd: Vec<String>, pattern: Pattern) -> Self {
        Self { cmd, pattern }
    }
}

impl Check for StdoutLacks {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        let result = run(&self.cmd);
        if let Some(ProcessOutput { stdout, .. }) = result.output {
            self.pattern
                .contains(&stdout)
                .map(|y| !y)
                .unwrap_or_default()
        } else {
            false
        }
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [StdoutLacks]
pub fn stdout_lacks<TCmd, TArg, TPattern>(cmd: TCmd, pattern: TPattern) -> Box<dyn Check>
where
    TArg: Into<String>,
    TCmd: Into<Vec<TArg>>,
    TPattern: Into<Pattern>,
{
    StdoutLacks::new(norm_cmd(cmd), pattern.into()).into_check()
}

/// Checks if stderr of the command contains no provided pattern
pub struct StderrLacks {
    cmd: Vec<String>,
    pattern: Pattern,
}

impl StderrLacks {
    const NAME: &'static str = "StderrLacks";

    pub fn new(cmd: Vec<String>, pattern: Pattern) -> Self {
        Self { cmd, pattern }
    }
}

impl Check for StderrLacks {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        let result = run(&self.cmd);
        if let Some(ProcessOutput { stderr, .. }) = result.output {
            self.pattern
                .contains(&stderr)
                .map(|y| !y)
                .unwrap_or_default()
        } else {
            false
        }
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [StderrLacks]
pub fn stderr_lacks<TCmd, TArg, TPattern>(cmd: TCmd, pattern: TPattern) -> Box<dyn Check>
where
    TArg: Into<String>,
    TCmd: Into<Vec<TArg>>,
    TPattern: Into<Pattern>,
{
    StderrLacks::new(norm_cmd(cmd), pattern.into()).into_check()
}

/// Checks if command returns provided [ExitCode] after execution
pub struct IsExitCode {
    cmd: Vec<String>,
    exit_code: ExitCode,
}

impl IsExitCode {
    const NAME: &'static str = "IsExitCode";

    pub fn new(cmd: Vec<String>, exit_code: ExitCode) -> Self {
        Self { cmd, exit_code }
    }
}

impl Check for IsExitCode {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        run(&self.cmd).code == self.exit_code
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [IsExitCode], checks if exit code is [ExitCode::SuccessOnExit]
pub fn command_ok<TCmd, TArg>(cmd: TCmd) -> Box<dyn Check>
where
    TArg: Into<String>,
    TCmd: Into<Vec<TArg>>,
{
    IsExitCode::new(norm_cmd(cmd), ExitCode::SuccessOnExit).into_check()
}

/// init [IsExitCode], checks if exit code is [ExitCode::ErrorOnExit]
pub fn command_err<TCmd, TArg>(cmd: TCmd) -> Box<dyn Check>
where
    TArg: Into<String>,
    TCmd: Into<Vec<TArg>>,
{
    IsExitCode::new(norm_cmd(cmd), ExitCode::ErrorOnExit).into_check()
}

/// init [IsExitCode], checks if exit code is [ExitCode::FailOnStart]
pub fn command_fail<TCmd, TArg>(cmd: TCmd) -> Box<dyn Check>
where
    TArg: Into<String>,
    TCmd: Into<Vec<TArg>>,
{
    IsExitCode::new(norm_cmd(cmd), ExitCode::FailOnStart).into_check()
}

/// Checks if file matches exactly with provided content
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

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [IsFileContent]
pub fn is_file_content<FilePath, Content>(path: FilePath, content: Content) -> Box<dyn Check>
where
    FilePath: Into<PathBuf>,
    Content: Into<Vec<u8>>,
{
    IsFileContent::new(path.into(), content.into()).into_check()
}

/// Checks if file contains provided pattern exactly once
pub struct FileContainsOnce {
    path: PathBuf,
    pattern: Pattern,
}

impl FileContainsOnce {
    const NAME: &'static str = "FileContainsOnce";

    pub fn new(path: PathBuf, pattern: Pattern) -> Self {
        Self { path, pattern }
    }
}

impl Check for FileContainsOnce {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn yes(&self) -> bool {
        if let Ok(file_content) = std::fs::read(&self.path) {
            self.pattern
                .contains_once(&file_content)
                .unwrap_or_default()
        } else {
            false
        }
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [FileContainsOnce]
pub fn file_contains_once<TPath, TPattern>(path: TPath, data: TPattern) -> Box<dyn Check>
where
    TPath: Into<PathBuf>,
    TPattern: Into<Pattern>,
{
    FileContainsOnce::new(path.into(), data.into()).into_check()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServiceStatus {
    Active,
    Inactive,
    Failed,
}

/// Checks if service has provided status
pub struct ServiceInStatus {
    service: String,
    status: ServiceStatus,
}

impl ServiceInStatus {
    const NAME: &'static str = "ServiceInStatus";

    pub fn new(service: String, status: ServiceStatus) -> Self {
        Self { service, status }
    }
}

impl Check for ServiceInStatus {
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

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [ServiceInStatus], checks if service active
pub fn service_is_active<Service>(service: Service) -> Box<dyn Check>
where
    Service: Into<String>,
{
    ServiceInStatus::new(service.into(), ServiceStatus::Active).into_check()
}

/// init [ServiceInStatus], checks if service inactive
pub fn service_is_inactive<Service>(service: Service) -> Box<dyn Check>
where
    Service: Into<String>,
{
    ServiceInStatus::new(service.into(), ServiceStatus::Inactive).into_check()
}

/// init [ServiceInStatus], checks if service failed
pub fn service_is_failed<Service>(service: Service) -> Box<dyn Check>
where
    Service: Into<String>,
{
    ServiceInStatus::new(service.into(), ServiceStatus::Failed).into_check()
}

/// Checks if service is enabled
pub struct ServiceIsEnabled {
    service: String,
    is_enabled: bool,
}

impl ServiceIsEnabled {
    const NAME: &'static str = "ServiceIsEnabled";

    pub fn new(service: String, is_enabled: bool) -> Self {
        Self {
            service,
            is_enabled,
        }
    }
}

impl Check for ServiceIsEnabled {
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

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

/// init [ServiceIsEnabled], checks if service enabled
pub fn service_is_enabled<Service>(service: Service) -> Box<dyn Check>
where
    Service: Into<String>,
{
    ServiceIsEnabled::new(service.into(), true).into_check()
}

/// init [ServiceIsEnabled], checks if service disabled
pub fn service_is_disabled<Service>(service: Service) -> Box<dyn Check>
where
    Service: Into<String>,
{
    ServiceIsEnabled::new(service.into(), false).into_check()
}

/// Implements [Check] for tuple with name and function
impl<N, F> Check for (N, F)
where
    N: AsRef<str> + 'static,
    F: Fn() -> bool + 'static,
{
    fn name(&self) -> &str {
        self.0.as_ref()
    }

    fn yes(&self) -> bool {
        self.1()
    }

    fn into_check(self) -> Box<dyn Check> {
        Box::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pattern::re;
    use std::path::Path;

    const NOT_A_FILE: &str = "/tmp/not-a-pass-test-file-5555555555";

    fn create_test_file(name: &str) -> String {
        let path = format!("/tmp/pass-test-file-111222333-{}", name);
        std::fs::write(&path, "aaabbbccc").unwrap();
        path
    }

    fn delete_test_file<P>(path: P)
    where
        P: AsRef<Path>,
    {
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_always_yes() {
        assert!(always_yes().yes());
    }

    #[test]
    fn test_always_no() {
        assert!(always_yes().yes());
    }

    #[test]
    fn test_named() {
        let c = check("aaa", always_yes());
        assert_eq!(c.name(), "aaa");
        assert!(c.yes());
        let c = check("aaa", always_no());
        assert!(!c.yes());
    }

    #[test]
    fn test_user_is_root() {
        // use `test_user_is_root` example for manual testing
    }

    #[test]
    fn test_is_file() {
        let path = create_test_file("is_file");
        assert!(is_file(&path).yes());
        delete_test_file(path);
        assert!(!is_file(NOT_A_FILE).yes());
        assert!(!is_file("/tmp").yes());
    }

    #[test]
    fn test_is_dir() {
        let path = create_test_file("is_dir");
        assert!(!is_dir(&path).yes());
        delete_test_file(path);
        assert!(is_dir("/tmp").yes());
        assert!(!is_dir("/tmp111111111111111").yes());
    }

    #[test]
    fn test_can_read() {
        let path = create_test_file("can_read");
        assert!(can_read(&path).yes());
        delete_test_file(path);
        assert!(!can_read(NOT_A_FILE).yes());
    }

    #[test]
    fn test_can_write() {
        let path = create_test_file("can_write");
        assert!(can_write(&path).yes());
        delete_test_file(path);
        assert!(!can_write(NOT_A_FILE).yes());
    }

    #[test]
    fn test_path_is_missing() {
        assert!(path_is_missing("/tmp111111111111111122222222222").yes());
        assert!(!path_is_missing("/tmp").yes());
    }

    #[test]
    fn test_or_op() {
        assert!(or_op([]).yes());
        assert!(or_op([always_yes(), always_yes()]).yes());
        assert!(or_op([always_yes(), always_no()]).yes());
        assert!(!or_op([always_no(), always_no()]).yes());
    }

    #[test]
    fn test_and_op() {
        assert!(and_op([]).yes());
        assert!(and_op([always_yes(), always_yes()]).yes());
        assert!(!and_op([always_yes(), always_no()]).yes());
        assert!(!and_op([always_no(), always_no()]).yes());
    }

    #[test]
    fn test_stdout_contains_once() {
        assert!(stdout_contains_once(["echo", "111222333"], "23").yes());
        assert!(!stdout_contains_once(["echo", "1112222"], "22").yes());
        assert!(!stdout_contains_once(["echo", "1112222"], "44").yes());
        assert!(stdout_contains_once(["echo", "111222333"], re("1.2")).yes());
        assert!(!stdout_contains_once(["echo", "111222333"], re("1.")).yes());
    }

    #[test]
    fn test_stderr_contains_once() {
        assert!(stderr_contains_once(["ls", NOT_A_FILE], "cannot access").yes());
        assert!(!stderr_contains_once(["ls", NOT_A_FILE], "c").yes());
        assert!(!stderr_contains_once(["ls", NOT_A_FILE], "11111111111111111").yes());
        assert!(
            stderr_contains_once(["ls", "/tmp/not-a-pass-test-file-1111222333"], re("1.2{3}"))
                .yes()
        );
        assert!(
            !stderr_contains_once(["ls", "/tmp/not-a-pass-test-file-1111222333"], re("1{2}")).yes()
        );
    }

    #[test]
    fn test_stdout_lacks() {
        assert!(stdout_lacks(["echo", "111222333"], "000").yes());
        assert!(!stdout_lacks(["echo", "111222333"], "111").yes());
        assert!(!stdout_lacks(["eeeeeeeeeeeeeeeeeeeeerrrrrr123"], "111").yes());
        assert!(stdout_lacks(["echo", "111222333"], re("1{4}")).yes());
        assert!(!stdout_lacks(["echo", "111222333"], re("1{3}")).yes());
    }

    #[test]
    fn test_stderr_lacks() {
        let path = "/tmp/not-a-pass-test-file-5555555555-111222333-test_stderr_lacks";
        assert!(stderr_lacks(["ls", path], "000").yes());
        assert!(!stderr_lacks(["ls", path], "111").yes());
        assert!(!stderr_lacks(["eeeeeeeeeeeeeeeeeerrrrrrrrrrrrrrrrrr123"], "111").yes());
        assert!(stderr_lacks(["ls", path], re("1{4}")).yes());
        assert!(!stderr_lacks(["ls", path], re("1{3}")).yes());
    }

    #[test]
    fn test_is_exit_code() {
        assert!(command_ok(["/bin/true"]).yes());
        assert!(command_err(["/bin/false"]).yes());
        assert!(command_fail(["/bin/command-which-does-not-exist-5kGHx7FDlwolHKpmJQim9X"]).yes());
    }

    #[test]
    fn test_is_file_content() {
        let path = create_test_file("is_file_content");
        assert!(is_file_content(&path, "aaabbbccc").yes());
        assert!(!is_file_content(&path, "111").yes());
        delete_test_file(&path);
    }

    #[test]
    fn test_file_contains_once() {
        let path = create_test_file("file_contains_once");
        assert!(file_contains_once(&path, "bbb").yes());
        assert!(!file_contains_once(&path, "b").yes());
        assert!(file_contains_once(&path, re("a.ab{3}")).yes());
        assert!(!file_contains_once(&path, re("a.")).yes());
        delete_test_file(&path);
    }

    #[test]
    fn test_service_in_status() {
        // use `test_service_in_status` example for manual testing
    }

    #[test]
    fn test_service_is_enabled() {
        // use `test_service_is_enabled` example for manual testing
    }

    #[test]
    fn test_tuple_as_check() {
        {
            let c = ("t1", || true).into_check();
            assert_eq!(c.name(), "t1");
            assert!(c.yes());
        }
        {
            let c = ("t2", || false).into_check();
            assert!(!c.yes());
        }
        {
            let c = ("t3".to_owned(), || true).into_check();
            assert!(c.yes());
        }
    }
}
