pub mod memory;
use text_io::read;

use crate::memory::process::*;
use crate::memory::value::*;

fn main() {
    // Loop until the user selects a process (by id)
    let pid = loop {
        match select_process() {
            Ok(n) => break n,
            Err(_) => clearscreen::clear().unwrap(),
        }
    };
    
    let mut value: u32 = read!();
    const VALUE_TYPE: ValueType = ValueType::u32;
    
    let mut mode = ScanMode::Initial(VALUE_TYPE, CurrentScan::Exact(value.to_le_bytes().to_vec()));
    let mut scanned = scan(pid, mode);

    let _ = loop {
        println!("{:?}", &scanned);
        value = read!();
        
        match scanned {
            Some(addresses) => {
                println!("{:?}", &addresses.len());
        
                if addresses.len() < 2 { break addresses; }
                
                // mode = ScanMode::Continuous(VALUE_TYPE, EitherScan::Current(CurrentScan::Exact(value.to_le_bytes().to_vec())), addresses);
                mode = ScanMode::Continuous(VALUE_TYPE, EitherScan::Last(LastScan::Decreased), addresses);
            }
            None => mode = ScanMode::Initial(VALUE_TYPE, CurrentScan::Exact(value.to_le_bytes().to_vec())),
        };

        scanned = scan(pid, mode);
    };
}