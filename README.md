# ruboxer

Rudimentary container tool for Linux

## About

`ruboxer` stands for both "**ru**dimentary **boxer**" and "**Ru**st **boxer**". It was conceived as a simple yet educational systems programming project to explore the following areas:

- Rust as a systems programming language, a type and memory-safe alternative to C
- System calls and features exported by the Linux kernel used in modern container runtimes such as Docker and Kubernetes

## Installing

### From source

Ensure Rust and Cargo are installed.

1. Clone the repository to your local environment: `$ git clone https://github.com/DonaldKellett/ruboxer.git`
1. Change directory to the local copy of your repo: `$ cd /path/to/your/ruboxer`
1. Build the project using Cargo: `$ cargo build --release`

The compiled binary `ruboxer` should be located at `/path/to/your/ruboxer/target/release`. You may wish to copy or move it to some location included in your PATH so you may invoke it directly from the command line as `ruboxer`.

## Linux features employed by `ruboxer`

- chroot
- Process namespaces
- Memory cgroups

## Linux features yet to be employed

- Capabilities
- Network namespaces
- Seccomp profiles
- Mandatory access control (MAC)
- ...

## Credits

Inspired by Eric Chiang's [Containers from Scratch](https://ericchiang.github.io/post/containers-from-scratch/)

## License

[MIT](./LICENSE)
