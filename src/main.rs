#![no_std]
#![no_main]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let version = "v0.2";
    println!("ekos {}", version);

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}