extern crate i3status;

use i3status::I3Status;
use i3status::block::{Time, NetUsage};

fn main() {
    let mut time_block = Time::new();
    let mut net_block = NetUsage::new();
    let mut status = I3Status::new();

    status.add_block(&mut time_block, "Time");
    status.add_block(&mut net_block, "Net");

    status.run();
}
