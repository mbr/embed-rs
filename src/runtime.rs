// provisional runtime
// apparently, even with panic="abort" you need to have a panic_fmt
// implementation
use core::fmt;
#[lang = "panic_fmt"]
#[unwind]
extern "C" fn panic_impl(_: fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

// apparently, we need this even though we use panic="abort"?
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

// end runtime
