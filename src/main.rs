#[cfg_attr(windows, path = "windows/mod.rs")]
pub mod memory;
use crate::memory::*;

fn main() {
    let pid = select_process();
    dbg!(pid);
}