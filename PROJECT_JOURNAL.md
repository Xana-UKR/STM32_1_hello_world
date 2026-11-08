# Project journal

Working record of decisions, commands, observations, failures, and next steps. Stable requirements and the learning workflow live in [PROJECT_SPEC.md](PROJECT_SPEC.md). Tested, repeatable newcomer instructions are distilled into [README.md](README.md) at milestones and polished at the end.

## Current state

**Last updated:** 2026-11-08

The owner reported successful toolchain setup and communication with the STM32. Earlier setup dates were not recorded.

- The board was connected through CN1 after the owner checked the jumper settings listed in the README. Windows detected the ST-LINK virtual COM port, then assigned COM6.
- Rust, the `thumbv7em-none-eabihf` target, and probe-rs were installed. ST-LINK detection and a processor identification register read succeeded; the exact commands and output are in the README.
- `probe-rs info` failed to identify the chip automatically; an explicit-chip memory read succeeded. The README records this workaround.
- A 2 MiB main-flash copy was saved and its size, first bytes, and SHA-256 were checked. Firmware creation, build, flash, LED behavior, serial output, source debugging, and restart behavior remain unverified.

## Next steps

1. Continue the owner's chat-guided learning process: explain the Rust/Cargo build path, select an STM32F4 library, verify the onboard LED mapping, and create each project file with the owner before building.
2. Explain that flashing replaces target firmware, then test LED behavior with the owner. Keep the main-flash copy safe.
3. Verify UART pins and ST-LINK routing in UM1974. Choose a serial terminal, add recurring output, recheck the COM port, and test the settings.
4. Choose and configure debugger integration; demonstrate a breakpoint, stepping, and value inspection. Test reset or power-cycle behavior.
5. Record working observations here; add tested repeatable steps to the README at milestones and complete its newcomer review at the end.

## Decisions and progress

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

