# Project specification: Rust on NUCLEO-F439ZI

## Goal and audience

The primary goal is for the owner to learn how to work with the NUCLEO-F439ZI using Rust: understand the board, memory, build process, programming, serial communication, and debugging through hands-on steps. A working firmware application and a reproducible README are visible results of that learning, not substitutes for it. The owner is an experienced C++ developer new to Rust, microcontrollers, and development boards. The original time budget is approximately three days; favor small, understandable exercises.

## Collaboration and learning workflow

- Work together in chat, one command or file change at a time. Before an action, explain its purpose, the relevant Rust or board concept, what it reads or changes, the expected result, and how to interpret a different result. Connect new ideas to C++ where useful.
- The owner performs commands and observes the board, then shares results. Discuss the result and questions before choosing the next action. Do not silently create a firmware project or batch several unexplained steps. The owner may explicitly ask the assistant to perform a particular action instead.
- Introduce each project file and its role before creating it. Keep explanations deep enough to support learning but divided into manageable steps.
- At the end of each stage, update the relevant Markdown files: record commands, results, decisions, and open questions in the journal; add tested, repeatable steps to the README; and update this specification if requirements or the workflow changed. Keep the chat focused on learning during the stage, then polish the full README at the end of the project.

## Target and constraints

- Board: ST NUCLEO-F439ZI with onboard ST-LINK programmer/debugger.
- Computer: Windows 11.
- Connection: one USB data cable to the ST-LINK connector, CN1, using documented default jumper settings.
- Rust target: `thumbv7em-none-eabihf`.
- Programming and debugging: `probe-rs`; the installed version recognizes chip name `STM32F439ZI`.
- Output: an onboard LED and recurring serial messages through the ST-LINK virtual COM port. Confirm LED pins, UART pins, and board routing before implementation.
- Use onboard peripherals; require no external circuits, soldering, or additional equipment. Avoid unnecessary jumper, power, or signal changes.
- Keep the core project small. Add an RTOS, networking, or complex peripherals only if requested or necessary.

## Acceptance criteria

Complete and document each item on the actual board:

- The owner can explain what each major tool and project file contributes to the path from Rust source to running firmware, and what the observed board results establish.
- Build the Rust firmware for `thumbv7em-none-eabihf` using recorded commands and dependency versions.
- Flash it onto the board and demonstrate code-controlled LED behavior.
- Display a recurring firmware message in a Windows serial terminal; record the terminal settings and how to identify the port.
- Reach a Rust source breakpoint, step through code, and inspect a value.
- Confirm that the firmware restarts after a reset or power cycle.
- Provide a colleague-facing [README](README.md) covering prerequisites, connections, jumpers, installation, building, flashing, serial output, debugging, expected results, and useful troubleshooting. Include PowerShell commands with short explanations, but do not paste the firmware source into the README. Identify any untested instructions or limitations.

Record evidence and remaining work in [PROJECT_JOURNAL.md](PROJECT_JOURNAL.md); agent workflow rules are in [AGENTS.md](AGENTS.md).

## High-level stages

1. Install tools and establish PC-to-board communication; inspect and make a precautionary copy of existing main flash.
2. Learn Rust/Cargo and the STM32 build path, then create and build a minimal project file by file.
3. Flash, observe an onboard LED, and test restart behavior.
4. Add serial output through the ST-LINK virtual COM port.
5. Debug at a Rust source breakpoint, step, and inspect a value.
6. Finish and review the newcomer README using the journal's tested evidence.

## References

- [ST NUCLEO-F439ZI product page](https://www.st.com/en/evaluation-tools/nucleo-f439zi.html)
- [ST Nucleo-144 board manual UM1974](https://www.st.com/resource/en/user_manual/dm00244518-stm32-nucleo144-boardsstmicroelectronics.pdf)
- [probe-rs documentation](https://probe.rs/docs/)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
