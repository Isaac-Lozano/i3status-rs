extern crate i3status;

use i3status::I3Status;
use i3status::block::Time;

fn main() {
    let mut block = Time::new();
    let mut status = I3Status::new();

    status.add_block(&mut block, "Test");

    status.run();
}
