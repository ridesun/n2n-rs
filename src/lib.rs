pub extern crate n2n_sys as sys;

use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use libc::c_int;

use sys::{
    edge_conf_add_supernode, edge_init, quick_edge_init, run_edge_loop, tuntap_open, DEFAULT_MTU,
};

use crate::edge::edge::Edge;
use crate::edge::edge_conf::EdgeConf;
use crate::tuntap::tuntap_dev::TunTapDev;
pub use crate::{sys as ffi, util::error::N2NError};

mod edge;
mod tuntap;
pub mod util;

pub fn init() -> anyhow::Result<()> {
    let conf = EdgeConf::quick_init();
    let supernode = CString::new("121.37.42.224:22210").unwrap();
    let tundev = TunTapDev::init();
    unsafe {
        let supernode_ptr = supernode.as_ptr();
        edge_conf_add_supernode(conf.as_mut_ptr(), supernode_ptr);
        let dev = CString::new("edge0").unwrap();
        let mode = CString::new("dhcp").unwrap();
        let ip = CString::new("10.200.175.165").unwrap();
        let mask = CString::new("255.255.255.0").unwrap();
        let mac = CString::new("DE:AD:BE:EF:01:10").unwrap();
        let _ = tuntap_open(
            tundev.as_mut_ptr(),
            dev.as_ptr().cast_mut(),
            mode.as_ptr(),
            ip.as_ptr().cast_mut(),
            mask.as_ptr().cast_mut(),
            mac.as_ptr(),
            DEFAULT_MTU as c_int,
            0,
        );
        let mut rc = 1;
        let rc_ptr: *mut c_int = &mut rc;
        let eee_ptr = edge_init(conf.as_mut_ptr(), rc_ptr);
        let eee = Edge::wrap(eee_ptr);
        let mut keep_running = AtomicBool::new(true);
        let kr_ptr: *mut bool = keep_running.get_mut();
        let kill = thread::spawn(move || {
            println!("end");
            thread::sleep(Duration::from_secs(10));
            keep_running.swap(false, Ordering::Relaxed);
        });
        kill.join().unwrap();
        eee.keep_running(kr_ptr);
        rc = run_edge_loop(eee.as_mut_ptr());
    }
    Ok(())
}

pub fn test() -> anyhow::Result<()> {
    unsafe {
        let supernode = CString::new("121.37.42.224:22210").unwrap().into_raw();
        let dev = CString::new("edge0").unwrap().into_raw();
        let ip = CString::new("10.200.175.165").unwrap().into_raw();
        let mac = CString::new("DE:AD:BE:EF:01:10").unwrap().into_raw();
        let c = CString::new("test").unwrap().into_raw();
        let s = CString::new("secret").unwrap().into_raw();
        let mut keep_running = true;
        let kr_ptr: *mut bool = &mut keep_running;
        let _ = quick_edge_init(dev, c, s, mac, ip, supernode, kr_ptr);
    }
    Ok(())
}
#[test]
fn t() -> anyhow::Result<()> {
    init()?;
    Ok(())
}
