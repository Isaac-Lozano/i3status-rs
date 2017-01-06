//! Block for measuring and displaying net usage.
extern crate libc;

use std::ptr;
use std::ffi::CStr;

use block::{Block, Status, Color};
use std::time::Duration;

#[derive(Debug)]
pub enum SockAddr
{
    BLAH,
}

#[derive(Debug)]
pub struct IFAddrs
{
    pub name: String,
    pub flags: u32,
    pub addr: Option<SockAddr>,
    pub stats: Option<rtnl_link_stats>,
}

impl IFAddrs
{
    pub fn get() -> Vec<IFAddrs>
    {
        let mut addrs = Vec::new();
        unsafe
        {
            let mut addr_list = ptr::null_mut::<libc::ifaddrs>();
            libc::getifaddrs(&mut addr_list);

            let mut iter = addr_list;
            while !(*iter).ifa_next.is_null()
            {
                let addr = IFAddrs::from_ptr(iter);
                addrs.push(addr);

                iter = (*iter).ifa_next;
            }

            libc::freeifaddrs(addr_list);
        }
        addrs
    }

    unsafe fn from_ptr(addr: *mut libc::ifaddrs) -> IFAddrs
    {
        let name = CStr::from_ptr((*addr).ifa_name).to_string_lossy().into_owned();
        let flags: u32 = (*addr).ifa_flags.clone();
        let stats;

        if (*addr).ifa_data.is_null()
        {
            stats = None;
        }
        else
        {
            stats = Some(ptr::read(((*addr).ifa_data as *const rtnl_link_stats)).clone());
        }

        IFAddrs
        {
            name: name,
            flags: flags,
            addr: None,
            stats: stats,
        }
    }
}

#[derive(Debug,Clone)]
#[repr(C)]
pub struct rtnl_link_stats
{
    pub rx_packets: u32,        /* total packets received */
    pub tx_packets: u32,        /* total packets transmitted */
    pub rx_bytes: u32,        /* total bytes received */
    pub tx_bytes: u32,        /* total bytes transmitted */
    pub rx_errors: u32,        /* bad packets received */
    pub tx_errors: u32,        /* packet transmit problems */
    pub rx_dropped: u32,        /* no space in linux buffers */
    pub tx_dropped: u32,        /* no space available in linux */
    pub multicast: u32,        /* multicast packets received */
    pub collisions: u32,

    /* detailed rx_errors: */
    pub rx_length_errors: u32,
    pub rx_over_errors: u32,        /* receiver ring buff overflow */
    pub rx_crc_errors: u32,        /* recved pkt with crc error */
    pub rx_frame_errors: u32,    /* recv'd frame alignment error */
    pub rx_fifo_errors: u32,        /* recv'r fifo overrun */
    pub rx_missed_errors: u32,    /* receiver missed packet */

    /* detailed tx_errors */
    pub tx_aborted_errors: u32,
    pub tx_carrier_errors: u32,
    pub tx_fifo_errors: u32,
    pub tx_heartbeat_errors: u32,
    pub tx_window_errors: u32,

    /* for cslip etc */
    pub rx_compressed: u32,
    pub tx_compressed: u32,
}

#[derive(Debug)]
pub struct NetUsage
{
    last_up: u64,
    last_down: u64,
    up: u64,
    down: u64,
}

impl NetUsage
{
    pub fn new() -> NetUsage
    {
        NetUsage
        {
            last_up: 0,
            last_down: 0,
            up: 0,
            down: 0,
        }
    }
}


impl Block for NetUsage
{
    fn update(&mut self) -> Duration
    {
        let ifaddrs = IFAddrs::get();

        let mut cur_up = 0u64;
        let mut cur_down = 0u64;
        for addr in ifaddrs
        {
            match addr.stats
            {
                Some(rtnl) =>
                {
                    cur_down += rtnl.rx_bytes as u64;
                    cur_up += rtnl.tx_bytes as u64;
                }
                None => {}
            }
        }

        self.up = cur_up - self.last_up;
        self.down = cur_down - self.last_down;

        self.last_up = cur_up;
        self.last_down = cur_down;

        Duration::new(3, 0)
    }

    fn get_status(&self) -> Status
    {
        let mut status = Status::new(format!("↑ {} - ↓ {}", format_bytes(self.up/3), format_bytes(self.down/3)));
//        status.color = Some(Color(((self.down / 3) * 255 / 5000000) as u8, (255 - ((self.down / 3) * 255 / 5000000)) as u8, 30));
        status
    }

    fn click_callback(&mut self)
    {
    }
}

fn format_bytes(bytes: u64) -> String
{
    if bytes < 1000
    {
        format!("{} B/s", bytes)
    }
    else if bytes < 1000000
    {
        format!("{:.1} KB/s", bytes as f64 / 1000.0)
    }
    else if bytes < 1000000000
    {
        format!("{:.1} MB/s", bytes as f64 / 1000000.0)
    }
    else
    {
        format!("{:.1} GB/s", bytes as f64 / 1000000000.0)
    }
}
