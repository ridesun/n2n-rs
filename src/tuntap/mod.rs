pub mod tuntap_dev;

use crate::ffi::*;
use crate::tuntap::tuntap_dev::TunTapDev;
use crate::util::*;
use libc::c_int;
use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::addr_of_mut;

pub(crate) struct TunTapPrivConfig {
    ptr: *mut n2n_tuntap_priv_config_t,
}
impl TunTapPrivConfig {
    pub unsafe fn as_ptr(&self) -> *const n2n_tuntap_priv_config_t {
        self.ptr as *const _
    }
    pub unsafe fn as_mut_ptr(&self) -> *mut n2n_tuntap_priv_config_t {
        self.ptr
    }
}
impl TunTapPrivConfig {
    pub fn default() -> Self {
        unsafe {
            let mut uninit = MaybeUninit::<n2n_tuntap_priv_config_t>::uninit();
            let ptr = addr_of_mut!(*uninit.as_mut_ptr());
            let t = TunTapPrivConfig { ptr };
            (*t.as_mut_ptr()).mtu = DEFAULT_MTU as c_int;
            (*t.as_mut_ptr()).daemon = 1;
            (*t.as_mut_ptr())
                .tuntap_dev_name
                .copy_from_u8_slice(N2N_EDGE_DEFAULT_DEV_NAME)
                .unwrap();
            (*t.as_mut_ptr())
                .netmask
                .copy_from_u8_slice(N2N_EDGE_DEFAULT_NETMASK)
                .unwrap();
            t
        }
    }
    pub fn show(&self) {
        let mtu = (unsafe { *self.as_ptr() }).mtu;
        let daemon = (unsafe { *self.as_ptr() }).daemon;
        let dev_name = (unsafe { *self.as_ptr() }).tuntap_dev_name.to_string();
        let netmask = (unsafe { *self.as_ptr() }).netmask.to_string();
        println!(
            "mtu is {},and daemon is {},dev_name is {},netmask is {}",
            mtu, daemon, dev_name, netmask
        );
    }
}
impl Drop for TunTapDev {
    fn drop(&mut self) {
        unsafe {
            tuntap_close(self.as_mut_ptr());
            ptr::drop_in_place(self.as_mut_ptr());
            println!("drop");
        }
    }
}

#[test]
fn test() {
    let t = TunTapPrivConfig::default();
    t.show();
}
