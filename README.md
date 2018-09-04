# SEGGER RTT Support for Rust

This repo implements support for the Real Time Transfer (RTT) debugger
extensions that are present in J-Link devices produced by SEGGER.

## Using it

Basic logging:

```
extern crate jlink_rtt;

fn boo() {
   let mut output = jlink_rtt::Output::new();
   let _ = writeln!("Hello {}", 42);
}
```

Handling panics:

```
#![no_std]

extern crate panic_rtt;

fn main() {
    panic!("message is logged to debugger");
}
```

## More info

More information on RTT can be found here:
<https://www.segger.com/products/debug-probes/j-link/technology/about-real-time-transfer/>

The author of this repo is not affiliated with SEGGER, nor is
this repo supported by them.

## License

The implementation is derived from code produced by SEGGER Microcontroller GmbH
under BSD-3-Clause license.

