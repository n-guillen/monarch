#[cfg_attr(windows, path = "windows/mod.rs")]
pub mod memory;
use crate::memory::*;

fn main() {
    // Loop until the user selects a process (by id)
    let pid = loop {
        match select_process() {
            Ok(n) => break n,
            Err(_) => clearscreen::clear().unwrap(),
        }
    };

    println!("{:?}", pid);
}
