pub mod error;

use crate::N2NError;
use ::std::ffi::CStr;
use libc::c_char;
pub trait ToString {
    fn to_string(&self) -> String;
}
pub trait CopyFromU8Slice {
    fn copy_from_u8_slice(self: &'_ mut Self, slice: &[u8]) -> Result<(), N2NError>;
}
impl ToString for [c_char] {
    fn to_string(&self) -> String {
        unsafe {
            let c_str = CStr::from_ptr(self.as_ptr());
            c_str.to_string_lossy().into_owned()
        }
    }
}

impl CopyFromU8Slice for [c_char] {
    fn copy_from_u8_slice(self: &'_ mut Self, slice: &[u8]) -> Result<(), N2NError> {
        let len = self.len();
        if len > slice.len() {
            let a = unsafe { ::core::mem::transmute::<&mut [c_char], &mut [u8]>(self) };
            let mut temp = slice.to_vec();
            temp.extend(vec![0u8; len - slice.len()]);
            a.copy_from_slice(temp.as_slice());
            Ok(())
        } else if len < slice.len() {
            Err(N2NError::CCharLenTooLong(len, slice.len()))
        } else {
            Ok(())
        }
    }
}
