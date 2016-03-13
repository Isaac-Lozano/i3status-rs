# i3status-rs
An i3bar compliant program written in rust.

This library provides support for creating custom status bars for i3bar.

Blocks are run independent of each other and can be set to run at different
intervals from each other.

Running shell scripts through this interface and click callbacks will
eventually be supported.

# Examples
The basic rundown for usage of this library is:

* Create a I3Status object through `I3Status::new()`
* Add blocks (Either from the library, or custom-coded) with `.add_block()`
* call `.run()`

A simple code example would be:

```
extern crate i3status;
use i3status::I3Status;
use i3status::block::Time;

fn main()
{
    let mut time_block = Time::new();
    let mut i3s = I3Status::new();
    i3s.add_block(&mut time_block, "Time");
    i3s.run();
}
```

This will output something similar to `Sun Mar 13 14:37:31 2016` and will
update once a second.

# Custom Blocks
If you want to create a custom status block, you will have to create a
struct and impl `Block`.

Care should be taken that the bulk of the processing take place within
the `update` function and that little processing is done during the
`get_status` function. This is because the `get_status` function is
currently being called whenever *any* block is updated.
