EK-TM4C1294XL-DEMO
==================

This repository contains a sample firmware written in Rust that runs on
the [TI EK-TM4C1294XL][evb] evaluation board.

[evb]: http://www.ti.com/tool/ek-tm4c1294xl

Steam cannon start
------------------

If you aren't familiar with embedded development, skip to the next section.

If you have nightmares about JTAG and then you wake up and you still have
a nightmare about JTAG because there's a JTAG adapter omniously staring
at you with its malicious little red eye and you have nowhere to run and
just want the pain to end, here's the sequence of incantations for going
from a clean slate system to flashing, debugging and semihosting:

1. Install `arm-none-eabi-ld` and `arm-none-eabi-gdb`.
2. Install [openocd][].
3. Install [rustup][].
4. Install the embedded Rust toolchain:
    ```sh
    rustup default nightly
    rustup component add rust-src
    cargo install -f xargo
    ```
5. Allow unprivileged access to the evaluation board. On Linux, install the provided
   udev rules file, and re-plug the device afterwards:
   ```sh
   sudo cp support/99-stellaris.rules /etc/udev/rules.d
   ```
6. Start the openocd server:
    ```sh
    openocd -f support/openocd.cfg
    ```
7. Build the firmware:
    ```sh
    xargo build --release
    ```
8. Load the firmware:
    ```sh
    arm-none-eabi-gdb -x support/load.gdb \
      target/thumbv7em-none-eabihf/release/ek-tm4c1294xl-demo
    ```

At this point, the target is stopped immediately after reset. Let it continue,
and observe the `Hello, world!` printed by openocd. You're done! Now, repeat
steps 7 and 8 until you're happy with the rest of the owl.

[openocd]: http://openocd.org/
[rustup]: https://rustup.rs/

Quick start
-----------

If you want a less terse explanation, you're in luck, because @japaric
wrote an [excellent introduction][quickstart]! The code in this repository
only differs in that it's adapted to the EK-TM4C1294XL board, and a few
insignificant details.

[quickstart]: http://blog.japaric.io/quickstart/

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
