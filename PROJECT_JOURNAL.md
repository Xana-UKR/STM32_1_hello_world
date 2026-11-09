# Project journal

Working record of decisions, commands, observations, failures, and next steps. Stable requirements and the learning workflow live in [PROJECT_SPEC.md](PROJECT_SPEC.md). Tested, repeatable newcomer instructions are distilled into [README.md](README.md) at milestones and polished at the end.

## Current state

**Last updated:** 2026-11-08

The owner reported successful toolchain setup and communication with the STM32. Earlier setup dates were not recorded.

- The board was connected through CN1 after the owner checked the jumper settings listed in the README. Windows detected the ST-LINK virtual COM port, then assigned COM6.
- Rust, the `thumbv7em-none-eabihf` target, and probe-rs were installed. ST-LINK detection and a processor identification register read succeeded; the exact commands and output are in the README.
- `probe-rs info` failed to identify the chip automatically; an explicit-chip memory read succeeded. The README records this workaround.
- A 2 MiB main-flash copy was saved and its size, first bytes, and SHA-256 were checked. The owner confirmed steady LED startup and subsequently blinking that resumes after USB disconnect/reconnect. Recurring serial output on COM6 and Rust source debugging (breakpoint, stepping, value inspection) are confirmed.
- The owner built and ran successive LED, serial, and counter firmware versions. Debugger logs confirm completed flash writes; earlier CLI flash output was not supplied, so a separate readback verification result is unconfirmed.
- The final technical/editorial README review is complete. It now describes reproduction using the supplied final project files; earlier creation exercises remain in this journal. Stable requirements and the guided learning workflow are unchanged.

## Next steps

1. Optional owner readability review: identify any README explanation that is unclear from a newcomer's perspective. The technical/editorial review is complete and requires no further firmware changes.
2. For stronger reproduction evidence, optionally record exact VS Code, probe-rs extension, and PowerShell versions and repeat the guide on a second computer. These were not part of this review's hardware checks.
3. Keep the main-flash copy safe. Retain documented limits: no restoration test or separately reported CLI readback result; final counter text and power-cycle startup were inferred from source and earlier hardware observations, not separately rechecked in chat.

## Decisions and progress

### 2026-11-08 — Final newcomer README review completed

- Owner asked who should perform the review. Assistant undertook the technical/editorial review, leaving owner feedback focused on clarity; only documentation was changed.
- Compared requirements, journal evidence, and actual Cargo/source/memory/build/debugger/serial-reader files. Reorganized README into seven repeatable steps for the supplied project: tools, board connection, precautionary flash copy, build, programming, serial reading, and source debugging.
- Removed obsolete desktop-run, steady-LED, and half-second instructions from the reproduction path; their learning history remains here. Current README consistently describes two 5-second delays and a roughly 10-second message cycle, with counter text identified as expected from current source.
- Added a complete file-role table, explicit project-root/hidden-folder requirements, backup overwrite caution and generic unused filename, rebuild-before-debug instruction, and consolidated troubleshooting. Kept firmware source listings out of the guide and preserved tested versions and unverified items.
- Static checks passed: README local links resolve, Markdown code fences balance, and the configured debug ELF exists. Inspected actual source/configuration consistency. No build, flash, driver installation, serial-port access, or debugger operation was performed during this documentation review.
- README is 183 lines. Specification and AGENTS were unchanged because the requirements and teaching workflow did not change. Owner readability feedback remains optional.

### 2026-11-08 — Source debugging demonstrated in VS Code

- Owner clarified the editor is Visual Studio Code and installed Debugger for probe-rs (probe-rs.probe-rs-debugger); the Uninstall button confirmed installation. Exact editor/extension versions are unrecorded.
- Guided creation of `.vscode/launch.json`: probe-rs-debug launch request, STM32F439ZI, existing ELF path, flashingEnabled/restoreUnwrittenBytes/haltAfterReset enabled, coreIndex 0. Explained that debug launch flashes the built ELF but does not rebuild, and firmware remains in flash after debugging.
- Supplied debug logs showed FLASHING completed write. A later attempt failed to open the USB probe (error 5) while a prior debug session remained open. Owner stopped the existing session and launched once; the USB error did not recur. Competing sessions are a plausible explanation, not conclusively proven.
- Breakpoint on `led.set_high()` was rejected with no valid breakpoint information. Moving it to the next delay line worked. Source and ELF timestamps and dev build settings were checked; no optimization change was needed. Exact cause of the line mapping issue remains unproven.
- Step Over across a 5-second delay produced "Long running operations between debug steps are not currently supported" and forced-halt messages. Changed the exercise to quick counter arithmetic and Continue across waits.
- Owner added a u32 cycle_count initialized to zero, wrapping increment each loop, and the value in serial formatting; reported a successful build and working debug session. Owner watched cycle_count change each cycle and reported that three Step Over clicks advanced past the increment and exposed its new value. This demonstrates breakpoint, stepping, and value inspection; one source statement did not correspond to one debugger step.
- Source inspection confirms the final counter and two 5-second delays. No assistant hardware commands or firmware edits were performed. Next: final newcomer documentation review.

