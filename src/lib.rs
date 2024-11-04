use std::ffi::OsStr;

pub mod blocking;

/// Execute commands on the Windows platform,
/// without opening a window to maintain consistency with other system behaviors.
pub struct Command;

impl Command {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: AsRef<OsStr>>(program: S) -> async_process::Command {
        blocking::Command::new(program).into()
    }
}
