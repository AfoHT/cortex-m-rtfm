//! [compile-pass] check that `#[cfg]` attributes applied on resources work
//!

#![no_main]
#![no_std]

use panic_halt as _;

//#[cfg(rustc_is_nightly)]
#[rtfm::app(device = lm3s6965)]
const APP: () = {
        struct Resources {
            // A resource
            #[init(0)]
            shared: u32,

            // A conditionally compiled resource behind feature_x
            #[cfg(feature = "feature_x")]
            x: u32,

            dummy: (),
        }

        #[init]
        fn init(_: init::Context) -> init::LateResources {
            init::LateResources {
                // The feature needs to be applied everywhere x is defined or used
                #[cfg(feature = "feature_x")]
                x: 0,
                dummy: () // dummy such that we have at least one late resource
            }
        }

        #[idle]
        fn idle(_cx: idle::Context) -> ! {
            //debug::exit(debug::EXIT_SUCCESS);

            loop {}
        }

        // `shared` can be accessed from this context
        #[task(binds = UART0, resources = [#[cfg(feature = "feature_x")] x, shared])]
        fn uart0(cx: uart0::Context) {
            let shared: &mut u32 = cx.resources.shared;
            *shared += 1;

            //hprintln!("UART0: shared = {}", shared).unwrap();

            #[cfg(feature = "feature_x")]
            {
                *shared += 1;
                //hprintln!("UART0: X = {}", shared).unwrap();
            }
        }
    };
//}
