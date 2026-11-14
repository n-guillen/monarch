use crate::memory::value::read::{collect_regions, pid_to_handle};

#[cfg_attr(windows, path = "read/windows.rs")]
pub mod read;

#[allow(non_camel_case_types)]
pub enum ValueType {
    u32,
    i32,
}

impl ValueType{
    pub fn size(&self) -> usize {
        match self {
            ValueType::u32 => size_of::<u32>(),
            ValueType::i32 => size_of::<i32>(),
        }
    }
}

pub enum ScanMode {
    Initial(ValueType, CurrentScan),
    Continous(ValueType, EitherScan, Vec<Address>),
}

pub enum EitherScan {
    Current(CurrentScan),
    Last(LastScan),
}

pub enum CurrentScan {
    Exact(Vec<u8>),
    BiggerThan(Vec<u8>),
    SmallerThan(Vec<u8>),
    Unknown,
}

pub enum LastScan {
    Increased,
    Decreased,
    Changed,
    Unchanged,
}

// pub trait Scannable {
//     fn evaluate(&self, data: Vec<u8>) -> bool;
// }

impl CurrentScan {
    fn evaluate(&self, value_type: &ValueType, address_data: Vec<u8>) -> bool {
        match self {
            CurrentScan::Exact(value) => *value == address_data,
            CurrentScan::BiggerThan(value_data) => {
                match value_type {
                    ValueType::u32 => u32::from_le_bytes(address_data.try_into().unwrap()) > u32::from_le_bytes(value_data.clone().try_into().unwrap()),
                    ValueType::i32 => i32::from_le_bytes(address_data.try_into().unwrap()) > i32::from_le_bytes(value_data.clone().try_into().unwrap()),
                }
            },
            CurrentScan::SmallerThan(value_data) => {
                match value_type {
                    ValueType::u32 => u32::from_le_bytes(address_data.try_into().unwrap()) < u32::from_le_bytes(value_data.clone().try_into().unwrap()),
                    ValueType::i32 => i32::from_le_bytes(address_data.try_into().unwrap()) < i32::from_le_bytes(value_data.clone().try_into().unwrap()),
                }
            },
            CurrentScan::Unknown => true,
        }
    }
}

impl LastScan {
    fn evaluate(&self, value_type: &ValueType, address_data: Vec<u8>, last_data: Vec<u8>) -> bool {
        match self {
            LastScan::Increased => {
                match value_type {
                    ValueType::u32 => u32::from_le_bytes(address_data.try_into().unwrap()) > u32::from_le_bytes(last_data.try_into().unwrap()),
                    ValueType::i32 => i32::from_le_bytes(address_data.try_into().unwrap()) > i32::from_le_bytes(last_data.try_into().unwrap()),
                }
            },
            LastScan::Decreased => {
                match value_type {
                    ValueType::u32 => u32::from_le_bytes(address_data.try_into().unwrap()) < u32::from_le_bytes(last_data.try_into().unwrap()),
                    ValueType::i32 => i32::from_le_bytes(address_data.try_into().unwrap()) < i32::from_le_bytes(last_data.try_into().unwrap()),
                }
            },
            LastScan::Changed => address_data != last_data,
            LastScan::Unchanged => address_data == last_data,
        }
    }
}

#[derive(Debug)]
pub struct Address {
    base_address: usize,
    size: usize,
    data: Vec<u8>,
}

// TO-DO: Consider memchr crate's memmem?
// https://varik.dev/blog/cheat-engine-rs
pub fn scan(pid: u32, mode: ScanMode) -> Option<Vec<Address>> {
    let handle  = pid_to_handle(pid).ok()?;
    let mut scanned_memory: Vec<Address> = vec![];
    
    match mode {
        ScanMode::Initial(value_type, scan) => {
            for region in collect_regions(&handle).iter() {
                let buffer = region.to_bytes(&handle);
                let size = value_type.size();
                
                for (i, data) in buffer.chunks(size).enumerate() {
                    if scan.evaluate(&value_type, data.to_vec()) {
                        let base_address = region.base_address.wrapping_add(i * size) as usize;
                        scanned_memory.push(Address { base_address, size, data: data.to_vec()});
                    }
                }
            }
        },

        ScanMode::Continous(value_type, scan_type, scanned_addresses) => {

            for address in scanned_addresses {
                let Some(address_data) = address.to_bytes(&handle) else {continue};
                let original_data = address_data.clone();
                
                let valid = match &scan_type {
                    EitherScan::Current(scan) => scan.evaluate(&value_type, address_data),
                    EitherScan::Last(scan) => scan.evaluate(&value_type, address_data, address.data),
                };

                if valid { scanned_memory.push(Address { base_address: address.base_address, size: address.size, data: original_data }); }
            }
            
        },
    }
    

    if scanned_memory.is_empty() {
        None
    } else {
        Some(scanned_memory)
    }
}