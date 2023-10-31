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
        write_generic(core::hint::black_box(&mut string), "Test");
    }
    if cfg!(feature = "dyn") {
        write_dyn(core::hint::black_box(&mut string), "Test");
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

#[inline(never)]
fn write_generic(target: &mut impl core::fmt::Write, value: &'static str) {
    target.write_str(value).unwrap();
}

#[inline(never)]
fn write_dyn(target: &mut dyn core::fmt::Write, value: &'static str) {
    target.write_str(value).unwrap();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
