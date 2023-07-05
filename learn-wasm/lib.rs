#![no_std]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

extern crate alloc;
use alloc::vec::Vec;

// #[no_mangle]
// pub extern "C" fn add(left: usize, right: usize) -> usize {
//     left + right
// }

static PRIMES: &[i32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];

#[no_mangle]
pub extern "C" fn nth_prime(n: usize) -> i32 {
    PRIMES[n]
    // PRIMES.get(n).copied().unwrap_or(0)
}
