use windows_sys::Win32::Foundation::{CloseHandle, STILL_ACTIVE};
use windows_sys::Win32::System::Threading::{
    GetExitCodeProcess, OpenProcess, TerminateProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE,
};

pub struct ProcessProbe;

impl ProcessProbe {
    pub fn isAlive(pid: u32) -> bool {
        if pid == 0 {
            return false;
        }
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if handle.is_null() {
                return false;
            }
            let mut exitCode: u32 = 0;
            let queried = GetExitCodeProcess(handle, &mut exitCode) != 0;
            CloseHandle(handle);
            queried && exitCode == STILL_ACTIVE as u32
        }
    }

    pub fn terminate(pid: u32) -> bool {
        unsafe {
            let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
            if handle.is_null() {
                return false;
            }
            let terminated = TerminateProcess(handle, 0) != 0;
            CloseHandle(handle);
            terminated
        }
    }
}
