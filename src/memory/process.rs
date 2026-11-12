use std::io::stdin;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SelectProcessError {
    #[error("There was an issue with finding all processes.")]
    SysInfo(#[from] sysinfo::Error),

    #[error("There was an issue with the user's input.")]
    Input(#[from] std::io::Error),

    #[error("There was an issue parsing the user's input.")]
    ParsingInput(#[from] std::num::ParseIntError),

    #[error("Process of pid {0} is unable to be found.")]
    ProcessNotFound(u32),
}

/// Displays all of the system's processes and allows the user to select one.
pub fn select_process() -> Result<u32, SelectProcessError> {
    let s = sysinfo::System::new_all()?;

    // Only printing the processes that have a root path (temporary preference)
    for (pid, process) in s.processes().iter() {
        if process.root().is_some() {
            println!("{0:⋅<35} {pid}", process.name().display());
        }
    }
    // Read the user's input and then attempt to parse into usize.
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let i: u32 = input.trim().parse()?;

    // Return the pid if it is an actual process, otherwise return error.
    let selected_pid = sysinfo::Pid::from_u32(i);
    if s.process(selected_pid).is_some() {
        Ok(i)
    } else {
        Err(SelectProcessError::ProcessNotFound(i))
    }
}