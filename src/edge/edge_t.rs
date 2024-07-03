use std::ptr;
use std::ptr::addr_of_mut;

use sys::tuntap_close;

use crate::ffi::n2n_edge_t;
use crate::supernode_connect;

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
    pub unsafe fn wrap(ptr: *mut n2n_edge_t) -> Self {
        Edge { ptr }
    }
}

impl Edge {
    pub fn supernode_connect(&self) -> anyhow::Result<()> {
        unsafe {
            supernode_connect(self.as_mut_ptr());
        }
        Ok(())
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
            tuntap_close(addr_of_mut!((*self.as_mut_ptr()).device));
            ptr::drop_in_place(self.as_mut_ptr())
        }
    }
}

unsafe impl Send for Edge {}

unsafe impl Sync for Edge {}
