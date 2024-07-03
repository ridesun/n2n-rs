pub extern crate n2n_sys as sys;

use std::ffi::CString;
use std::mem::{size_of, MaybeUninit};
use std::ptr::{addr_of_mut, null_mut};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};

use libc::{c_int, fd_set, time_t, timeval, FD_ISSET, FD_SET, FD_ZERO};

use sys::{
    edge_conf_add_supernode, edge_init, edge_term, edge_term_conf, n2n_edge_t,
    n2n_resolve_parameter_t, n2n_seed, n2n_srand, n2n_transform_N2N_TRANSFORM_ID_AES,
    print_edge_stats, run_edge_loop, tuntap_open, BOOTSTRAP_TIMEOUT, N2N_SN_PKTBUF_SIZE,
    NUMBER_SN_PINGS_INITIAL, SWEEP_TIME, TUNTAP_IP_MODE_SN_ASSIGN,
};

use crate::edge::edge_conf::EdgeConf;
use crate::edge::edge_t::Edge;
use crate::tuntap::tun_tap_priv_config::TunTapPrivConfig;
use crate::tuntap::tuntap_dev::TunTapDev;
pub use crate::{sys as ffi, util::error::N2NError};

mod edge;
mod tuntap;
pub mod util;

pub struct EdgeJob {
    pub main_job: JoinHandle<()>,
    pub keep: Arc<AtomicBool>,
}

