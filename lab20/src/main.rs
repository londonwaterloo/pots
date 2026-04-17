#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    #[cfg(all(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    unsafe extern "C" {
        pub unsafe fn opendir(s: *const c_char) -> *mut DIR;
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;
        pub unsafe fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::ptr;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        let c_path = CString::new(path)
            .map_err(|_| format!("Путь содержит нулевой байт: {path}"))?;

        let dir = unsafe { ffi::opendir(c_path.as_ptr()) };

        if dir.is_null() {
            Err(format!("Не удалось открыть каталог: {path}"))
        } else {
            Ok(DirectoryIterator { path: c_path, dir })
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;

    fn next(&mut self) -> Option<OsString> {
        if self.dir.is_null() {
            return None;
        }

        let entry = unsafe { ffi::readdir(self.dir) };

        if entry.is_null() {
            return None;
        }

        let name = unsafe {
            let name_ptr = (*entry).d_name.as_ptr();
            let c_str = CStr::from_ptr(name_ptr);
            OsStr::from_bytes(c_str.to_bytes()).to_os_string()
        };

        Some(name)
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        if !self.dir.is_null() {
            unsafe {
                ffi::closedir(self.dir);
            }
            self.dir = ptr::null_mut();
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("файлы: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}