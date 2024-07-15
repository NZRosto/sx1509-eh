An [`embedded-hal`](https://docs.rs/embedded-hal/latest/embedded_hal/) focused driver for the SX1509 I/O expander.

This crate allows you to use the expansion pins on the SX1509 as if they were simply GPIO pins on your microcontroller.

## Portable Atomic
This crate uses [`portable-atomic`](https://docs.rs/portable-atomic/latest/portable_atomic/) to provide platform-agnostic atomic operations. This is necessary to implement the internal shared i2c bus. You may need to enable certain features of `portable-atomic` to get this crate to compile on platforms that don't natively support atomic operations.

## Usage

```rust
// 0x3F is used here, the actual address will vary
// depending on the configuration of the ADDR
// pins on the chip.
let mut expander = Sx1509::new(i2c, 0x3F).unwrap();

// This borrows the expander, so the expander
// cannot be dropped before it's pins.
let sx1509::Pins {
    a0,
    a1,
    ..
} = expander.split();

let pin_0 = a0.into_output().unwrap();
let pin_1 = a1.into_input().unwrap();

pin_0.set_high().unwrap();
assert_eq!(pin_1.is_high().unwrap(), true);
```
