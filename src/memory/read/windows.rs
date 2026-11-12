use std::ptr;
use windows::Win32::{Foundation::HANDLE, System::{Diagnostics::Debug::ReadProcessMemory, Memory::{VIRTUAL_ALLOCATION_TYPE, VirtualQueryEx}, Threading::{OpenProcess, PROCESS_ALL_ACCESS}}};
use thiserror::Error;
use windows::Win32::System::Memory::MEMORY_BASIC_INFORMATION;

use crate::memory::region::Region;

impl From<MEMORY_BASIC_INFORMATION> for Region {
    fn from(value: MEMORY_BASIC_INFORMATION) -> Self {
        Region { base_address: value.BaseAddress, region_size: value.RegionSize }
    }
}

#[derive(Error, Debug)]
#[error("There was an issue finding the process handle.")]
pub struct HandleError(#[from] windows::core::Error);

pub fn pid_to_handle(pid: u32) -> Result<HANDLE, HandleError> {
    let handle  = unsafe {OpenProcess(PROCESS_ALL_ACCESS, false, pid)}?;
    Ok(handle)
}

pub fn collect_regions(handle: &HANDLE) -> Vec<Region> {
    let mut base_address = ptr::null_mut();
    let mut buffer: MEMORY_BASIC_INFORMATION = MEMORY_BASIC_INFORMATION::default();
    
    let mut commited_addresses: Vec<Region> = vec![];
    while unsafe{
        VirtualQueryEx(*handle, Some(base_address), &mut buffer, size_of::<MEMORY_BASIC_INFORMATION>())
    } != 0 {
        if buffer.State == VIRTUAL_ALLOCATION_TYPE(4096) {
            commited_addresses.push(Region::from(buffer));
        }

        base_address = base_address.wrapping_add(buffer.RegionSize);
    }

    commited_addresses
}

impl Region {
    pub fn memory_buffer(&self, handle: &HANDLE) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; self.region_size];

        match unsafe {
            ReadProcessMemory(*handle, self.base_address, buffer.as_mut_ptr().cast(), self.region_size, None)
        } {
            Ok(_) => buffer,
            Err(_) => vec![], // Strange design choice, I'll need to re-consider later.
        }
    }
}