use std::mem::MaybeUninit;
use std::ptr::addr_of_mut;

use crate::ffi::tuntap_dev;

pub struct TunTapDev {
    ptr: *mut tuntap_dev,
}
impl TunTapDev {
    pub unsafe fn as_ptr(&self) -> *const tuntap_dev {
        self.ptr as *const _
    }
    pub unsafe fn as_mut_ptr(&self) -> *mut tuntap_dev {
        self.ptr
    }
}
impl TunTapDev {
    pub(crate) fn init() -> Self {
        unsafe {
            let mut uninit = MaybeUninit::<tuntap_dev>::uninit().assume_init();
            let ptr = addr_of_mut!(uninit);
            TunTapDev { ptr }
        }
    }
}
