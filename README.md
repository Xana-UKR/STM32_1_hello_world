# Rust on NUCLEO-F439ZI: build, flash, serial, and debugging

Use the supplied project files to run a small Rust application on an ST NUCLEO-F439ZI from Windows. It drives blue LED LD2 on for 5 seconds and off for 5 seconds, and transmits a message with a cycle counter approximately every 10 seconds. No external wiring is needed.

The owner demonstrated building, LED control, power-cycle startup of the earlier LED firmware, recurring serial output, and a Rust source breakpoint with stepping and variable inspection. See [tested evidence and limitations](#tested-evidence-and-limitations), the [project journal](PROJECT_JOURNAL.md), and the [requirements](PROJECT_SPEC.md).

## 1. Prepare the computer and project

You need Windows, the NUCLEO-F439ZI, one USB data cable with Micro-B at the board end, and internet access for installation and Rust dependencies. Use Visual Studio Code for the debugging exercise.

Open this project **folder** in VS Code, including its hidden `.cargo` and `.vscode` folders and `Cargo.lock`. Run the PowerShell commands below from the folder containing `Cargo.toml`. The supplied project is already initialized; do not run `cargo init` again. Earlier project-creation exercises are recorded in the journal.

Install Rust through rustup from the [official Rust installation page](https://www.rust-lang.org/tools/install), using the default installation and following any Microsoft C++ build-tool prompts. Reopen PowerShell, then check:

```powershell
rustc --version
cargo --version
rustup --version
```

`rustc` compiles Rust; Cargo manages dependencies and builds; rustup installs compiler toolchains and compilation targets. Add the target for the STM32's Cortex-M4 processor with hardware floating-point support:

```powershell
rustup target add thumbv7em-none-eabihf
rustup target list --installed
```

The list must include `thumbv7em-none-eabihf`. This selects embedded machine code rather than a Windows executable.

Install probe-rs using the PowerShell command used on the project computer, from the [probe-rs installation guide](https://probe.rs/docs/getting-started/installation/):

```powershell
irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
```

This downloads and immediately executes an installer script. The `latest` URL can install a newer version than the tested one. Reopen PowerShell and check:

```powershell
probe-rs --version
```

Before connecting the board, install **STSW-LINK009** from [ST's ST-LINK driver page](https://www.st.com/en/development-tools/stsw-link009.html). Extract the package and run its Windows driver installer; administrator rights may be needed.

## 2. Connect and check the board

With USB disconnected, place the board on a nonconductive surface and check the documented USB-powered configuration. These settings were checked by the owner; consult [ST manual UM1974](https://www.st.com/resource/en/user_manual/dm00244518-stm32-nucleo144-boardsstmicroelectronics.pdf) if your board differs.

| Jumper | Setting | Meaning |
| --- | --- | --- |
| JP1 | OFF | No cap joining its pins. |
| JP3 | U5V, pins 3–4 | Select ST-LINK USB power. |
| JP5 | ON | Cap joining its pins. |
| CN4 | Both ON | Two caps connect ST-LINK to the target STM32. |

Connect the USB data cable to **CN1**, the ST-LINK connector. CN13 is the separate user USB connector and is not used here. ST-LINK supplies programming/debugging access and a virtual serial port through the same cable. An Explorer disk window may also appear; no copying to that virtual disk is needed.

In **Device Manager → Ports (COM & LPT)**, find **STMicroelectronics STLink Virtual COM Port**. It was **COM6** on the project computer. Recheck after reconnection; the number can change. If uncertain, identify which entry disappears and returns when unplugging and reconnecting the board.

Check the probe and target connection:

```powershell
probe-rs list
probe-rs chip list | Select-String "STM32F439ZI"
probe-rs read --chip STM32F439ZI b32 0xE000ED00 1
```

The owner observed STLink V2-1, supported chip name STM32F439ZI, and processor identification value `e000ed00: 410fc241`. This establishes debug communication without programming flash.

## 3. Preserve existing firmware before the first flash

**Flashing replaces the board's existing firmware.** Before programming a previously used board, save its main flash if its contents may matter. Choose an unused output filename and keep the copy safe; never overwrite the original backup with a read taken after flashing.

```powershell
probe-rs read --chip STM32F439ZI --format binary --output .\before-first-flash.bin b8 0x08000000 2097152
```

This reads the 2 MiB main flash into a PC file without erasing or programming the board. The command can finish without console output. Check the file size and record its hash:

```powershell
(Get-Item .\before-first-flash.bin).Length
Get-FileHash .\before-first-flash.bin -Algorithm SHA256
```

Expected size is **2097152 bytes**. The original project's copy has a different, dated filename; its hash and matching startup-byte checks are recorded in the journal. This is a main-flash copy, not a copy of RAM or every special memory region. Restoration has not been tested.

## 4. Understand and build the supplied firmware

The build path is **Rust source → compiler and linker → ELF firmware → probe-rs → STM32**. The program runs without an operating system. `cortex-m-rt` supplies startup; the STM32F4 HAL (Hardware Abstraction Layer) provides hardware-control methods.

| Project file | Role |
| --- | --- |
| [Cargo.toml](Cargo.toml) | Package metadata, Rust edition 2024, exact direct dependencies, and the stm32f439 feature. |
| [Cargo.lock](Cargo.lock) | Exact resolved dependency versions; keep it with the application. |
| [src/main.rs](src/main.rs) | Bare-metal entry point, LED, timer, serial transmitter, counter, and panic handler. |
| [.cargo/config.toml](.cargo/config.toml) | Default embedded target and the `-Tlink.x` linker option. |
| [memory.x](memory.x) | 2048 KiB flash at 0x08000000 and 192 KiB RAM at 0x20000000. Separate 64 KiB CCM RAM is unused. |
| [build.rs](build.rs) | Runs on Windows, copies the memory description into build output, and supplies a linker search path. |
| [.vscode/launch.json](.vscode/launch.json) | VS Code debugger settings and the built ELF path. |
| [Read-Serial.ps1](Read-Serial.ps1) | Windows serial reader with configurable port, baud rate, and duration. |

`link.x` is supplied by the runtime and includes `memory.x`; it arranges the vector table, code, and data. The memory regions match [ST's STM32F439ZI datasheet](https://www.st.com/resource/en/datasheet/stm32f439zi.pdf), memory map on page 86.

Check the source without linking, then build the complete firmware:

```powershell
cargo check --target thumbv7em-none-eabihf
cargo build
```

Both commands succeeded during the guided exercises. The build uses the default dev profile, unoptimized with debug information. Output is `target\thumbv7em-none-eabihf\debug\stm32_1_hello_world`: an **ELF** containing machine code, addresses, and debug information, with no `.exe` extension. Building only changes PC files. Rebuild after every source change; the debug configuration does not do this automatically.

## 5. Flash and observe the LED

With the board connected through CN1 and any active debug session ended, run:

```powershell
probe-rs download --chip STM32F439ZI --verify --restore-unwritten --reset .\target\thumbv7em-none-eabihf\debug\stm32_1_hello_world
```

`download` programs the ELF. `--verify` requests readback verification; `--restore-unwritten` preserves bytes outside the image within sectors erased during programming; `--reset` starts the firmware afterward. Options were checked against installed probe-rs help. Review the output for errors before proceeding.

Expected behavior is blue **LD2 on PB7** on for 5 seconds and off for 5 seconds. GPIO means General-Purpose Input/Output; PB7 is port B, pin 7, not a connector pin number. UM1974 documents this LED as active HIGH. TIM5 provides the blocking delays: the processor waits before continuing the loop.

To check independent startup, unplug USB and reconnect through CN1 without another programming command. The owner confirmed power-cycle startup with earlier steady-LED and blinking firmware. The latest counter version is expected to behave the same, but has not been separately power-cycle checked in chat.

## 6. Read serial messages in PowerShell

The route is **firmware → USART3 TX on PD8 → ST-LINK → USB → Windows COM port**. USART3 is used in asynchronous serial mode; PD9 RX is unused. UM1974 section 6.9 documents the default ST-LINK routing. Serial communication worked without wiring or solder-bridge changes.

Recheck the COM number in Device Manager. Settings are **115200 baud, 8 data bits, no parity, one stop bit (8N1), no flow control**. Run:

```powershell
.\Read-Serial.ps1 -Port COM6 -DurationSeconds 30
```

The script displays available characters, then closes the port. Defaults are COM6, 115200 baud, and 30 seconds, so `.\Read-Serial.ps1` also works. `-BaudRate` is configurable but must match the firmware. Only one serial reader should open a given COM port at a time.

Expected current text is `Hello from STM32! Cycle: N`, approximately once every 10 seconds: one loop includes both 5-second delays. The counter increments once per loop using wrapping u32 arithmetic. After reset its initial value is zero and the first message uses one.

The owner observed recurring `Hello from STM32!` messages before the counter was added, including three messages in 30 seconds, and tested the saved script with a 20-second duration. Counter changes were subsequently observed in the debugger; counter text has not been separately confirmed in the serial reader.

## 7. Debug Rust source in VS Code

In Extensions (Ctrl+Shift+X), install **Debugger for probe-rs**, publisher **probe-rs**, ID `probe-rs.probe-rs-debugger`. An Uninstall button confirms installation. The supplied launch configuration follows the [official probe-rs debugger guide](https://probe.rs/docs/tools/debugger/).

1. Save source changes and run `cargo build` in the project terminal.
2. In `src/main.rs`, place a breakpoint beside the cycle_count increment inside the loop. Remove breakpoints on the delays.
3. Open Run and Debug (Ctrl+Shift+D), select **Debug STM32**, and click the green start triangle. F5 or Fn+F5 is an alternative. This **flashes the built ELF** and requests a halt after reset; a separate CLI flash is unnecessary.
4. If paused in startup, use **Continue** on the active debug toolbar. Wait for the counter breakpoint; later loop iterations are about 10 seconds apart.
5. Inspect cycle_count in Variables/Locals or Watch. Use **Step Over** on its increment and observe the value increase. The owner needed three clicks to advance to the next source line and updated value; one source statement need not correspond to one debugger step.
6. Use Continue through the long delays. Paused application execution also stops progress of LED updates and messages.
7. End the session with the red Stop square before starting another session or flashing from the terminal.

The new firmware remains in flash after debugging; ending a session does not restore the previous program. If it remains halted, power cycling starts the flashed application again. The owner demonstrated a source breakpoint, stepping, and variable inspection; debugger logs confirmed completed flash writes.

## Troubleshooting

| Symptom | Check or action |
| --- | --- |
| No power, COM port, or probe | Check CN1, data cable, driver, power jumpers, and Device Manager. Recheck COM numbers after reconnection. |
| Probe listed but target reads fail | Check both CN4 caps and explicit chip name STM32F439ZI before changing hardware. |
| `probe-rs info` cannot identify the chip | In version 0.32.0 automatic identification failed; `info --chip` ignored that option. The explicit-chip `read` in step 2 worked. |
| USB probe open error 5 during debugging | End the existing debug session before starting one new session. This resolved the observed incident; contention was suspected, not proven. |
| Grey breakpoint / no valid breakpoint information | Try another executable statement, preferably the counter increment. The GPIO set_high line was rejected but the delay and counter locations worked. No optimization changes were needed. |
| Step Over reports unsupported long-running operations | A 5-second delay exceeded the observed stepping timeout and caused a forced halt. Use Continue across waits and Step Over on quick arithmetic. |
| Serial messages arrive in an initial burst | ReadExisting can display a backlog. The script discards bytes already waiting on the PC when opened, but upstream data can still arrive. Check that later live messages arrive about every 10 seconds. |
| COM6 access denied or unavailable | Close another serial reader and confirm the current port number in Device Manager. |
| No serial output | Allow at least one complete 10-second cycle, confirm 115200 8N1 and the current COM port, and ensure the debugger has not paused execution. |
| PowerShell blocks the script | Read the exact execution-policy error and resolve it according to your computer's policy; no policy change was required or tested here. |

## Tested evidence and limitations

Recorded project environment: **Windows 11 version 25H2**.

| Tool or dependency | Recorded version |
| --- | --- |
| rustc / Cargo | 1.99.1 / 1.99.1 |
| rustup | 1.29.2 |
| probe-rs | 0.32.0 |
| cortex-m-rt | 0.7.7 |
| stm32f4xx-hal, feature stm32f439 | 0.23.0 |

The owner supplied observations and command output; this documentation review did not repeat hardware operations. CLI readback verification output was not supplied, although `--verify` was requested. Debugger logs establish completed writes, not a separately reported readback check. LED timing was not independently measured. Backup restoration, a separate reset-button test, and reproduction on a second computer are untested. Exact VS Code, extension, and PowerShell versions are unrecorded. The journal preserves the detailed evidence and earlier learning stages.
