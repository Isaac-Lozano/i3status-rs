extern crate i3status;

use i3status::I3Status;
use i3status::block::{Time, NetUsage};

fn main() {
    let time_block = Time::new("%a %F %T");
    let net_block = NetUsage::new();
    let mut status = I3Status::new();

    status.add_block(time_block, "Time");
    status.add_block(net_block, "Net");

    status.run();
}
