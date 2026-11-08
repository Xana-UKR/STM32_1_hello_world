# NUCLEO-F439ZI Rust learning guide

Follow this guide to repeat the board setup and hands-on Rust exercises from the beginning. It records tested steps and PowerShell commands with short explanations. Firmware source belongs in the project files rather than this guide. The project journal holds the working observations; this guide will be reviewed and polished when the project is complete.

**Status:** Toolchain setup and STM32 debug communication were checked. A copy of the board's main flash was saved and checked for size and matching startup bytes. Firmware creation, flashing, serial output, and source debugging remain to be done. See [PROJECT_JOURNAL.md](PROJECT_JOURNAL.md) for progress and [PROJECT_SPEC.md](PROJECT_SPEC.md) for completion criteria.

## What you need

- A NUCLEO-F439ZI and a Windows 11 computer.
- A USB **data** cable with Micro-B at the board end and a connector suitable for the computer. A charge-only cable will not work.
- Internet access for tools and Rust packages. Driver installation may require administrator rights.

The STM32 microcontroller runs the firmware. The onboard ST-LINK connects the computer to it for programming and debugging. One USB cable also supplies power and exposes a virtual serial port; no extra probe, power supply, or jumper wires are needed.

## 1. Install the ST-LINK driver

With the board disconnected, download **STSW-LINK009** from [ST's driver page](https://www.st.com/en/development-tools/stsw-link009.html). Extract the package, run its driver installer, and follow the Windows prompts.

## 2. Check and connect the board

Keep USB disconnected and place the board on a dry, nonconductive surface while checking the jumpers. These settings were checked by the owner for the USB-powered setup; see [ST manual UM1974](https://www.st.com/resource/en/user_manual/dm00244518-stm32-nucleo144-boardsstmicroelectronics.pdf).

| Jumper | Setting | What to look for |
| --- | --- | --- |
| JP1 | OFF | No cap joining its two pins. |
| JP3 | U5V | A cap joining pins 3 and 4, selecting ST-LINK USB power. |
| JP5 | ON | A cap joining its two pins. |
| CN4 | Both ON | Two caps, each joining its pair of pins, connecting ST-LINK to the STM32. |

A jumper cap electrically joins the pins it covers. If the configuration is unclear, consult the manual before changing it.

Connect the cable to **CN1**, the ST-LINK Micro-B connector, and the computer. CN13 is the separate user USB connector and is not used here.

Open **Device Manager → Ports (COM & LPT)**. The project computer showed `STMicroelectronics STLink Virtual COM Port (COM6)`. Recheck the number after reconnecting; it can change. To identify the port, watch which entry disappears and reappears when unplugging and reconnecting the board. A listed port confirms the interface is detected; firmware output still needs testing.

## 3. Install Rust and the STM32 target

Install Rust through **rustup** from the [Rust installation page](https://www.rust-lang.org/tools/install), using the default installation. Follow any prompts for Microsoft C++ build tools. Reopen PowerShell, then check the tools:

```powershell
rustc --version
cargo --version
rustup --version
```

Add the compilation target for the STM32's Cortex-M4 processor with floating-point support:

```powershell
rustup target add thumbv7em-none-eabihf
rustup target list --installed
```

The list should include `thumbv7em-none-eabihf`. This target builds firmware for the STM32 rather than a Windows executable.

## 4. Install probe-rs

`probe-rs` communicates with ST-LINK to program and debug the STM32. The project computer used this command from the [probe-rs installation guide](https://probe.rs/docs/getting-started/installation/):

```powershell
irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
```

This downloads and immediately runs a PowerShell script. The `latest` URL can install a newer release than the version recorded below.

Reopen PowerShell if needed, then check the version and connected probes:

```powershell
probe-rs --version
probe-rs list
```

The project computer reported **STLink V2-1** with the board connected. The next step checks communication with the STM32 itself.

## 5. Check the STM32 debug connection

With the board connected, find the supported chip name and read one 32-bit value from the processor identification register:

```powershell
probe-rs chip list | Select-String "STM32F439ZI"
probe-rs read --chip STM32F439ZI b32 0xE000ED00 1
```

The first command listed `STM32F439ZI`; the second returned:

```text
e000ed00: 410fc241
```

This confirmed debug communication without flashing firmware. Command behavior can differ between probe-rs versions.

## Observed environment

Earlier setup was reported on **Windows 11 version 25H2**, with these versions:

| Tool | Version |
| --- | --- |
| `rustc` | 1.99.1 |
| `cargo` | 1.99.1 |
| `rustup` | 1.29.2 |
| `probe-rs` | 0.32.0 |

The instructions have not yet been tested on a second computer. An editor, serial terminal, debugger integration, and firmware dependencies remain to be selected and documented.

## 6. Copy the current main flash before programming

On the project board, a read of the first two 32-bit words at `0x08000000` returned `20030000 080030d9`. These are consistent with an initial stack pointer and a reset handler, so an application appears to be present. This does not identify its purpose or value.

The owner ran this command in the project directory to save the STM32F439ZI's 2 MiB main flash as a raw binary file:

```powershell
probe-rs read --chip STM32F439ZI --format binary --output .\stm32f439zi-before-first-flash-2026-11-08.bin b8 0x08000000 2097152
```

The command returned to the PowerShell prompt without console output. The resulting PC file had exactly **2,097,152 bytes**. Its first eight bytes, `00 00 03 20 D9 30 00 08`, matched the earlier board read in little-endian byte order. `Get-FileHash` reported SHA-256 `D1935C2544CB369F50FB06B92BF0029DCC0E9CE9442652A66E4890F79E9D28C2`. Keep this file safe before flashing. The copy covers main flash, not ordinary RAM or other special memory, and restoring it has not been tested. The read did not erase or program the board.

## Troubleshooting

- **No power:** Check CN1, the cable, the computer's USB port, and the jumper settings above.
- **Missing COM port or a driver warning:** Check the ST-LINK driver installation and reconnect the board.
- **No probes in `probe-rs list`:** Check power, the data cable, and the driver. This occurred when the project board was disconnected.
- **Probe detected but memory read fails:** Check both CN4 caps and the chip name in `probe-rs chip list`. Review the error before changing hardware.
- **`probe-rs info` cannot identify the chip:** With version 0.32.0, automatic identification failed and `info --chip` reported that it ignored the chip option. The explicit-chip `read` command above succeeded.

## Firmware instructions pending

Build, flash, serial, and debugger instructions will be added after those stages are tested. **Flashing replaces existing target firmware.** A precautionary main-flash copy has been saved, but restoring it has not been tested; see the journal.
