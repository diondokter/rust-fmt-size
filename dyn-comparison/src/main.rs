#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Free unused ram
const SCRATCH_SPACE_START: u32 = 0x20001000;

#[cortex_m_rt::entry]
fn main() -> ! {
    #[allow(unused_mut)]
    let mut string = heapless::String::<64>::new();

    if cfg!(feature = "raw-str") {
        write_generic_str(core::hint::black_box(&mut string), "Test");
    }
    if cfg!(feature = "dyn-str") {
        write_dyn_str(core::hint::black_box(&mut string), "Test");
    }
    if cfg!(feature = "raw-u32") {
        write_generic_u32(core::hint::black_box(&mut string), core::hint::black_box(123456));
    }
    if cfg!(feature = "dyn-u32") {
        write_dyn_u32(core::hint::black_box(&mut string), core::hint::black_box(123456));
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

fn write_generic_str(target: &mut impl core::fmt::Write, value: &'static str) {
    target.write_str(value).unwrap();
}

fn write_dyn_str(target: &mut dyn core::fmt::Write, value: &'static str) {
    target.write_str(value).unwrap();
}

fn write_generic_u32(target: &mut impl core::fmt::Write, mut value: u32) {
    // Implementation uses more flash than ufmt, but doesn't use an intermediary buffer

    if value == 0 {
        target.write_char('0').unwrap();
        return;
    }

    let mut digits = value.ilog10() + 1;

    while digits > 0 {
        let upper_value_digit = value / 10u32.pow(digits - 1);
        target.write_char(char::from_digit(upper_value_digit, 10).unwrap()).unwrap();
        value -= upper_value_digit * 10u32.pow(digits - 1);
        digits -= 1;
    }
}

fn write_dyn_u32(target: &mut dyn core::fmt::Write, mut value: u32) {
    if value == 0 {
        target.write_char('0').unwrap();
        return;
    }

    let mut digits = value.ilog10() + 1;

    while digits > 0 {
        let upper_value_digit = value / 10u32.pow(digits - 1);
        target.write_char(char::from_digit(upper_value_digit, 10).unwrap()).unwrap();
        value -= upper_value_digit * 10u32.pow(digits - 1);
        digits -= 1;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
