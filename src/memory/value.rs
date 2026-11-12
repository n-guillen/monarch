use crate::memory::value::read::{collect_regions, pid_to_handle};

#[cfg_attr(windows, path = "read/windows.rs")]
pub mod read;

#[allow(non_camel_case_types)]
pub enum Value {
    u32(u32),
}

impl Value {
    pub fn offset(&self) -> usize {
        match self {
            Value::u32(_) => 4,
        }
    }
    
    pub fn matching(&self, bytes: &[u8]) -> bool {
        match self {
            Value::u32(n) => {
                *n == u32::from_le_bytes(*bytes.as_array().unwrap())
            },
        }
    }
}

pub enum ScanType {
    ExactValue(Value),
    // UnknownValue,
}

// TO-DO: Consider memchr crate's memmem?
// https://varik.dev/blog/cheat-engine-rs
pub fn scan(pid: u32, scan_type: ScanType, found: Option<Vec<u32>>) -> Option<Vec<u32>> {
    let handle  = pid_to_handle(pid).ok()?;
    
    let ScanType::ExactValue(value) = scan_type else {
        return None; // TO-DO: Remove this.
    };
    
    for region in collect_regions(&handle).iter() {
        let buffer = region.memory_buffer(&handle);
        
        for (i, data) in buffer.chunks(value.offset()).enumerate() {
            if value.matching(data) {
                println!("{:?}", region.base_address.wrapping_add(i*value.offset()) as u64);
            }
        }
    }

    None
}