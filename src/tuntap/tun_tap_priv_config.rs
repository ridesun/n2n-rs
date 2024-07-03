use std::mem::MaybeUninit;

use libc::c_int;

use sys::{
    n2n_tuntap_priv_config_t, DEFAULT_MTU, N2N_EDGE_DEFAULT_DEV_NAME, N2N_EDGE_DEFAULT_NETMASK,
    N2N_MACNAMSIZ,
};

use crate::util::*;

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
        let all0 = [0i8; N2N_MACNAMSIZ as usize];
        unsafe {
            let uninit = MaybeUninit::<n2n_tuntap_priv_config_t>::uninit();
            let init = uninit.assume_init();
            let boxed = Box::into_raw(Box::new(init));

            (*boxed).mtu = DEFAULT_MTU as c_int;
            (*boxed).daemon = 1;
            (*boxed)
                .tuntap_dev_name
                .copy_from_u8_slice(N2N_EDGE_DEFAULT_DEV_NAME)
                .unwrap();
            (*boxed)
                .netmask
                .copy_from_u8_slice(N2N_EDGE_DEFAULT_NETMASK)
                .unwrap();

            //make sure zero
            (*boxed).device_mac.copy_from_slice(&all0);
            (*boxed).ip_mode = [0i8; 16];
            TunTapPrivConfig { ptr: boxed }
        }
    }
}
