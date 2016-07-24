//! STM32-Discovery boards minimal runtime.
//!
//! See the `README.md` for a detailed introduction.

#![feature(asm)]
#![feature(lang_items)]
#![feature(unwind_attributes)]
#![no_std]

pub mod irq;
pub mod runtime;
pub mod util;

// board-specific stuff
pub mod stm32f429i;

pub type InterruptHandler = extern "C" fn() -> ();

#[allow(improper_ctypes)]
extern {
    static _STACK_TOP: ();
}

#[macro_export]
macro_rules! board {
    ($board:ident,
     {
         $( $fname:ident : $fval:expr),*
     }
    )  =>
    (
        #[allow(improper_ctypes)]
        extern {
            static _STACK_TOP: ();
        }

        extern "C" fn _rust_start() {
            ::main()
        }

        #[link_section="vectors"]
        #[no_mangle]
        pub static VECTORS: $crate::$board::VectorTable =
        $crate::$board::VectorTable {
            msp: &_STACK_TOP,
            reset: Some(_rust_start),
            $( $fname: $fval, )*
            ..$crate::$board::VECTOR_TABLE
        };

        use $crate::$board::INITIAL_CPU_FREQ;
    )
}