### 2026-11-08 — Saved serial reader tested

- Owner ran `.\Read-Serial.ps1 -Port COM6 -DurationSeconds 20` and observed an initial burst of messages followed by one new message every 10 seconds. Explained that ReadExisting displays available buffered text; the exact buffering layer was not established.
- Suggested DiscardInBuffer immediately after Open to discard bytes already queued on the PC. Current file inspection confirms the owner added this line. It does not guarantee clearing upstream ST-LINK/USB data.
- Owner confirmed everything works as expected. Serial reader operation and the live 10-second message interval are now verified by the owner. Next: source breakpoint, stepping, and value inspection; exact editor remains to be clarified.

### 2026-11-08 — Serial output confirmed; reader saved

- Owner rechecked ST-LINK's port as COM6. Through guided edits, configured USART3 TX on PD8 at 115200 baud, imported core::fmt::Write, and added a recurring `Hello from STM32!` message with CRLF. PD9 RX is unused. No hardware routing changes were made.
- Owner changed both LED delays to `5.secs()` and reported a successful build and blinking after the next flashing instruction. Source inspection confirms 5 seconds HIGH plus 5 seconds LOW: approximately 10 seconds per complete blink/message cycle. Flash console output was not supplied.
- Owner ran the provided 30-second PowerShell reader on COM6 at 115200 baud, 8 data bits, no parity, one stop bit, no flow control, and reported three `Hello from STM32!` messages. This confirms the end-to-end firmware/USART3/ST-LINK/Windows serial path.
- At the owner's request, saved the reader as root `Read-Serial.ps1`, with optional Port, BaudRate, and DurationSeconds parameters (defaults COM6, 115200, 30). Retained try/finally port disposal, added input ranges and terminating error handling. PowerShell parser check passed; saved script not executed against hardware by the assistant. Next: owner runs the saved script, then begin source debugging.

### 2026-11-08 — Blinking and its power-cycle startup confirmed

- Owner added a TIM5 blocking delay using `peripherals.TIM5.delay_us(&mut rcc)` and a loop driving PB7 HIGH for 500 ms then LOW for 500 ms. The HAL example was checked before providing the edit.
- Owner reported `cargo build` success, was given the same probe-rs download command with --verify, --restore-unwritten, and --reset, and reported that the LED blinks and resumes after disconnect/reconnect. The approximate timing was not independently measured; flash console output was not supplied.
- Explained why the HAL example's `use panic_halt as _;` includes a library-provided panic handler. Retained our explicit handler; no panic-halt dependency was added.
- Next: serial output. ST UM1974 section 6.9 / table 9 documents USART3 TX on PD8 and RX on PD9; default ST-LINK routing uses SB5/SB6 ON and SB7/SB4 OFF. Actual routing remains to be established by serial communication; no hardware changes are planned.

### 2026-11-08 — Power-cycle startup confirmed

- After the blue LED observation, the owner disconnected and reconnected USB through CN1 without another build or programming command and reported the blue LED lit again. This confirms startup of the steady LED firmware after a power cycle; a separate reset-button test was not performed.
- Next exercise: timed blinking using the HAL's TIM5 blocking delay API, checked against the stm32f4xx-hal 0.23.0 delay-timer-blinky example. The blinking change has not yet been made or tested.

### 2026-11-08 — First blue LED observation

- Added through owner edits: exact dependency stm32f4xx-hal 0.23.0 with feature stm32f439; main takes peripheral handles, constrains RCC, splits GPIOB, configures PB7 as push-pull output, sets it HIGH, and loops. Owner requested short inline code comments; provided them in chat.
- Confirmed LD2 is blue, connected to PB7, and active HIGH in ST UM1974 section 7.5: https://www.st.com/resource/en/user_manual/dm00244518-stm32-nucleo-144-boards-mb1137-stmicroelectronics.pdf . This describes the documented board routing; actual board solder bridges were not separately inspected.
- Owner reported successful `cargo build` for the LED firmware; exact output and HAL transitive dependency versions were not supplied.
- Before the first flash, explained replacement of existing target firmware and the saved backup's untested restoration. Checked installed `probe-rs download --help`; instructed `probe-rs download --chip STM32F439ZI --verify --restore-unwritten --reset .\target\thumbv7em-none-eabihf\debug\stm32_1_hello_world`. Explained sector preservation, verification, and reset options. No whole-chip erase was requested.
- Following the flashing instruction, owner reported the blue LED lit. This matches the intended firmware behavior; no flash console output was supplied, so verification completion is not independently confirmed. Power-cycle startup remains to be tested.
- Owner noticed a virtual disk on USB connection. Explained ST-LINK's mass-storage, debug, and virtual COM interfaces; no disk file operations were requested.

