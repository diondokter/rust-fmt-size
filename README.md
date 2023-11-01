# Rust fmt size

Let's compare different binary sizes with regards to fmt code.
This is made specifically for embedded use.

The code generation has been set up to use the least amount of flash.
- Compile with opt-level 'z'
- Panic abort (and build the core lib with panic abort)
- LTO is true (fat)
- codegen units is 1
- incremental is false

To run it yourself, call `cargo +nightly -Zscript run.rs`.
It will update the [results page](./results.md) by compiling the projects with different combinations of feature flags.

There are two embedded projects:
- [fmt-comparison](./fmt-comparison) - Compare `core::fmt` with the much smaller `ufmt`
- [dyn-comparison](./dyn-comparison) - Compare `&mut dyn Write` to `&mut impl Write`

More written about it here: https://jamesmunns.com/blog/fmt-unreasonably-expensive/

Since James' blog post from 2019, there's now the [`defmt` crate](https://defmt.ferrous-systems.com/) that's quite prolific in the embedded space right now.

## TOC

- [Rust fmt size](#rust-fmt-size)
  - [TOC](#toc)
  - [Analyzing u32 formatting](#analyzing-u32-formatting)
    - [u32 by ufmt](#u32-by-ufmt)
    - [u32 by fmt](#u32-by-fmt)
  - [The effects of `dyn Write`](#the-effects-of-dyn-write)
  - [Conclusion](#conclusion)


## Analyzing u32 formatting

Let's analyze how fmt and ufmt differ. They use similar algorithms.
But the difference is that the fmt code is way more optimized for speed and sacrifices a lot of code size for it by
including a lookup table and specializations for converting multiple digits at once.

The results can be seen on the [results page](./results.md#fmt-comparison).

I've removed the existing comments about the unsafe code. We can assume everything is safe.
I've also changed the order of some of the functions and macros for clarity.

### u32 by ufmt

<details>

<summary>Rust ufmt code</summary>

```rust
// Display refers to debug, so this is also the display implementation
impl uDebug for u32 {
    #[cfg(not(target_pointer_width = "16"))] // Optimization. This basically asks, is usize >= u32?
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>  // Note, not the same formatter as in core
    where
        W: uWrite + ?Sized,
    {
        // Create a buffer of bytes that can contain the largest text value a u32 can represent.
        // Note that even though we're gonna cast to a potentially 64-bit usize, the buffer is specialized for 32-bit.
        let mut buf = [MaybeUninit::uninit(); 10];

        // Write to the formatter
        // Optimization, call the usize impl so every `uxx` <= usize shares the same impl
        f.write_str(usize(*self as usize, &mut buf))
    }

    #[cfg(target_pointer_width = "16")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        // Create a buffer of bytes that can contain the largest text value a u32 can represent
        let mut buf = [MaybeUninit::<u8>::uninit(); 10];

        // Write to the formatter
        // No optimization here since u32 > usize
        let s = uxx!(*self, buf);
        f.write_str(s)
    }
}

// Optimization function so all uxx <= usize can use the same implementation.
fn usize(n: usize, buf: &mut [MaybeUninit<u8>]) -> &str {
    uxx!(n, buf)
}

macro_rules! uxx {
    ($n:expr, $buf:expr) => {{
        // Get the pointer and length to the buffer. The maybeuninit is cast away
        let ptr = $buf.as_mut_ptr().cast::<u8>();
        let len = $buf.len();
        // We need to mutate the number
        let mut n = $n;
        // We're gonna write the smallest number one by one so the buffer needs to be accessed from the back
        let mut i = len - 1;
        loop {
            // Ascii trick where we add the lowest digit to the 0 character
            // The value is written to the buffer
            // Integer modulo is quite expensive performance-wise
            unsafe { ptr.add(i).write((n % 10) as u8 + b'0') }
            // Lowest number has been dealt with,
            // so we can divide by 10 so we can get the next lowest base 10 number next time.
            // Integer division is quite expensive performance-wise though
            n /= 10;

            // Are we done?
            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        // Return the string
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr.add(i), len - i)) }
    }};
}
```

</details>

### u32 by fmt

<details>

<summary>Rust fmt code</summary>

```rust
// Display is implemented through macros and there's some optimization here with the selection of the bits.
// This is the one that's used for ARM embedded. Everything 32-bit and smaller is formatted as 32-bit.
// 64 and 128 bit have their own implementation. (Though 128 is somewhere else)
#[cfg(not(any(target_pointer_width = "64", target_arch = "wasm32")))]
mod imp {
    use super::*;
    // First the types that display is implemented for.
    // Then 'as' with the type that they are converted to.
    // After that 'via' the function to do the conversion.
    // The created fmt function is 'named' the last argument.
    impl_Display!(i8, u8, i16, u16, i32, u32, isize, usize as u32 via to_u32 named fmt_u32);
    impl_Display!(i64, u64 as u64 via to_u64 named fmt_u64);
}

// 2 digit decimal look up table
static DEC_DIGITS_LUT: &[u8; 200] = b"0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

macro_rules! impl_Display {
    ($($t:ident),* as $u:ident via $conv_fn:ident named $name:ident) => {
        // Implement the display trait.
        // Since this macro is used for both signed and unsigned integers, it needs to deal with negative numbers
        $(#[stable(feature = "rust1", since = "1.0.0")]
        impl fmt::Display for $t {
            #[allow(unused_comparisons)]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // We need to keep track of negative numbers because we might need to add a minus in the front
                let is_nonnegative = *self >= 0;
                // Get the number as the converted unsigned integer
                let n = if is_nonnegative {
                    self.$conv_fn()
                } else {
                    // convert the negative num to positive by summing 1 to it's 2 complement
                    (!self.$conv_fn()).wrapping_add(1)
                };
                // Do the formatting
                $name(n, is_nonnegative, f)
            }
        })*

        fn $name(mut n: $u, is_nonnegative: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // 2^128 is about 3*10^38, so 39 gives an extra byte of space
            // That's a lot of space we're not using
            let mut buf = [MaybeUninit::<u8>::uninit(); 39];
            let mut curr = buf.len();
            let buf_ptr = MaybeUninit::slice_as_mut_ptr(&mut buf);
            let lut_ptr = DEC_DIGITS_LUT.as_ptr();

            unsafe {
                // need at least 16 bits for the 4-characters-at-a-time to work.
                assert!(crate::mem::size_of::<$u>() >= 2);

                // eagerly decode 4 characters at a time
                while n >= 10000 {
                    let rem = (n % 10000) as usize;
                    n /= 10000;

                    let d1 = (rem / 100) << 1;
                    let d2 = (rem % 100) << 1;
                    curr -= 4;

                    ptr::copy_nonoverlapping(lut_ptr.add(d1), buf_ptr.add(curr), 2);
                    ptr::copy_nonoverlapping(lut_ptr.add(d2), buf_ptr.add(curr + 2), 2);
                }

                // if we reach here numbers are <= 9999, so at most 4 chars long
                let mut n = n as usize; // Optimization: possibly reduce 64bit math

                // decode 2 more chars, if > 2 chars
                if n >= 100 {
                    let d1 = (n % 100) << 1;
                    n /= 100;
                    curr -= 2;
                    ptr::copy_nonoverlapping(lut_ptr.add(d1), buf_ptr.add(curr), 2);
                }

                // decode last 1 or 2 chars
                if n < 10 {
                    curr -= 1;
                    *buf_ptr.add(curr) = (n as u8) + b'0';
                } else {
                    let d1 = n << 1;
                    curr -= 2;
                    ptr::copy_nonoverlapping(lut_ptr.add(d1), buf_ptr.add(curr), 2);
                }
            }

            let buf_slice = unsafe {
                str::from_utf8_unchecked(
                    slice::from_raw_parts(buf_ptr.add(curr), buf.len() - curr))
            };
            f.pad_integral(is_nonnegative, "", buf_slice)
        }
    };
}
```

</details>

## The effects of `dyn Write`

The rust formatting code uses a lot of `dyn Write`. This is probably to make the code not generic over the underlying
output buffer types. If this weren't the case, both the `Debug` and `Display` traits would need to be generic over a `T: Write`.

For example, this is also the case in the formatter.

```rust
pub struct Formatter<'a> {
    flags: u32,
    fill: char,
    align: rt::Alignment,
    width: Option<usize>,
    precision: Option<usize>,

    buf: &'a mut (dyn Write + 'a), // <== Here
}
```

This likely has some implications though. Because the code needs to go through this interface using essentially function pointers,
it's opaque to the optimizer which makes us miss a lot of optimizations. And also the full `Write` trait implementation needs to be present.

Let's see how much that matters.

*Note: The compiler is pretty good at devirtualization in simple situations, but it can't do it in real fmt code.
So we need to use the black box so that the compiler can't optimize in the simple situation we have in the code.*

The results can be seen on the [results page](./results.md#dyn-comparison).

## Conclusion

We've looked at the two main things that are costing flash memory space with regard to formatting.

1. Using `dyn Write` as the interface to write to the buffers to.  
   This causes us to miss optimizations (due to missing inlining) and requires the compiler to generate the full `Write` implementation for types even when parts of it aren't used.

   `uFmt` solves this by creating its own `Formatter` that is generic over buffer `W: Write`.
2. Formatting code created for speed, not for binary size.

We cannot change 1 because just like `uFmt` we'd have to change the public `Formatter` signature which would be a pretty big breaking change.

We can change 2, but lots of people will not be happy with slower formatting.
That leaves us with 3 options:

1. Have a pluggable formatting solution, similar to global alloc.  
   People can then develop their own implementation.
2. Have both a slow and a fast mode which you can choose to compile in.  
   On embedded, we could then select the slow formatting code to save a bit on binary size.
3. Go through the existing code and see what can be improved without making it slower.  
   This means benchmarking alternative implementations, removing panic paths, more code reuse, etc.

*NOTE: I mention fast and slow formatting code, but `uFmt` [claims to be faster](https://docs.rs/ufmt/latest/ufmt/#benchmarks) on the tested embedded target.*
