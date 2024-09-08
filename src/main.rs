#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ekos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ekos::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use x86_64::registers::control::Cr3;
    let version = "0.5.0";
    println!("ekos {}", version);

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    ekos::hlt_loop();

    /* loop {
        use ekos::print;
        print!("-");
    } */
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ekos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}