### 2026-11-08 — First complete firmware build passed

- The owner created `memory.x` with flash at 0x08000000 (2048 KiB) and RAM at 0x20000000 (192 KiB), following the ST datasheet memory map (DS9484 Rev 14, p. 86). The separate 64 KiB CCM region is intentionally unused.
- The owner added `build.rs` to copy `memory.x` into Cargo's OUT_DIR, emit a linker search path, and track memory-file changes. Explained that this build script runs on Windows using std.
- The owner added `.cargo/config.toml` selecting `thumbv7em-none-eabihf` by default and passing `-Tlink.x` to the linker. The runtime's linker script consumes `memory.x`.
- Owner output from `cargo build` showed successful completion in the dev profile in 10.43 s, with no warnings shown. Expected ELF path: `target/thumbv7em-none-eabihf/debug/stm32_1_hello_world`; ELF contents have not been separately inspected. No flash or board execution occurred.
- Next: introduce the chip HAL, confirm LED routing, and add observable LED behavior before the first flash. Official library docs list stm32f4xx-hal 0.23.0 with a stm32f439 feature; this dependency is not yet added or tested.

### 2026-11-08 — Bare-metal Rust check passed

- The owner reported the expected failure when building the original desktop source for `thumbv7em-none-eabihf`; exact diagnostics were not supplied.
- Through explained edits, the owner added `no_std` and `no_main`, the exact dependency `cortex-m-rt = "=0.7.7"`, an `#[entry]` function with an infinite loop, and a custom `#[panic_handler]` that loops. Discussed references, inferred lifetime notation, and the never type.
- Owner output from `cargo check --target thumbv7em-none-eabihf` showed success in the dev profile. Locked dependencies: cortex-m-rt 0.7.7, cortex-m-rt-macros 0.7.7, proc-macro2 1.0.107, quote 1.0.47, syn 3.0.9, unicode-ident 1.0.27.
- This checks the embedded Rust code but does not produce a linked firmware image or establish board execution. No flash occurred. Next: explain `memory.x` and linker setup.

### 2026-11-08 — Cargo skeleton and desktop build verified

- The owner reported successful `cargo init --bin --name stm32_1_hello_world --vcs none .` and confirmed the generated manifest and source matched the examples discussed in chat. Explained package metadata, edition 2024, `fn main`, and the `println!` macro. The owner reads and edits files in their editor (described as Visual Studio).
- Owner output from `cargo run` showed successful compilation in the dev profile (unoptimized, with debug information), execution of `target\debug\stm32_1_hello_world.exe`, and `Hello, world!`. Running `.\target\debug\stm32_1_hello_world.exe` directly printed the same message.
- This verifies the Windows build and execution path only. No STM32 firmware was built or flashed. Next: explain the transition to the installed embedded target and the lack of an operating system.

### 2026-11-08 — Documentation initialized and reviewed

- Assigned requirements to the specification, working instructions to AGENTS, colleague setup to the README, and handover state to this journal.
- Removed duplicated scope and setup details, shortened explanations, and corrected stale notes about the missing README and unrecorded Windows version. The README records Windows 11 version 25H2.
- Preserved earlier reported results; this review did not install tools, run hardware checks, or flash the board.

### 2026-11-08 — Main-flash copy saved

- The owner prefers a chat-guided sequence: explain one command or file change, have the owner run it, and discuss the result before moving on. Documentation updates follow completed steps.
- The board was previously used for custom work, but the owner has no original firmware file and does not expect its contents to be important. We made a precautionary main-flash copy before any flash.
- `probe-rs read --chip STM32F439ZI b32 0x08000000 2` returned `20030000 080030d9`, consistent with an application vector table. The owner saved 2 MiB to `stm32f439zi-before-first-flash-2026-11-08.bin`; size was 2,097,152 bytes and first eight file bytes matched the board read. SHA-256 was `D1935C2544CB369F50FB06B92BF0029DCC0E9CE9442652A66E4890F79E9D28C2`, independently checked in the workspace.
- No board flash or erase occurred. Other memory was not copied, and restoration was not tested. Next: teach the Rust build path and create the project incrementally with the owner.

### 2026-11-08 — Documentation roles and workflow agreed

- The owner made teaching through close, step-by-step chat collaboration the primary requirement. A working program and README are secondary visible results. Updated the specification and agent instructions to require explanations before each command or file change and discussion of the owner's results before the next step.
- The journal is the working evidence record. The README is the clean newcomer guide: PowerShell commands and short explanations, without firmware source listings. Update it at completed milestones and polish it at the end. No board action was taken for this documentation change.

