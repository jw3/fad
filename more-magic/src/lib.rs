use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct FileInfo {
    _data: [u8; 0],
    _marker:
    core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// todo;; should come from limits.h
const PATH_MAX: usize = 4096;

extern {
    fn file_init();
    fn file_close();

    fn get_file_type_from_fd(fd: i32, info: *const FileInfo, path: *const c_char,
                             sz: usize, buf: *mut c_char) -> *mut c_char;

    fn stat_file_entry(fd: i32) -> *const FileInfo;
}

pub fn initialize() {
    unsafe { file_init() }
}
pub fn destroy() {
    unsafe { file_close() }
}

pub fn get_ftype(fd: i32, path: &str) -> String {
    let finfo = unsafe { stat_file_entry(fd) };
    if finfo.is_null() {
        return "??".to_string()
    }

    let path = CString::new(path).unwrap();
    let mut buf = vec![0; PATH_MAX + 1];
    let res = unsafe { get_file_type_from_fd(fd, finfo, path.as_c_str().as_ptr(), buf.len(), buf.as_mut_ptr()) };
    if res.is_null() {
        return "?".to_string()
    }

    unsafe { CStr::from_ptr(buf.as_mut_ptr()).to_str().unwrap().to_string() }
}
