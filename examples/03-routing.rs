#![cfg_attr(feature = "alloc", feature(alloc_error_handler))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

// - bare metal dependencies --------------------------------------------------

#[cfg(feature = "alloc")]
mod allocator;

#[cfg(not(feature = "std"))]
use ockam::{
    compat::string::{String, ToString},
    println
};

#[cfg(feature = "cortexm")]
use panic_semihosting as _;

#[cfg(feature = "qemu")]
use cortex_m_semihosting::debug;

#[cfg(feature = "atsame54")]
use atsame54_xpro as _;

#[cfg(feature = "stm32f4")]
use stm32f4xx_hal as _;

// - bare metal entrypoint ----------------------------------------------------

#[cfg(feature = "cortexm")]
#[cortex_m_rt::entry]
fn entry() -> ! {
    #[cfg(feature = "alloc")]
    allocator::init();

    // execute ockam::node main function
    main().unwrap();

    // exit qemu
    #[cfg(feature = "qemu")]
    {
        debug::exit(debug::EXIT_SUCCESS);
    }

    loop { }
}

// - ockam::node entrypoint ---------------------------------------------------

use talk_rustlondon21::{Echoer, Hop};
use ockam::{route, Context, Result};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Start a worker, of type Echoer, at address "echoer"
    ctx.start_worker("echoer", Echoer).await?;

    // Start a worker, of type Hop, at address "h1"
    ctx.start_worker("h1", Hop).await?;

    // Send a message to the worker at address "echoer",
    // via the worker at address "h1"
    ctx.send(route!["h1", "echoer"], "Hello Ockam!".to_string()).await?;

    // Wait to receive a reply and print it.
    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply); // should print "Hello Ockam!"

    // Stop all workers, stop the node, cleanup and return.
    ctx.stop().await
}
