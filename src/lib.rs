#![feature(libc)]
extern crate libc;
use std::ffi::CStr;
use std::str;

#[link(name = "groonga")]
extern {
    pub fn grn_init() -> i64;
    pub fn grn_ctx_open(rc: i64) -> *mut libc::c_void;
    pub fn grn_ctx_close(ctx: *mut libc::c_void) -> i64;
    pub fn grn_db_open(ctx: *mut libc::c_void, dbpath: *mut libc::c_char) -> *mut libc::c_void;
    pub fn grn_db_create(ctx: *mut libc::c_void, dbpath: *mut libc::c_char,
                         flag: Option<extern "C" fn(libc::c_int) -> libc::c_int>) -> *mut libc::c_void;
    pub fn grn_ctx_send(ctx: *mut libc::c_void, command: libc::c_char, command_length: libc::c_int,
                        flag: libc::c_int) -> i64;
    pub fn grn_ctx_recv(ctx: *mut libc::c_void, command: *mut libc::c_char, command_length: *mut libc::c_int,
                        flag: *mut libc::c_int) -> i64;
    pub fn grn_get_version() -> *mut libc::c_char;
    pub fn grn_fin() -> i64;
}

pub fn groonga_init() -> *mut libc::c_void {
    unsafe {
        let rc = grn_init();
        return grn_ctx_open(rc);
    }
}

pub fn groonga_fin(ctx: *mut libc::c_void) {
    unsafe {
        let _ = grn_ctx_close(ctx);
        let _ = grn_fin();
    }
}

pub fn get_groonga_version() -> &'static str {
    unsafe {
        let slice = CStr::from_ptr(grn_get_version());
        return str::from_utf8(slice.to_bytes()).unwrap();
    }
}
