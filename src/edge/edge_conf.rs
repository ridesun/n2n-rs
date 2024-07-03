use std::mem::MaybeUninit;

use crate::ffi::{edge_init_conf_defaults, n2n_edge_conf_t};

#[derive(Eq, PartialEq, Debug)]
pub struct EdgeConf {
    ptr: *mut n2n_edge_conf_t,
}

impl EdgeConf {
    pub unsafe fn as_ptr(&self) -> *const n2n_edge_conf_t {
        self.ptr as *const _
    }
    pub unsafe fn as_mut_ptr(&self) -> *mut n2n_edge_conf_t {
        self.ptr
    }
}

impl EdgeConf {
    pub fn default() -> Self {
        unsafe {
            let mut uninit = MaybeUninit::uninit();
            edge_init_conf_defaults(uninit.as_mut_ptr());
            let e = uninit.assume_init();
            EdgeConf {
                ptr: Box::into_raw(Box::new(e)),
            }
        }
    }
}
