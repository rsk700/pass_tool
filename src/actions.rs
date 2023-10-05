use crate::interfaces::{Action, ActionResult};

/// [Action] which does nothing and always succeed
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

// Action_Named = struct {
// Action_DoNothing = struct {
// Action_Constant = struct {
// Action_Many = struct {
// Run?
// Action_RunProcess = struct {
// Action_InstallAptPackages = struct {
// Action_DeleteFile = struct {
// Action_WriteFile = struct {
// Action_CreateDir = struct {
// Action_SetFilePermissions = struct {
// Action_ReplaceInFileOnce = struct {
// Action_RenameDir = struct {
// Action_ServiceCommand = struct {


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
