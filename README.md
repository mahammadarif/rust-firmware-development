# Blink LED Project with rust
This project demonstrates a simple LED blinking application using Morse code on an STM32F401 microcontroller, written in Rust. The LED on pin PC13 blinks the Morse code for "HELLO" repeatedly.
Project Structure

---

- Cargo.toml: Configuration file for the Rust project, specifying dependencies and build settings.
- memory.x: Linker script defining memory layout for the STM32F401 (256KB Flash, 64KB RAM).
- src/main.rs: Main application code implementing the Morse code blinking logic.
- .cargo/config.toml: Build configuration specifying the target architecture and linker script.

--- 

## Prerequisites

- **Rust toolchain with rustup installed.
- **arm-none-eabi toolchain for generating the binary.
- **An STM32F401-based development board (e.g., Black Pill).
- **A programmer (e.g., ST-Link) to flash the binary to the microcontroller.

---

## Setup Instructions

---

Install Rust and Required Components:

```bash
Install Rust using rustup:curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Install the ARM target:

```bash
rustup target add thumbv7em-none-eabihf
```
---

## Install ARM Toolchain:

Install the arm-none-eabi toolchain for compiling and converting the binary:

```bash
sudo apt-get install gcc-arm-none-eabi
```

## Project Files

- **Cargo.toml
This file defines the project metadata and dependencies:

---

Specifies the thumbv7em-none-eabihf target for STM32F401.
Includes dependencies like embedded-hal, stm32f4xx-hal, cortex-m, and others for embedded development.

- ** memory.x
The linker script defines the memory layout:

---
- **Flash memory: 256KB starting at 0x08000000.
RAM: 64KB starting at 0x20000000.
Sets the stack start at the end of RAM.

- **src/main.rs
## The main application:

Uses the STM32F401 HAL to configure the system clock (84 MHz using a 25 MHz HSE).
Sets up GPIOC pin PC13 as an output for the LED (active low).
Implements a Morse code transmitter for the message "HELLO".
Uses a timer (TIM5) for precise delays.

---

- **.cargo/config.toml
The .cargo/config.toml file is a Rust configuration file that customizes the build process for the project. It is located in the .cargo directory at the root of the project. Below is its content:

---

```bash
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

## Explanation:

[build] Section:
target = "thumbv7em-none-eabihf": Specifies the target architecture for the build, which is the ARM Cortex-M4F with no operating system (bare-metal) and hard-float support, suitable for the STM32F401 microcontroller.


[target.thumbv7em-none-eabihf] Section:
rustflags: Passes additional flags to the Rust compiler.
"-C", "link-arg=-Tlink.x": Instructs the linker to use the link.x script provided by the cortex-m-rt crate, which works in conjunction with the memory.x file to define the memory layout and linking process.



Note: The .cargo/config.toml file is not checked into the repository by default, as it is specific to the build configuration. You need to create it manually in the .cargo directory with the above content to ensure the project builds correctly for the STM32F401 target.
Build Instructions

Create .cargo/config.toml:

Create a .cargo directory in the project root:mkdir .cargo


Create and edit the config.toml file:nano .cargo/config.toml


Paste the content shown above and save.


## Build the Project:

Compile the project in release mode for the ARM target:cargo build --release --target thumbv7em-none-eabihf


## Generate the Binary:

Convert the compiled output to a .bin file for flashing:arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/blink_led blink_led.bin


## Check Binary Size:

Verify the size of the compiled binary:cargo size --release --target thumbv7em-none-eabihf

Example output:   text    data     bss     dec     hex filename
  10224       0       4   10228    27f4 blink_led


## Clean the Project:

Remove build artifacts:cargo clean


## List Available Targets:

View supported Rust targets:rustup target list



## Flashing the Binary
Use a programmer (e.g., ST-Link) to flash blink_led.bin to the STM32F401 board. For example, with openocd:
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c "program blink_led.bin 0x08000000 verify reset exit"

## Running the Project
Once flashed, the LED on pin PC13 will blink the Morse code for "HELLO" (.... . .-.. .-.. ---) in an infinite loop. The timing is based on a 20 WPM Morse code speed:

Dot: 200ms on
Dash: 600ms on
Inter-element pause: 200ms
Letter space: 500ms
Word space: 1000ms

## Notes

The LED is active low (set low to turn on, high to turn off).
The project uses panic-halt to stop execution on errors, ensuring no undefined behavior.
Ensure the HSE clock source is a 25 MHz crystal for correct timing.
The .cargo/config.toml file must be present for the build to link correctly with the memory.x script.

## Troubleshooting

Build Errors: Ensure the arm-none-eabi toolchain, Rust target, and .cargo/config.toml file are correctly set up.
Flashing Issues: Verify the programmer connection and configuration.
No Blinking: Check the LED pin (PC13) and ensure the board is powered correctly.

