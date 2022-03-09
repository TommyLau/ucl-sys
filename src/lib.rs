#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use std::mem;
use std::os::raw::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// initializes libucl
///
/// call this once before calling any other function in this package
///
/// # Panics
/// If initialization failed for some reason, this function will panic.
pub fn ucl_init() -> c_int {
    let res = unsafe {
        __ucl_init2(
            UCL_VERSION,
            mem::size_of::<c_short>() as i32,
            mem::size_of::<c_int>() as i32,
            mem::size_of::<c_long>() as i32,
            mem::size_of::<u32>() as i32,
            mem::size_of::<c_uint>() as i32,
            -1i32,
            mem::size_of::<*mut u8>() as i32,
            mem::size_of::<*mut c_void>() as i32,
            mem::size_of::<*mut c_void>() as i32, // function ptr
        )
    };
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const IN_LEN: usize = 128 * 1024;
        const OUT_LEN: usize = IN_LEN + IN_LEN / 8 + 256;
        assert_eq!(ucl_init(), UCL_E_OK as i32, "ucl_init failed!");

        let mut in_: Vec<u8> = vec![0; IN_LEN];
        let mut out_: Vec<u8> = vec![0; OUT_LEN];

        let in_length: ucl_uint = IN_LEN as u32;
        let mut out_length: ucl_uint = OUT_LEN as u32;
        unsafe { ucl_memset(in_.as_mut_ptr() as *mut c_void, 0, in_length) };

        let r = unsafe {
            ucl_nrv2b_99_compress(
                in_.as_ptr(),
                in_length,
                out_.as_mut_ptr(),
                &mut out_length,
                std::ptr::null_mut(),
                5,
                std::ptr::null(),
                std::ptr::null_mut(),
            )
        };
        assert_eq!(r, UCL_E_OK as i32, "ucl_nrv2b_99_compress failed!");
        assert!(out_length < in_length, "out_len >= in_len");

        let mut new_length: ucl_uint = 0;
        let r = unsafe {
            ucl_nrv2b_decompress_8(
                out_.as_ptr(),
                out_length,
                in_.as_mut_ptr(),
                &mut new_length,
                std::ptr::null_mut(),
            )
        };
        assert_eq!(r, UCL_E_OK as i32, "ucl_nrv2b_decompress_8 failed!");
        assert!(out_length < new_length, "out_len >= new_len");
    }
}
