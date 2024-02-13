#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod system;
mod panic;
mod config;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    system::stdlib::cpu::init();
    system::init();
}