use std::ffi::CString;
use std::mem::MaybeUninit;

use anyhow::Error;

use sys::{edge_conf_add_supernode, edge_verify_conf, n2n_transform_N2N_TRANSFORM_ID_AES};

use crate::ffi::{edge_init_conf_defaults, n2n_edge_conf_t};
use crate::N2NError;

#[derive(Copy, Clone)]
pub struct EdgeConfBuilder<'a> {
    encrypt_key: &'a str,
    supernode: &'a str,
    community_name: &'a str,
}

impl<'a> EdgeConfBuilder<'a> {
    pub fn new() -> Self {
        EdgeConfBuilder {
            encrypt_key: "",
            supernode: "",
            community_name: "",
        }
    }
    pub fn encrypt_key(&mut self, encrypt_key: &'a str) -> &mut EdgeConfBuilder<'a> {
        self.encrypt_key = encrypt_key;
        self
    }
    pub fn supernode(&mut self, supernode: &'a str) -> &mut EdgeConfBuilder<'a> {
        self.supernode = supernode;
        self
    }
    pub fn community_name(&mut self, community_name: &'a str) -> &mut EdgeConfBuilder<'a> {
        self.community_name = community_name;
        self
    }
    pub fn build(self) -> anyhow::Result<EdgeConf> {
        let mut uninit = MaybeUninit::uninit();
        let key = CString::new(self.encrypt_key).unwrap();
        let key_ptr = key.as_ptr();
        let sn = CString::new(self.supernode).unwrap();
        let supernode_ptr = sn.as_ptr();
        unsafe {
            edge_init_conf_defaults(uninit.as_mut_ptr());
            (*uninit.as_mut_ptr()).transop_id = n2n_transform_N2N_TRANSFORM_ID_AES;
            let mut e = uninit.assume_init();
            e.encrypt_key = libc::strdup(key_ptr);
            e.community_name[..self.community_name.len()]
                .copy_from_slice(self.community_name.as_bytes());
            let conf = EdgeConf {
                ptr: Box::into_raw(Box::new(e)),
            };
            edge_conf_add_supernode(conf.as_mut_ptr(), supernode_ptr);
            match edge_verify_conf(conf.as_mut_ptr()) {
                0 => Ok(conf),
                -1 => Err(Error::from(N2NError::CommunityNameNull)),
                -2 => Err(Error::from(N2NError::SnNumIsZero)),
                _ => Err(Error::from(N2NError::UnKnown)),
            }
        }
    }
}

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