impl EdgeJob {
    pub fn new(sr: &str, sn: &str, community_name: &str) -> anyhow::Result<Self> {
        //FOR TRACKING
        use sys::{setTraceLevel, TRACE_DEBUG};
        unsafe {
            setTraceLevel(TRACE_DEBUG as c_int);
        }

        //init n2n_edge_conf_t,n2n_tuntap_priv_config_t and tuntap_dev
        let conf = EdgeConf::default();
        let ec = TunTapPrivConfig::default();
        let tuntap = TunTapDev::init();

        let mut position = 0;
        let mut expected = size_of::<u16>() as u16;
        let mut pktbuf = vec![0u8; N2N_SN_PKTBUF_SIZE as usize + size_of::<u16>()];
        let mut now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

        random();

        let mut rc = 0;
        let keep = Arc::new(AtomicBool::new(false));
        let keep_clone = Arc::clone(&keep);
        let mut runlevel = 2;
        let mut seek_answer = 1;
        let main_job = {
            unsafe {
                let conf_ptr = conf.as_mut_ptr();
                (*conf_ptr).transop_id = n2n_transform_N2N_TRANSFORM_ID_AES;
                let key = CString::new(sr).unwrap();
                let key_ptr = key.as_ptr();
                let sn = CString::new(sn).unwrap();
                let supernode_ptr = sn.as_ptr();
                (*conf_ptr).community_name[..community_name.len()]
                    .copy_from_slice(community_name.as_bytes());
                (*conf_ptr).encrypt_key = libc::strdup(key_ptr);
                edge_conf_add_supernode(conf.as_mut_ptr(), supernode_ptr);

                let rc_ptr: *mut c_int = &mut rc;
                let eee = Edge::wrap(edge_init(conf.as_mut_ptr(), rc_ptr));

                (*eee.as_mut_ptr()).tuntap_priv_conf = ec.as_mut_ptr().read();
                (*eee.as_mut_ptr()).conf.tuntap_ip_mode = TUNTAP_IP_MODE_SN_ASSIGN as u8;

                (*eee.as_mut_ptr()).last_sup = 0;
                (*eee.as_mut_ptr()).curr_sn = (*eee.as_mut_ptr()).conf.supernodes;
                eee.supernode_connect()?;

                let uninit_wait_time = MaybeUninit::<timeval>::uninit();
                let init_wait_time = uninit_wait_time.assume_init();
                let wait_time = Box::into_raw(Box::new(init_wait_time));
                let uninit_socket_mask = MaybeUninit::<fd_set>::uninit();
                let init_socket_mask = uninit_socket_mask.assume_init();
                let socket_mask = Box::into_raw(Box::new(init_socket_mask));

                while runlevel < 5 {
                    now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
                    if (*eee.as_ptr()).sn_pong != 0 {
                        (*eee.as_mut_ptr()).sn_pong = 0;
                    }
                    match runlevel {
                        2 => {
                            (*eee.as_mut_ptr()).sn_wait = 1;
                            send_register_super(eee.as_mut_ptr());
                            runlevel += 1;
                        }
                        3 => {
                            if (*eee.as_ptr()).sn_wait == 0 {
                                runlevel += 1;
                            }
                        }
                        4 => {
                            let _ = tuntap_open(
                                tuntap.as_mut_ptr(),
                                (*eee.as_mut_ptr())
                                    .tuntap_priv_conf
                                    .tuntap_dev_name
                                    .as_mut_ptr(),
                                (*eee.as_mut_ptr()).tuntap_priv_conf.ip_mode.as_ptr(),
                                (*eee.as_mut_ptr()).tuntap_priv_conf.ip_addr.as_mut_ptr(),
                                (*eee.as_mut_ptr()).tuntap_priv_conf.netmask.as_mut_ptr(),
                                (*eee.as_mut_ptr()).tuntap_priv_conf.device_mac.as_ptr(),
                                (*eee.as_mut_ptr()).tuntap_priv_conf.mtu,
                                (*eee.as_mut_ptr()).tuntap_priv_conf.metric,
                            );
                            (*eee.as_mut_ptr()).device = tuntap.as_mut_ptr().read();
                            runlevel = 5;
                            seek_answer = 0;
                        }
                        _ => {}
                    }
                    if seek_answer != 0 {
                        FD_ZERO(socket_mask);
                        FD_SET((*eee.as_mut_ptr()).sock, socket_mask);
                        (*wait_time).tv_sec = BOOTSTRAP_TIMEOUT as time_t;
                        (*wait_time).tv_usec = 0;

                        if libc::select(
                            (*eee.as_mut_ptr()).sock + 1,
                            socket_mask,
                            null_mut(),
                            null_mut(),
                            wait_time,
                        ) > 0
                        {
                            if FD_ISSET((*eee.as_mut_ptr()).sock, socket_mask) {
                                fetch_and_eventually_process_data(
                                    eee.as_mut_ptr(),
                                    (*eee.as_mut_ptr()).sock,
                                    pktbuf.as_mut_ptr(),
                                    addr_of_mut!(expected),
                                    addr_of_mut!(position),
                                    now,
                                );
                            }
                        }
                    }
                    seek_answer = 1;
                    resolve_check((*eee.as_mut_ptr()).resolve_parameter, 0, now);
                }

                (*eee.as_mut_ptr()).conf.number_max_sn_pings = NUMBER_SN_PINGS_INITIAL as u8;
                (*eee.as_mut_ptr()).last_sweep =
                    now - (SWEEP_TIME as i64) + 2 * (BOOTSTRAP_TIMEOUT as i64);
                (*eee.as_mut_ptr()).sn_wait = 1;
                (*eee.as_mut_ptr()).last_register_req = 0;
                thread::spawn(move || {
                    let keep_ptr = keep.as_ptr();
                    eee.keep_running(keep_ptr);
                    run_edge_loop(eee.as_mut_ptr());
                    print_edge_stats(eee.as_mut_ptr());
                    edge_term_conf(addr_of_mut!((*eee.as_mut_ptr()).conf));
                    edge_term(eee.as_mut_ptr());
                })
            }
        };
        Ok(EdgeJob {
            main_job,
            keep: keep_clone,
        })
    }
}

fn random() {
    unsafe {
        n2n_srand(n2n_seed());
    }
}

extern "C" {
    pub(crate) fn supernode_connect(eee: *mut n2n_edge_t);
    fn send_register_super(eee: *mut n2n_edge_t);
    fn fetch_and_eventually_process_data(
        eee: *mut n2n_edge_t,
        sock: libc::c_int,
        pktbuf: *mut u8,
        expected: *mut u16,
        position: *mut u16,
        now: time_t,
    );
    fn resolve_check(param: *mut n2n_resolve_parameter_t, resolution_request: u8, now: time_t);
}
