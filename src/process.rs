use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    SuccessOnExit,
    ErrorOnExit,
    FailOnStart,
}

pub struct ProcessOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub struct ProcessResult {
    pub code: ExitCode,
    pub output: Option<ProcessOutput>,
}

impl ProcessResult {
    pub fn fail_on_start() -> Self {
        Self {
            code: ExitCode::FailOnStart,
            output: None,
        }
    }

    pub fn ok(&self) -> bool {
        self.code == ExitCode::SuccessOnExit
    }
}

// todo: try `Arg: AsRef<str>`
pub fn norm_cmd<Cmd, Arg>(cmd: Cmd) -> Vec<String>
where
    Arg: Into<String>,
    Cmd: Into<Vec<Arg>>,
{
    cmd.into().into_iter().map(|c| c.into()).collect()
}

pub fn run(cmd: &[String]) -> ProcessResult {
    let Some((cmd, args)) = cmd.split_first() else {
        return ProcessResult::fail_on_start();
    };
    let Ok(output) = Command::new(cmd).args(args).output() else {
        return ProcessResult::fail_on_start();
    };
    if output.status.success() {
        ProcessResult {
            code: ExitCode::SuccessOnExit,
            output: Some(ProcessOutput {
                stdout: output.stdout,
                stderr: output.stderr,
            }),
        }
    } else {
        ProcessResult {
            code: ExitCode::ErrorOnExit,
            output: Some(ProcessOutput {
                stdout: output.stdout,
                stderr: output.stderr,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        {
            let result = run(&norm_cmd(["echo", "1"]));
            matches!(result.code, ExitCode::SuccessOnExit);
            assert!(result.ok());
            let Some(output) = result.output else {
                panic!("expecting output");
            };
            assert_eq!(output.stdout, "1\n".as_bytes());
        }
        {
            let result = run(&norm_cmd(["false"]));
            matches!(result.code, ExitCode::ErrorOnExit);
            assert!(!result.ok());
        }
        {
            let result = run(&norm_cmd(["aaabbb-not-a-command-bbbaaa"]));
            matches!(result.code, ExitCode::FailOnStart);
            assert!(!result.ok());
        }
    }
}
