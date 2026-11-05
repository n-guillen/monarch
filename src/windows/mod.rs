use sysinfo::System;

// pub enum ScanValues {
//     Unsigned32BitInteger,
// }

#[derive(Debug)]
pub struct ProcessNotFoundError(usize);
impl std::fmt::Display for ProcessNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to find process under the index {}", self.0)
    }
}
impl std::error::Error for ProcessNotFoundError {}

pub fn select_process() -> Result<u32, anyhow::Error> {
    let s = System::new_all()?;
    for (i, (pid, process)) in s.processes().iter().enumerate() {
        println!("({i}) {pid} - {:#?}", process.name());
    }
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let i: usize = input.trim().parse()?;
    
    let (selected_pid, _) = s.processes().iter().nth(i).ok_or(ProcessNotFoundError(i))?;
    Ok(selected_pid.as_u32())
}