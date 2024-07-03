use std::mem::MaybeUninit;

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
            let uninit = MaybeUninit::<tuntap_dev>::uninit();
            let init = uninit.assume_init();
            let ptr = Box::into_raw(Box::new(init));
            TunTapDev { ptr }
        }
    }
}
