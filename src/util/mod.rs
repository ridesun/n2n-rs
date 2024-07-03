use ::std::ffi::CStr;
use std::cmp::Ordering;

use libc::c_char;

use crate::N2NError;

pub mod error;

pub trait ToString {
    fn to_string(&self) -> String;
}

pub trait CopyFromU8Slice {
    fn copy_from_u8_slice(&'_ mut self, slice: &[u8]) -> Result<(), N2NError>;
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
    fn copy_from_u8_slice(&'_ mut self, slice: &[u8]) -> Result<(), N2NError> {
        let len = self.len();
        match len.partial_cmp(&slice.len()) {
            None => {
                unreachable!()
            }
            Some(cmp) => match cmp {
                Ordering::Less => Err(N2NError::CCharLenTooLong(len, slice.len())),
                Ordering::Equal => Ok(()),
                Ordering::Greater => {
                    let a = unsafe { ::core::mem::transmute::<&mut [c_char], &mut [u8]>(self) };
                    let mut temp = slice.to_vec();
                    temp.extend(vec![0u8; len - slice.len()]);
                    a.copy_from_slice(temp.as_slice());
                    Ok(())
                }
            },
        }
    }
}
