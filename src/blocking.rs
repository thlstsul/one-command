use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Execute commands on the Windows platform,
/// without opening a window to maintain consistency with other system behaviors.
pub struct Command;

impl Command {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: AsRef<OsStr>>(program: S) -> std::process::Command {
        #[cfg(windows)]
        {
            let mut inner = std::process::Command::new(program);
            inner.creation_flags(CREATE_NO_WINDOW);
            inner
        }
        #[cfg(not(windows))]
        std::process::Command::new(program)
    }
}
