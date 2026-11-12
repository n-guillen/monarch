#[derive(Debug)]
pub struct Region {
    pub base_address: *mut core::ffi::c_void,
    pub region_size: usize,
}