use std::os::raw::{c_char, c_int};
use std::ffi::CStr;

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct SwapContext;

static CONTEXT: Lazy<Mutex<Option<SwapContext>>> = Lazy::new(|| Mutex::new(None));

#[no_mangle]
pub extern "C" fn mw_atomic_swap_init() -> c_int {
    let mut ctx = CONTEXT.lock().unwrap();
    *ctx = Some(SwapContext);
    0
}

#[no_mangle]
pub extern "C" fn mw_atomic_swap_offer(slate: *const c_char) -> c_int {
    if slate.is_null() {
        return -1;
    }
    let _ = unsafe { CStr::from_ptr(slate) };
    0
}

#[no_mangle]
pub extern "C" fn mw_atomic_swap_accept(slate: *const c_char) -> c_int {
    if slate.is_null() {
        return -1;
    }
    let _ = unsafe { CStr::from_ptr(slate) };
    0
}

#[no_mangle]
pub extern "C" fn mw_atomic_swap_finalize() -> c_int {
    let mut ctx = CONTEXT.lock().unwrap();
    *ctx = None;
    0
}
