pub mod memory;
use crate::memory::process::*;
use crate::memory::value::read::collect_regions;
use crate::memory::value::*;

fn main() {
    // Loop until the user selects a process (by id)
    let pid = loop {
        match select_process() {
            Ok(n) => break n,
            Err(_) => clearscreen::clear().unwrap(),
        }
    };
    
    // scan(pid, ScanType::ExactValue(Value::u32(1166686085)), None);
}
