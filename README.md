# Neutron Star Runtime

This project, neutron-star-rt, implements the most low level of details to build a smart contract in the Neutron system. It should normally not be used directly, and instead only used through the neutron-star abstraction layer. 

# Example Minimal Contract

main.rs:

```rs
#![no_main]
#![no_std]

use neutron_star_rt::*;

extern crate panic_halt;


#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    let msg = "Hello World!";
    __push_costack(msg.as_ptr(), msg.len());
    let count = 1 as u8;
    let count_ptr: *const u8 = &count;
    __push_costack(count_ptr, 1);
    __system_call(4, 2);
    __exit(5);
}
```

Cargo.toml:

```
[package]
name = "neutron-star-test"
version = "0.1.0"
authors = ["earlz <earlz@earlz.net>"]
edition = "2018"

[[bin]]
name = "neutron-star-test"
test = false
bench = false

[dependencies]
neutron-star-rt = { path = "../neutron-star-rt" }
panic-halt = "0.2.0"
```

.cargo/config:

```
[target.thumbv6m-none-eabi]

[target.'cfg(all(target_arch = "arm", target_os = "none"))']

rustflags = [

  # LLD (shipped with the Rust toolchain) is used as the default linker
  "-C", "link-arg=-Tlink.x",
  "-C", "relocation-model=static",
  "-C", "target-feature=+crt-static"

  # if you run into problems with LLD switch to the GNU linker by commenting out
  # this line
  # "-C", "linker=arm-none-eabi-ld",

  # if you need to link to pre-compiled C libraries provided by a C toolchain
  # use GCC as the linker by commenting out both lines above and then
  # uncommenting the three lines below
  # "-C", "linker=arm-none-eabi-gcc",
  # "-C", "link-arg=-Wl,-Tlink.x",
  # "-C", "link-arg=-nostartfiles",
]

[build]
target = "thumbv6m-none-eabi"
```