#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Free unused ram
const SCRATCH_SPACE_START: u32 = 0x20001000;

#[cortex_m_rt::entry]
fn main() -> ! {
    #[allow(unused_mut)]
    let mut string = heapless::String::<64>::new();

    let value = unsafe { ((SCRATCH_SPACE_START + 0x800) as *const i32).read_volatile() };

    if cfg!(feature = "raw") {
        string.push_str("Test").unwrap();
    }
    if cfg!(feature = "fmt-no-args") {
        use core::fmt::Write;
        write!(string, "Test").unwrap();
    }
    if cfg!(feature = "fmt-u32") {
        use core::fmt::Write;
        write!(string, "{}", value as u32).unwrap();
    }
    if cfg!(feature = "fmt-i32") {
        use core::fmt::Write;
        write!(string, "{}", value as i32).unwrap();
    }
    if cfg!(feature = "fmt-f32") {
        use core::fmt::Write;
        write!(string, "{}", value as f32).unwrap();
    }
    if cfg!(feature = "ufmt-no-args") {
        use ufmt::uwrite;
        uwrite!(string, "Test").unwrap();
    }
    if cfg!(feature = "ufmt-u32") {
        use ufmt::uwrite;
        uwrite!(string, "{}", value as u32).unwrap();
    }
    if cfg!(feature = "ufmt-i32") {
        use ufmt::uwrite;
        uwrite!(string, "{}", value as i32).unwrap();
    }
    if cfg!(feature = "ufmt-f32") {
        use ufmt::uwrite;
        uwrite!(string, "{}", ufmt_float::uFmt_f32::Four(value as f32)).unwrap();
    }

    // Make sure the string is not optimized away
    unsafe {
        let scratch_ptr = SCRATCH_SPACE_START as *mut heapless::String<64>;
        scratch_ptr.write_volatile(string);
    }

    loop {
        cortex_m::asm::bkpt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
