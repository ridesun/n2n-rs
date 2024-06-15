use libc::c_int;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::addr_of_mut;

use crate::{
    ffi::*,
    ffi::{edge_init_conf_defaults, n2n_edge_conf_t},
    N2NError,
};

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
            let mut uninit = MaybeUninit::<n2n_edge_conf_t>::uninit();
            edge_init_conf_defaults(uninit.as_mut_ptr());
            let mut e = uninit.assume_init();
            EdgeConf {
                ptr: addr_of_mut!(e),
            }
        }
    }
    pub fn quick_init() -> Self {
        unsafe {
            let mut conf = EdgeConf::default();
            let ptr = conf.as_mut_ptr();
            (*ptr).allow_p2p = 1;
            (*ptr).allow_routing = 1;
            (*ptr)
                .community_name
                .copy_from_slice(b"mycommunity\000000000");
            (*ptr).disable_pmtu_discovery = 1;
            (*ptr).drop_multicast = 0;
            (*ptr).tuntap_ip_mode = TUNTAP_IP_MODE_SN_ASSIGN as u8;
            let c_str = CString::new("mysecret").unwrap();
            let c_ptr = c_str.as_ptr().cast_mut();
            (*ptr).encrypt_key = c_ptr;
            (*ptr).local_port = 22210;
            (*ptr).mgmt_port = N2N_EDGE_MGMT_PORT as c_int;
            (*ptr).register_interval = 1 as c_int;
            (*ptr).register_ttl = 1 as c_int;
            (*ptr).tos = 16;
            (*ptr).transop_id = n2n_transform_N2N_TRANSFORM_ID_TWOFISH;
            conf
        }
    }
    pub fn get_local_port(&self) -> i32 {
        unsafe { (*self.as_ptr()).local_port }
    }
    pub fn set_local_port(&self, port: i32) -> Result<i32, N2NError> {
        if port < 0 {
            Err(N2NError::I32NegativeToCInt)
        } else {
            unsafe {
                (*self.as_mut_ptr()).local_port = port;
                Ok((*self.as_ptr()).local_port)
            }
        }
    }
    pub fn set_encrypt_key(&self, encrypt_key: &str) -> Result<(), N2NError> {
        unsafe {
            let c_str = CString::new(encrypt_key).unwrap();
            let c_ptr = c_str.into_raw();
            (*self.as_mut_ptr()).encrypt_key = c_ptr;
            Ok(())
        }
    }
}
