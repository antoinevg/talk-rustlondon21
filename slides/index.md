### Building the Internet Of (Trusted) Things with Ockam & Embedded Rust

<!-- ![[images/ockam_rust.png|256]] -->

<img src="images/ockam_rust.png" width=256 />

Antoine van Gelder @ Ockam

---

### About me

* Systems programmer from Cape Town
* Rust since the weekend of 9 February 2019 (formerly `alt.c++.recovery`)
* Working on Ockam and Embedded Rust
* [@antoinevg](https://github.com/antoinevg)

---

### Embedded C / C ++

* All the standard things aren't!
* Every chip vendor ships their own:
  - forked Eclipse
  - forked gcc, gdb, ld
  - forked libc / libstdc++
  - Runtime / Peripheral API's

---

### Embedded Rust

* Use your favorite editor / IDE
* Unified toolchain
  - arm
  - riscv
  - x86
  - xtensa
  - ...and many many more
* Unified debugging: [https://probe.rs/](https://probe.rs/)

---

### What is `no_std` ?

* no `std` library
* only [`core`](https://doc.rust-lang.org/core/)
  - no heap allocation
  - no collections
  - no threads
  - no i/o

---

### Ockam on `no_std`: no heap or collections

* **cheat:** [`alloc`](https://doc.rust-lang.org/alloc/)
  - user just needs to provide an allocator for their target
  - heap allocation
  - collections

---

### Ockam on `no_std`: no threads

* **cheat:** Ockam is already a modern Rust library that supports `async/await`
* 1x Chonky refactor:
  - don't spawn any threads
  - never block execution
  - bonus: reduction in runtime-library size

---

### Ockam on `no_std`: async/await

* [embassy-rs](https://github.com/embassy-rs/embassy/)
  - nrf, stm32, rp2040
  - including i/o and peripheral support!

**... but our embedded partner is MicroChip!**

---

### `ockam_executor`

* light-weight ~ 4k
* minimal dependencies
* no i/o support (!)

---

### Ockam on `no_std`: firmware size

* When we started our Ockam end-to-end example compiled down to:

```
2 545 220 bytes (release build!)
```

---

### Ockam on `no_std`: firmware size

* Switching to `no_std` brought it down to:

```
1 146 060 bytes
```

---

### Ockam on `no_std`: firmware size

* After re-organizing ockam crates: (tx @sanjo!)

```
630 544 bytes
```

---

### Ockam on `no_std`: firmware size

* After compile flags:

```toml
[profile.release]
debug = true
incremental = false
lto = true
opt-level = "z"
codegen-units = 1
```

---

### Ockam on `no_std`: firmware size

* After compile flags:

```
424 592 bytes
```

---

### Future work:

* replace software crypto with hardware

---

### Ockam on `no_std`: ram

```
Example: 03-routing-many-hops

tokio-rs: 568 k
ockam_executor: 64 k
```

---

### Future work:

* optimize `ockam_executor` garbage collection
* `defmt` for logging / tracing

---

### Demo

---

```rust
use talk_rustlondon21::{Echoer, Hop};
use ockam::{route, Context, Result};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    ctx.start_worker("echoer", Echoer).await?;
    ctx.start_worker("h1", Hop).await?;
    ctx.send(route!["h1", "echoer"], "Hello Ockam!".to_string()).await?;

    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply); // should print "Hello Ockam!"

    ctx.stop().await
}
```

---

### bare metal attributes

```rust
#![cfg_attr(feature = "alloc", feature(alloc_error_handler))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]
```

---

### bare metal dependencies

```rust
#[cfg(feature = "alloc")]
mod allocator;
```

---

### bare metal dependencies

```rust
#[cfg(not(feature = "std"))]
use ockam::{
    compat::string::{String, ToString},
    println
};
```

---

### bare metal dependencies

```rust
#[cfg(feature = "qemu")]
use cortex_m_semihosting::debug;

#[cfg(feature = "cortexm")]
use panic_semihosting as _;
```

---

### bare metal dependencies

```rust
#[cfg(feature = "atsame54")]
use atsame54_xpro as _;

#[cfg(feature = "stm32f4")]
use stm32f4xx_hal as _;
```

---

### bare metal entry point

```rust
#[cfg(feature = "cortexm")]
#[cortex_m_rt::entry]
fn entry() -> ! {
    #[cfg(feature = "alloc")]
    allocator::init();

    main().unwrap();

    #[cfg(feature = "qemu")]
    debug::exit(debug::EXIT_SUCCESS);

    loop { }
}
```

---

### Bonus

* Run the same code on server with tokio _and_ ockam_executor!

        cargo run --example hello
        cargo run --example hello --no-default-features --features="alloc, no_std"

---

### Questions

---

### Building the Internet Of (Trusted) Things with Ockam & Embedded Rust

<!-- ![[images/ockam_rust.png|256]] -->

<img src="images/ockam_rust.png" width=256 />

Antoine van Gelder @ Ockam
