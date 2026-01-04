#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Memory::*;

#[cfg(target_os = "windows")]
pub fn mem_reserve(size: usize) -> *mut u8 {
    unsafe {
        VirtualAlloc(std::ptr::null_mut(), size, MEM_RESERVE, PAGE_READWRITE) as *mut u8
    }
}

#[cfg(target_os = "windows")]
pub fn mem_commit(ptr: *mut u8, size: usize) -> bool {
    unsafe {
        let ret = VirtualAlloc(ptr as _, size, MEM_COMMIT, PAGE_READWRITE); 
        !ret.is_null()
    }
}

#[cfg(target_os = "windows")]
pub fn get_pagesize() -> usize {
    unsafe {
        // create a blank 'system info' struct
        let mut sys_info = std::mem::zeroed::<windows_sys::Win32::System::SystemInformation::SYSTEM_INFO>();
        // ask daddy windows to fill it up for us 
        windows_sys::Win32::System::SystemInformation::GetSystemInfo(&mut sys_info);
        // return the page size field
        sys_info.dwPageSize as usize
    }
}
