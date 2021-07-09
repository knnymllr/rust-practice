# Rust Application With Cargo

I followed along with the [Rust Programming Tutorials](https://www.youtube.com/watch?v=vOMJlQ5B-M0&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL) playlist by [dcode](https://www.youtube.com/c/dcode-software/) and was inspired to write each step in one program.

When I first began to code my primary struggle was reading documentation.  Between the lack of line-breaks and overload of unfamiliar jargon I found myself on YouTube and Twitch searching for a deeper understanding of the tools I was learning to use.  Though I retain concepts best with hands-on learning and visual examples, I also understand the importance of reading documentation and turning those concepts into practice.  

My intention is to create interactive documentation within the program that facilitates familiarity with Rust and wrestling with documentation.  This is a work in progress and I welcome any suggestions to make this more interactive.  

## Install Rust

### Windows

- **If running Windows Subsystem for Linux** follow Linux instructions below
- Download rustup-init.exe from [installation](https://www.rust-lang.org/install) page and follow instructions
    - 64-bit is recommended but not required

### Mac | Linux | Unix-based

- Open terminal of choice and run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    - No 32-bit support
- **If running Windows Subsystem for Linux** run `sudo apt install build-essential` to fix `linker 'cc' not found` error
    -  If the error persists, refer to this [post](https://stackoverflow.com/questions/52445961/how-do-i-fix-the-rust-error-linker-cc-not-found-for-debian-on-windows-10) for  commands for CentOS, ArchLinux, and Solus

## Build From Scratch

The first two tutorials in Dom's playlist will show you how to create your own [Rust application](https://www.youtube.com/watch?v=vOMJlQ5B-M0&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=1) or [Cargo package](https://www.youtube.com/watch?v=_RfxLg6K9oE&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=2) on the command line. This interactive documentation begins with the third tutorial in the series, but these short videos will show you how to create your own project in the future.

## `target` Directory

The `target` directory is created by the Rust extension in Visual Studio Code, but it is not required to compile the application.  