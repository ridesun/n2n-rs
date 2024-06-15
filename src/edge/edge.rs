use std::ptr;

use sys::edge_term;

use crate::ffi::n2n_edge_t;

pub struct Edge {
    ptr: *mut n2n_edge_t,
}
impl Edge {
    pub unsafe fn as_ptr(&self) -> *const n2n_edge_t {
        self.ptr as *const _
    }
    pub unsafe fn as_mut_ptr(&self) -> *mut n2n_edge_t {
        self.ptr
    }
}

impl Edge {
    pub unsafe fn wrap(ptr: *mut n2n_edge_t) -> Self {
        Edge { ptr }
    }
    pub fn keep_running(&self, keep_running: *mut bool) {
        unsafe {
            (*self.as_mut_ptr()).keep_running = keep_running;
        }
    }
}
impl Drop for Edge {
    fn drop(&mut self) {
        unsafe {
            edge_term(self.as_mut_ptr());
            ptr::drop_in_place(self.as_mut_ptr())
        }
    }
}
